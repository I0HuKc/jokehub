use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Joke {
    pub uuid: String,
    pub category: String,
    pub language: String,
    pub setup: String,
    pub punchline: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewJoke<'r> {
    pub category: &'r str,
    pub language: &'r str,
    pub setup: &'r str,
    pub punchline: Option<&'r str>,
}