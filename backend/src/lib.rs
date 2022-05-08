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
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder as RocketResponder;
use rocket::response::Response as RocketResponse;
use rocket::serde::json::Json;
use serde::Serialize;
use std::fmt;

use crate::db::errors::{ERR_ALREADY_EXISTS, ERR_NOT_FOUND};

#[derive(Serialize)]
pub struct Error {
    // Кастомное сообщение об ошибке
    pub details: String,

    // HTTP статус
    #[serde(skip_serializing)]
    pub status: Status,
}

impl Error {
    pub fn new(err: String, s: Status) -> Self {
        Error {
            details: err,
            status: s,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.details.as_str())
    }
}
 
impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => Error::new(ERR_NOT_FOUND.to_string(), Status::NotFound),

            DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                Error::new(ERR_ALREADY_EXISTS.to_string(), Status::UnprocessableEntity)
            }

            DieselError::DatabaseError(_, err) => {
                Error::new(err.message().to_string(), Status::InternalServerError)
            }

            _ => Error::new(err.to_string(), Status::InternalServerError),
        }
    }
}

impl<'a> RocketResponder<'a, 'static> for Error {
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
