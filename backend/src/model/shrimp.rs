use chrono::NaiveDateTime;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize)]
pub struct Head {
    #[serde(rename(serialize = "_id"))]
    pub uuid: Uuid,
    pub counter: usize,
    pub author: String,
    pub timestamp: NaiveDateTime,
}

impl Head {
    pub fn new(header_slim: HeadSlim) -> Self {
        Head {
            uuid: Uuid::new_v4(),
            counter: 0,
            author: header_slim.author,
            timestamp: Utc::now().naive_utc(),
        }
    }
}

pub struct HeadSlim {
    pub author: String,
}

impl HeadSlim {
    pub fn new(author: String) -> Self {
        HeadSlim { author }
    }
}

#[derive(Clone, Serialize)]
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

#[derive(Clone, Serialize)]
pub struct Tail {
    flags: Flags,

    #[serde(rename(serialize = "language"))]
    pub lang: String,
}

impl Tail {
    pub fn new(flags: Flags, lang: String) -> Self {
        Tail { flags, lang }
    }
}

#[derive(Clone, Serialize)]
pub struct Shrimp<B> {
    #[serde(rename(serialize = "_header"))]
    pub head: Head,

    pub body: B,

    #[serde(rename(serialize = "_meta-data"))]
    pub tail: Tail,
}

impl<B> Shrimp<B> {
    pub fn new(header_slim: HeadSlim, body: B, tail: Tail) -> Self {
        Shrimp {
            head: Head::new(header_slim),
            body,
            tail,
        }
    }
}
