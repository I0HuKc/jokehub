pub mod account;
pub mod anecdote;
pub mod joke;
pub mod punch;
pub mod shrimp;

use lazy_static::lazy_static;
use regex::Regex;
use uuid::Uuid;

use crate::errors::{Error, Errors, ErrorsKind};
use serde_json::json;
use validator::ValidationError;

lazy_static! {
    pub(crate) static ref SUPPORTED_LANGUAGES: Vec<&'static str> = ["ru", "en"].to_vec();
}

pub fn uuid_validation<'a>(id: &str) -> Result<(), Errors<'a>> {
    match Uuid::parse_str(id) {
        Ok(_) => Ok(()),
        Err(_) => {
            let error = Error::new("uuid", json!("invalid format"));
            Err(Errors::new(ErrorsKind::Unprocessable(error)))
        }
    }
}

pub(crate) fn validate_query<'a>(username: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[^._ ](?:[\w-]|\.[\w-])+[^._ ]$").unwrap();
    if re.is_match(username) {
        Ok(())
    } else {
        return Err(ValidationError::new("username"));
    }
}
