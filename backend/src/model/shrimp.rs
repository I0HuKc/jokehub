use chrono::{NaiveDateTime, Utc};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use validator::ValidationError;

use crate::errors::HubError;

use super::account::Tariff;

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

    // Переключатели

    pub fn religious_coup(&mut self) -> Self {
        self.religious = !self.religious;
        return self.clone();
    }

    pub fn political_coup(&mut self) -> Self {
        self.political = !self.political;
        return self.clone();
    }

    pub fn racist_coup(&mut self) -> Self {
        self.racist = !self.racist;
        return self.clone();
    }

    pub fn sexist_coup(&mut self) -> Self {
        self.sexist = !self.sexist;
        return self.clone();
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tail {
    pub flags: Flags,

    pub author: String,

    pub tags: Vec<String>,

    #[serde(rename = "language")]
    pub lang: String,
}

pub(crate) fn default_tags() -> Vec<String> {
    return vec![String::from("general")];
}

pub(crate) fn validate_lang(lang: &str) -> Result<(), ValidationError> {
    for l in super::SUPPORTED_LANGUAGES.clone() {
        if lang == l {
            return Ok(());
        }
    }

    return Err(ValidationError::new("custom"));
}

impl Tail {
    pub fn new(flags: Flags, lang: &String, author: String, tags: &Vec<String>) -> Self {
        Tail {
            flags,
            lang: lang.to_string(),
            author,
            tags: tags.to_vec(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Shrimp<B>
where
    B: Serialize,
    B: Paws,
{
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "_header")]
    pub head: Head,

    #[serde(flatten)]
    pub body: B,

    #[serde(rename = "_meta-data")]
    pub tail: Tail,
}

pub trait Paws {}

impl<B> Shrimp<B>
where
    B: Serialize,
    B: Paws,
{
    pub fn new(body: B, tail: Tail) -> Self {
        Shrimp {
            id: Uuid::new_v4().to_string(),
            head: Head::new(),
            body,
            tail,
        }
    }

    pub fn tariffing(&self, tariff: Tariff, err: Option<HubError>) -> Value {
        match tariff {
            Tariff::Free => {
                let mut base = json!(self.body);
                Self::err_union(&mut base, err)
            }
            Tariff::Basic => {
                let mut base = json!({"id": self.id});

                Self::merge(&mut base, json!(self.body));
                Self::merge(&mut base, json!({"_meta-data": self.tail}));

                Self::err_union(&mut base, err)
            }
            Tariff::Standart => {
                let mut base = json!(self);
                Self::err_union(&mut base, err)
            }
            Tariff::Enterprice => {
                let mut base = json!(self);
                Self::err_union(&mut base, err)
            }
        }
    }

    fn merge(a: &mut Value, b: Value) {
        match (a, b) {
            (a @ &mut Value::Object(_), Value::Object(b)) => {
                let a = a.as_object_mut().unwrap();
                for (k, v) in b {
                    Self::merge(a.entry(k).or_insert(Value::Null), v);
                }
            }
            (a, b) => *a = b,
        }
    }

    fn err_union(base: &mut Value, err: Option<HubError>) -> Value {
        if let Some(e) = err {
            Self::merge(base, json!({ "errors": e }));

            base.clone()
        } else {
            base.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(
        "fr",
        false ;
        "unsupported_lang"
    )]
    #[test_case(
        "russian",
        false ;
        "invalid_lang_format"
    )]
    #[test_case(
        "ru",
        true ;
        "valid_ru"
    )]
    #[test_case(
        "en",
        true ;
        "valid_en"
    )]
    fn test_validate_lang(lang: &str, is_valid: bool) {
        match super::validate_lang(lang) {
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
