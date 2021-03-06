use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::model::shrimp::{default_tags, Category, Paws};
use shrimplib::Paws;

#[derive(Clone, Serialize, Deserialize, Paws)]
pub struct Joke {
    pub category: Category,
    pub text: String,
}

#[derive(Clone, Deserialize, Validate, Debug)]
pub struct NewJoke {
    #[validate(length(min = 10, max = 280, message = "Lenght is invalid"))]
    pub text: String,

    #[serde(default = "default_tags")]
    pub tags: Vec<String>,
}

impl From<NewJoke> for Joke {
    fn from(nj: NewJoke) -> Self {
        Joke {
            category: Category::Joke,
            text: nj.text,
        }
    }
}
