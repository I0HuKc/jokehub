use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::schema::jokes_tb;

use crate::model::SUPPORTED_LANGUAGES;

#[derive(Clone, Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Joke {
    pub uuid: Uuid,
    pub category: String,
    pub language: String,
    pub setup: String,
    pub punchline: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Validate, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "jokes_tb"]
pub struct NewJoke {
    pub category: String,
    #[validate(
        length(equal = 2, message = "Invalid length"),
        custom(function = "validate_language", message = "Unknown type")
    )]
    pub language: String,
    pub setup: String,
    pub punchline: Option<String>,
}

fn validate_language(lang: &str) -> Result<(), ValidationError> {
    for l in SUPPORTED_LANGUAGES.clone() {
        if lang == l {
            return Ok(());
        }
    }

    return Err(ValidationError::new("custom"));
}
