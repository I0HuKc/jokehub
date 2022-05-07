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

use rocket::serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Error {
    pub details: String,
}

impl Error {
    pub const fn new(err: String) -> Self {
        Error { details: err }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.details.as_str())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::new(format!("Database error: {}", err))
    }
}

pub enum Outcome<T> {
    Ok(T),
    // NotValid,
    AlreadyExists(Error),
    // NotFound,
    Other(Error),
}
