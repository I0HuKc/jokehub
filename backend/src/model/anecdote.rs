use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::model::shrimp::{default_tags, validate_lang, Paws};
use shrimplib::Paws;

#[derive(Clone, Serialize, Deserialize, Paws)]
pub struct Anecdote {
    pub category: String,
    pub text: String,
}

#[derive(Clone, Deserialize, Validate, Debug)]
pub struct NewAnecdote {
    #[validate(length(min = 10, max = 280, message = "Lenght is invalid"))]
    pub text: String,

    #[serde(default = "default_tags")]
    pub tags: Vec<String>,

    #[validate(
        length(equal = 2, message = "Invalid length"),
        custom(function = "validate_lang", message = "Unknown type")
    )]
    pub language: String,
}

impl From<NewAnecdote> for Anecdote {
    fn from(na: NewAnecdote) -> Self {
        Anecdote {
            text: na.text.to_string(),
            category: String::from("anecdote"),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use validator::Validate;

    #[test_case(
        "Как?",
        false ;
        "invalid_text_short"
    )]
    #[test_case(
        "Как каннибал называет Пашу?",
        true ;
        "valid"
    )]
    fn new_anecdote_validation(text: &str, is_valid: bool) {
        let na = super::NewAnecdote {
            text: text.to_string(),
            tags: vec![],
            language: "ru".to_string(),
        };

        match na.validate() {
            Ok(_) => {
                if is_valid {
                    assert!(true)
                } else {
                    assert!(false)
                }
            }
            Err(_) => {
                if !is_valid {
                    assert!(true)
                } else {
                    assert!(false)
                }
            }
        }
    }
}
