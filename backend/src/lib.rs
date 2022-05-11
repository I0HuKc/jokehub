pub mod db;
pub mod model;
pub mod schema;
pub mod server;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

use diesel::result::Error as DieselError;
use mongodb::error::Error as MongoDbError;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder as RocketResponder;
use rocket::response::Response as RocketResponse;
use rocket::serde::json::Json;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Error as UuidError;
use validator::ValidationErrors;

use crate::db::{ERR_ALREADY_EXISTS, ERR_NOT_FOUND};

#[derive(Clone, Serialize)]
pub struct Errors<'a> {
    pub success: bool,

    #[serde(rename(serialize = "errors"))]
    pub details: HashMap<&'a str, Vec<Value>>,

    // HTTP статус
    #[serde(skip_serializing)]
    pub status: Status,
}

impl<'a> Errors<'a> {
    pub fn new(s: Status) -> Self {
        Errors {
            success: false,
            details: HashMap::new(),
            status: s,
        }
    }

    pub fn internal_from_error<E>(ch: &'a str, err: E) -> Self
    where
        E: ToString,
    {
        let err_str = serde_json::to_string(&err.to_string()).unwrap();
        let object: Value = serde_json::from_str(&err_str).unwrap();
        Errors::new(Status::InternalServerError).add(ch, json!(object))
    }

    pub fn add(&mut self, ch: &'a str, err: Value) -> Self {
        self.details
            .entry(ch)
            .or_insert_with(|| Vec::new())
            .push(err);

        return self.clone();
    }
}

impl From<UuidError> for Errors<'_> {
    fn from(err: UuidError) -> Self {
        let object: Value = serde_json::from_str(&err.to_string()).unwrap();
        Errors::new(Status::UnprocessableEntity).add("mdb", json!(object))
    }
}

impl From<MongoDbError> for Errors<'_> {
    fn from(err: MongoDbError) -> Self {
        Errors::new(Status::InternalServerError).add("mdb", json!(format!("{:?}", err.kind)))
    }
}

impl From<bson::ser::Error> for Errors<'_> {
    fn from(err: bson::ser::Error) -> Self {
        Errors::new(Status::UnprocessableEntity).add("bson", json!(err.to_string()))
    }
}

impl From<DieselError> for Errors<'_> {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => {
                Errors::new(Status::NotFound).add("db", json!(ERR_NOT_FOUND.clone()))
            }

            DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                Errors::new(Status::UnprocessableEntity)
                    .add("db", json!(ERR_ALREADY_EXISTS.clone()))
            }

            _ => Errors::new(Status::InternalServerError).add("db", json!(err.to_string())),
        }
    }
}

impl<'a> RocketResponder<'a, 'static> for Errors<'_> {
    fn respond_to(self, req: &'a rocket::Request<'_>) -> rocket::response::Result<'static> {
        match Json(&self).respond_to(req) {
            Ok(resp) => RocketResponse::build_from(resp)
                .status(self.status)
                .header(ContentType::JSON)
                .ok(),
            Err(s) => RocketResponse::build()
                .status(s)
                .header(ContentType::JSON)
                .ok(),
        }
    }
}

impl From<ValidationErrors> for Errors<'_> {
    fn from(errs: ValidationErrors) -> Self {
        let mut r_errs = Errors::new(Status::UnprocessableEntity);

        for (k, v) in errs.field_errors() {
            for err in v {
                let err_str = serde_json::to_string(err).unwrap();
                let object: Value = serde_json::from_str(err_str.as_str()).unwrap();

                r_errs.add(k, object);
            }
        }

        return r_errs;
    }
}
