use lingua::Language;
use mongodb::bson::DateTime as MongoDateTime;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt;
use uuid::Uuid;

use super::account::Tariff;
use crate::db::mongo::varys::Varys;
use crate::errors::HubError;

/// Заголовок любой записи контента
#[derive(Clone, Serialize, Deserialize)]
pub struct Head {
    pub counter: usize,
    pub timestamp: i64,
}

impl Head {
    pub fn new() -> Self {
        Head {
            counter: 0,
            timestamp: MongoDateTime::now().timestamp_millis(),
        }
    }
}

#[derive(Clone, PartialEq, FromFormField, Debug)]
pub enum Flag {
    Nsfw,
    Religious,
    Political,
    Racist,
    Sexist,
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Флаги деликатности контента
#[derive(Clone, Serialize, Deserialize)]
pub struct Flags {
    pub nsfw: bool,
    pub religious: bool,
    pub political: bool,
    pub racist: bool,
    pub sexist: bool,
}

impl Flags {
    pub fn new(nsfw: bool, religious: bool, political: bool, racist: bool, sexist: bool) -> Self {
        Flags {
            nsfw,
            religious,
            political,
            racist,
            sexist,
        }
    }

    pub fn default() -> Self {
        Flags {
            nsfw: false,
            religious: false,
            political: false,
            racist: false,
            sexist: false,
        }
    }

    // Переключатели деликатности контента

    pub fn nsfw_coup(&mut self) -> Self {
        self.nsfw = !self.nsfw;
        return self.clone();
    }

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

impl Tail {
    pub fn new(flags: Flags, lang: Language, author: String, tags: &Vec<String>) -> Self {
        Tail {
            flags,
            lang: lang.to_string(),
            author,
            tags: tags.to_vec(),
        }
    }
}

/// Общее тело для любого текстового контента.
/// В зависимости от пользовательского тарифа в ответе доступны те или иные поля.
/// Тариф определяется исходя из токена в заголовке запроса.
/// Если токен отсутствует сериализация контента согласно тарифу Free.
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

pub trait Paws {
    fn get_category(&self) -> Category;
}

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

    /// Сериализация контента согласно пользовательскому тарифу
    pub fn tariffing(&self, tariff: &Tariff, err: &Option<HubError>) -> Value {
        match tariff {
            Tariff::Free => {
                let base = json!(self.body);
                Self::err_union(base, err)
            }
            Tariff::Basic => {
                let mut base = json!({"id": self.id});

                Self::merge(&mut base, json!(self.body));
                Self::merge(&mut base, json!({"_meta-data": self.tail}));

                Self::err_union(base, err)
            }
            Tariff::Standart => {
                let base = json!(self);
                Self::err_union(base, err)
            }
            Tariff::Enterprice => {
                let base = json!(self);
                Self::err_union(base, err)
            }
        }
    }

    /// Быстрое слияние json объектов
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

    /// Слияние базового json объекта и объекта ошибок (если они возникли)
    fn err_union(mut base: Value, err: &Option<HubError>) -> Value {
        if err.is_some() {
            Self::merge(&mut base, json!({ "errors": err.as_ref().unwrap() }));

            base
        } else {
            base
        }
    }
}

#[derive(Clone, Serialize, PartialEq, Deserialize, FromFormField, Debug)]
pub enum Category {
    #[serde(rename = "anecdote")]
    Anecdote,

    #[serde(rename = "joke")]
    Joke,

    #[serde(rename = "punch")]
    Punch,
}

impl Category {
    /// Выбор случайной категории.
    /// Если есть предпочитаемые категории, выбирается случайная из предоставленных.
    /// Если список предпочтений пуст, выбирается из общего списка категоий.
    ///
    /// rf — (rewrite flag)
    /// Если значение TRUE тогда в случае пустого массива предпочитаемые категории он будет
    /// перезаписан общим списком доступных категорий.
    ///
    /// Если значение FALSE, в случае пустого массива он не перезаписывается.
    pub fn random(mut list: Option<Vec<Category>>, rf: bool) -> (Option<Category>, Vec<Category>) {
        use super::shrimp::Category::{Anecdote, Joke, Punch};

        if rf {
            let list = list.get_or_insert(vec![Anecdote, Joke, Punch]);
            let random_category = list.choose(&mut rand::thread_rng()).unwrap().to_owned();

            // Удаляю выбранную категорию из доступных
            let index = list.iter().position(|x| *x == random_category).unwrap();
            list.remove(index);

            (Some(random_category), list.to_vec())
        } else {
            if list.as_ref().unwrap().len() != 0 {
                let random_category = list
                    .as_ref()
                    .unwrap()
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_owned();

                // Удаляю выбранную категорию из доступных
                let index = list
                    .as_ref()
                    .unwrap()
                    .iter()
                    .position(|x| *x == random_category)
                    .unwrap();

                list.as_mut().unwrap().remove(index);

                (Some(random_category), list.unwrap().to_vec())
            } else {
                (None, Vec::new())
            }
        }
    }

    pub fn to_collection(&self) -> Varys {
        match self {
            Category::Anecdote => Varys::Anecdote,
            Category::Joke => Varys::Joke,
            Category::Punch => Varys::Punch,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
