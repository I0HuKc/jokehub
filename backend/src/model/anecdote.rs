use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Anecdote {
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewAnecdote {
    pub tags: Vec<String>,
    pub language: String,
    pub text: String,
}

impl Anecdote {
    pub fn new(tags: Vec<String>, text: String) -> Self {
        Anecdote { tags, text }
    }
}
