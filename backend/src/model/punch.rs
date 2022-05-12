use serde::{Deserialize, Serialize};

use crate::model::shrimp::Paws;

#[derive(Clone, Serialize, Deserialize)]
pub struct Punch {
    pub category: String,

    pub setup: String,
    pub punchline: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct NewPunch {
    pub setup: String,
    pub punchline: String,

    pub tags: Vec<String>,
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