use crate::Error;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::serde::Serialize;

use crate::db::errors::{ERR_ALREADY_EXISTS, ERR_NOT_FOUND};

pub fn db_answer_handle<T: Serialize>(r: Result<T, DieselError>) -> (Result<T, Error>, Status) {
    match r {
        Ok(v) => (Ok(v), Status::Ok),
        Err(err) => match err {
            DieselError::NotFound => (Err(Error::new(ERR_NOT_FOUND.to_string())), Status::NotFound),
            DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => (
                Err(Error::new(ERR_ALREADY_EXISTS.to_string())),
                Status::UnprocessableEntity,
            ),
            _ => (Err(Error::from(err)), Status::InternalServerError),
        },
    }
}
