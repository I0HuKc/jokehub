use chrono::NaiveDateTime;
use chrono::Utc;

use bson::serde_helpers::uuid_as_binary;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Head {
    pub counter: usize,
    pub timestamp: NaiveDateTime,
}

impl Head {
    pub fn new() -> Self {
        Head {
            counter: 0,
            timestamp: Utc::now().naive_utc(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Flags {
    pub religious: bool,
    pub political: bool,
    pub racist: bool,
    pub sexist: bool,
}

impl Flags {
    pub fn new(religious: bool, political: bool, racist: bool, sexist: bool) -> Self {
        Flags {
            religious,
            political,
            racist,
            sexist,
        }
    }

    pub fn default() -> Self {
        Flags {
            religious: false,
            political: false,
            racist: false,
            sexist: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tail {
    pub flags: Flags,

    pub author: String,

    #[serde(rename = "language")]
    pub lang: String,
}

impl Tail {
    pub fn new(flags: Flags, lang: String, author: String) -> Self {
        Tail {
            flags,
            lang,
            author,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Shrimp<B> {
    #[serde(with = "uuid_as_binary")]
    #[serde(rename = "_id")]
    pub id: Uuid,

    #[serde(rename = "_header")]
    pub head: Head,

    #[serde(flatten)]
    pub body: B,

    #[serde(rename = "_meta-data")]
    pub tail: Tail,
}

impl<B> Shrimp<B> {
    pub fn new(body: B, tail: Tail) -> Self {
        Shrimp {
            id: Uuid::new_v4(),
            head: Head::new(),
            body,
            tail,
        }
    }
}
