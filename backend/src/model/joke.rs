use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::jokes_tb;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "jokes_tb"]
pub struct Joke {
    pub uuid: Uuid,
    pub category: String,
    pub language: String,
    pub setup: String,
    pub punchline: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Copy, Clone, Serialize, Deserialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "jokes_tb"]
pub struct NewJoke<'r> {
    pub category: &'r str,
    pub language: &'r str,
    pub setup: &'r str,
    pub punchline: Option<&'r str>,
}

impl From<NewJoke<'_>> for Joke {
    fn from(nj: NewJoke) -> Self {
        Joke {
            uuid: Uuid::new_v4(),
            category: nj.category.to_string(),
            language: nj.language.to_string(),
            setup: nj.setup.to_string(),
            punchline: Some(nj.punchline.unwrap_or_default().to_owned()),
            created_at: Utc::now().naive_utc(),
        }
    }
}
