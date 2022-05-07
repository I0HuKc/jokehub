use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::jokes_tb;

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

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "jokes_tb"]
pub struct NewJoke {
    pub category: String,
    pub language: String,
    pub setup: String,
    pub punchline: Option<String>,
}