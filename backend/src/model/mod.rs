pub mod account;
pub mod anecdote;
pub mod joke;
pub mod punch;
pub mod shrimp;

use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref SUPPORTED_LANGUAGES: Vec<&'static str> = ["ru", "en"].to_vec();
}

pub mod validation {
    use regex::Regex;
    use uuid::Uuid;
    use validator::ValidationError;

    use crate::errors::HubError;

    pub fn uuid_validation<'a>(id: &str) -> Result<&str, HubError> {
        match Uuid::parse_str(id) {
            Ok(_) => Ok(id),
            Err(_) => Err(HubError::new_unprocessable("Invalid format of uuid", None)),
        }
    }

    pub fn query_validation(username: &str) -> Result<&str, HubError> {
        let re = Regex::new(r"^[^._ ](?:[\w-]|\.[\w-])+[^._ ]$").unwrap();
        if re.is_match(username) {
            Ok(username)
        } else {
            Err(HubError::new_unprocessable("Invalid format of query", None))
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
}
