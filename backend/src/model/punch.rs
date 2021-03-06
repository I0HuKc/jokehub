use serde::{Deserialize, Serialize};

use shrimplib::Paws;

use crate::model::shrimp::{default_tags, Paws, Category};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Paws)]
pub struct Punch {
    pub category: Category,
    pub setup: String,
    pub punchline: String,
}

#[derive(Clone, Deserialize, Validate, Debug)]
pub struct NewPunch {
    #[validate(length(min = 15, max = 280, message = "Lenght is invalid"))]
    pub setup: String,

    #[validate(length(min = 2, max = 50, message = "Lenght is invalid"))]
    pub punchline: String,

    #[serde(default = "default_tags")]
    pub tags: Vec<String>,
}

impl From<NewPunch> for Punch {
    fn from(np: NewPunch) -> Self {
        Punch {
            setup: np.setup.to_string(),
            punchline: np.punchline.to_string(),
            category: Category::Punch,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use validator::Validate;

    #[test_case(
        "Some",
        "Паштет",      
        false ;
        "setup_lenght_short"
    )]
    #[test_case(
        "Как каннибал называет Пашу?",
        "1",        
        false ;
        "punchline_lenght_short"
    )]
    #[test_case(
        "Как каннибал называет Пашу?",
        "Паштет",       
        true ;
        "valid"
    )]
    fn new_puch_validation(setup: &str, punchline: &str, is_valid: bool) {
        let np = super::NewPunch {
            setup: setup.to_string(),
            punchline: punchline.to_string(),
            tags: vec![],
        };

        match np.validate() {
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
