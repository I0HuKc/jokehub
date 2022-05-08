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
pub struct Errors<'a> {
    // Кастомное сообщение об ошибке
    pub details: Vec<HashMap<&'a str, Vec<String>>>,

    // HTTP статус
    #[serde(skip_serializing)]
    pub status: Status,
}

impl<'a> Errors<'a> {
    pub fn new(s: Status) -> Self {
        Errors {
            details: Vec::new(),
            status: s,
        }
    }

    pub fn add(&mut self, ch: &'a str, err: String) -> Self {
        if self.details.len() > 1 {
            for d in self.details.clone() {
                match d.get(ch) {
                    Some(v) => {
                        println!("Test");
                        v.clone().push(err.to_string());
                        return self.clone();
                    }
                    None => {
                        println!("Test 2");
                        return self.clone();
                    }
                }

                // let b = d.get(ch).get_or_insert(&Vec::new());
                // b.push(err.to_string().clone())
            }
        }

        let mut hm: HashMap<&'a str, Vec<String>> = HashMap::new();
        let mut v: Vec<String> = Vec::new();

        println!("Test 5");

        v.push(err);
        hm.insert(ch, v);
        self.details.push(hm);

        return self.clone();
    }
}

impl From<DieselError> for Errors<'_> {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => {
                Errors::new(Status::NotFound).add("db", ERR_NOT_FOUND.to_string())
            }

            DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                Errors::new(Status::UnprocessableEntity).add("db", ERR_ALREADY_EXISTS.to_string())
            }

            _ => Errors::new(Status::InternalServerError).add("db", err.to_string()),
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
            r_errs.add("Dfgfhfh", "dgdgdg".to_string());
            println!("Tres 3");
        }

        return r_errs;
    }
}

// impl From<ValidationErrors> for Errors<'_> {
//     fn from(v_errs: ValidationErrors) -> Self {
//         let r_errs = Errors::new(Status::UnprocessableEntity);
//         let mut book_reviews = HashMap::new();

//         for (field_name, errs_kind) in v_errs.errors().to_owned() {
//             match errs_kind {
//                 validator::ValidationErrorsKind::Struct(_) => todo!(),
//                 validator::ValidationErrorsKind::List(_) => todo!(),
//                 validator::ValidationErrorsKind::Field(field_errs) => {

//                     for err in field_errs {
//                         match err.message {
//                             Some(cow) => match cow {
//                                 std::borrow::Cow::Borrowed(m) => {
//                                     println!("Fuck 1");
//                                     book_reviews.insert(field_name, m.to_string());
//                                     // r_errs.add(format!("Field {}. Error: {}", field_name, m.to_string()));
//                                 }
//                                 std::borrow::Cow::Owned(m) => {
//                                     println!("Fuck 2");
//                                     book_reviews.insert(field_name, m);
//                                     // r_errs.add(format!("Field {}. Error: {}", field_name, m.to_string()));
//                                 }
//                             },
//                             None => {
//                                 println!("{}", err);
//                                 book_reviews
//                                     .insert(field_name, "Undefined validation error".to_string());
//                                 // r_errs.add(format!("Field {}. Undefined validation error", field_name));
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         Errors::new(Status::UnprocessableEntity).add(book_reviews);
//         return r_errs;
//     }
// }
