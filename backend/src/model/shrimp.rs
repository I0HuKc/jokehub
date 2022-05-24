use lingua::Language;
use mongodb::bson::DateTime as MongoDateTime;
use rand::prelude::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt;
use uuid::Uuid;

use super::account::Tariff;
use crate::errors::HubError;

/// Заголовок любой записи контента
#[derive(Clone, Serialize, Deserialize)]
pub struct Head {
    pub counter: usize,
    pub rfd: u32,
    pub timestamp: MongoDateTime,
}

impl Head {
    pub fn new() -> Self {
        Head {
            counter: 0,
            rfd: rand::thread_rng().gen::<u32>(),
            timestamp: MongoDateTime::now(),
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
    pub fn tariffing(&self, tariff: Tariff, err: Option<HubError>) -> Value {
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
    fn err_union(mut base: Value, err: Option<HubError>) -> Value {
        if err.is_some() {
            Self::merge(&mut base, json!({ "errors": err.unwrap() }));

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
    pub fn random(mut list: Option<Vec<Category>>) -> Category {
        use super::shrimp::Category::{Anecdote, Joke, Punch};

        list.get_or_insert(vec![Anecdote, Joke, Punch])
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_owned()
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub mod filter {
    //! Модуль фильтрации контента
    use bson::Document;
    use mongodb::bson::doc;
    use rand::Rng;

    macro_rules! macro_filter {
        ($co:literal, $ri:expr, $( ($k:literal, $v:expr)), *) => {
            {
                let mut filter = doc! {"_header.rfd": {$co: $ri}};
                $(
                    if $v.is_some() {
                        filter.insert($k, $v);
                    }
                )*

                filter
            }
        };
    }

    pub struct Filter<'a> {
        // Случайное число от которого будет осуществляться поиск
        ri: u32,
        author: Option<&'a str>,
        language: Option<&'a str>,
        flags: Option<Vec<super::Flag>>,
    }

    impl<'a> Filter<'a> {
        pub fn new(
            author: Option<&'a str>,
            language: Option<&'a str>,
            flags: Option<Vec<super::Flag>>,
        ) -> Self {
            Filter {
                ri: rand::thread_rng().gen::<u32>(),
                author,
                language,
                flags,
            }
        }

        /// Генерация набора фильтров
        /// Первый документ сгенерирован для $gt поиска
        /// Второй для $lte поиска
        pub fn gen(&self) -> (Document, Document) {
            let mut gt = macro_filter!(
                "$gt",
                self.ri,
                ("_meta-data.language", self.language),
                ("_meta-data.author", self.author)
            );
            gt = self.add_flags(Box::new(gt));

            let mut lte = macro_filter!(
                "$lte",
                self.ri,
                ("_meta-data.language", self.language),
                ("_meta-data.author", self.author)
            );
            lte = self.add_flags(Box::new(lte));

            (gt, lte)
        }

        /// Метод добавления флагов (если они указаны явно) в общий объект фильтрации
        fn add_flags(&'a self, mut d: Box<Document>) -> Document {
            if self.flags.is_some() {
                for f in self.flags.as_ref().unwrap() {
                    d.insert(
                        format!("_meta-data.flags.{}", f.to_string().to_lowercase()),
                        true,
                    );
                }

                *d
            } else {
                *d
            }
        }
    }
}
