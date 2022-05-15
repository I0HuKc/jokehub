use serde::{Deserialize, Serialize};

use crate::model::shrimp::{default_tags, validate_lang, Paws};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize)]
pub struct Punch {
    pub category: String,
    pub setup: String,
    pub punchline: String,
}

#[derive(Clone, Deserialize, Validate, Debug)]
pub struct NewPunch {
    pub setup: String,
    pub punchline: String,

    #[serde(default = "default_tags")]
    pub tags: Vec<String>,

    #[validate(
        length(equal = 2, message = "Invalid length"),
        custom(function = "validate_lang", message = "Unknown type")
    )]
    pub language: String,
}

impl Punch {
    pub fn new(np: &NewPunch) -> Self {
        Punch {
            setup: np.setup.to_string(),
            punchline: np.punchline.to_string(),
            category: String::from("punch"),
        }
    }
}

impl Paws<Punch> for Punch {}
