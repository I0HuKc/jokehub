use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::model::shrimp::{default_tags, Paws, Category};
use shrimplib::Paws;

#[derive(Clone, Serialize, Deserialize, Paws)]
pub struct Anecdote {
    pub category: Category,
    pub text: String,
}

#[derive(Clone, Deserialize, Validate, Debug)]
pub struct NewAnecdote {
    #[validate(length(min = 10, max = 1000, message = "Lenght is invalid"))]
    pub text: String,

    #[serde(default = "default_tags")]
    pub tags: Vec<String>,
}

impl From<NewAnecdote> for Anecdote {
    fn from(na: NewAnecdote) -> Self {
        Anecdote {
            text: na.text.to_string(),
            category: Category::Anecdote,
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
