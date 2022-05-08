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
use std::collections::HashMap;
use validator::ValidationErrors;

use crate::db::errors::{ERR_ALREADY_EXISTS, ERR_NOT_FOUND};

#[derive(Clone, Serialize)]
pub struct Errors {
    // Кастомное сообщение об ошибке
    pub details: Vec<String>,

    // HTTP статус
    #[serde(skip_serializing)]
    pub status: Status,
}

impl Errors {
    pub fn new(s: Status) -> Self {
        Errors {
            details: Vec::new(),
            status: s,
        }
    }

    pub fn add(&mut self, err: String) -> Self {
        self.details.push(err);
        return self.clone();
    }
}

impl From<DieselError> for Errors {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => Errors::new(Status::NotFound).add(ERR_NOT_FOUND.to_string()),

            DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                Errors::new(Status::UnprocessableEntity).add(ERR_ALREADY_EXISTS.to_string())
            }

            DieselError::DatabaseError(_, err) => {
                Errors::new(Status::InternalServerError).add(err.message().to_string())
            }

            _ => Errors::new(Status::InternalServerError).add(err.to_string()),
        }
    }
}

impl<'a> RocketResponder<'a, 'static> for Errors {
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


impl From<ValidationErrors> for Errors {
    fn from(v_errs: ValidationErrors) -> Self {
        let mut r_errs = Errors::new(Status::UnprocessableEntity);

        for (field_name, errs_kind) in v_errs.errors().to_owned() {
            match errs_kind {
                validator::ValidationErrorsKind::Struct(_) => todo!(),
                validator::ValidationErrorsKind::List(_) => todo!(),
                validator::ValidationErrorsKind::Field(field_errs) => {
                    for err in field_errs {
                        match err.message {
                            Some(cow) => match cow {
                                std::borrow::Cow::Borrowed(m) => {
                                    r_errs.add(format!("Field {}. Error: {}", field_name, m.to_string()));
                                    
                                }
                                std::borrow::Cow::Owned(m) => {
                                    r_errs.add(format!("Field {}. Error: {}", field_name, m.to_string()));
                                }
                            },
                            None => {
                                r_errs.add(format!("Field {}. Undefined validation error", field_name));
                            }

                        }
                    }
                }
            }
        }

        return r_errs;
    }
}
