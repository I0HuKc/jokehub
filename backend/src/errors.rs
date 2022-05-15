use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env::VarError;

use serde::Serialize;
use serde_json::{json, Value};

use mongodb::error::Error as MongoDbError;
use validator::ValidationErrors;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder as RocketResponder;
use rocket::response::Response as RocketResponse;
use rocket::serde::json::Json;

// Раздел ошибки
#[derive(Clone, Serialize, Hash, PartialEq, Eq, Debug)]
pub struct ErrorChapter<'a>(pub &'a str);

// Ошибка
// Каждая ошибка принадлежит определенному разделу
#[derive(Clone, Serialize, Debug)]
pub struct Error<'a>(ErrorChapter<'a>, Value);

impl<'a> Error<'a> {
    pub fn new(ch: &'a str, body: Value) -> Self {
        Error(ErrorChapter(ch), body)
    }
}

// Общая структура ошибок которая будет отдаваться на вход
// в Rocket Responder и далее направлена клиенту HTTP запроса
#[derive(Clone, Serialize)]
pub struct Errors<'a> {
    #[serde(rename(serialize = "errors"))]
    details: HashMap<ErrorChapter<'a>, Vec<Value>>,

    // HTTP статус
    #[serde(skip_serializing)]
    status: Status,
}

#[derive(Clone, Serialize)]
pub enum ErrorsKind<'a> {
    // Ошибка со статусом 500
    //
    // Стоит использовать когда произошла внутренняя ошибка
    // и есть прямое взаимодействие с ее первоисточником.
    Internal(Error<'a>),

    // Ошибка со статусом 500
    //
    // Стоит использовать когда произошла внутренняя ошибка
    // и при этом нет взаимодействия с первоисточником, и пользователю
    // должно быть отправленно базовое сообщение о произошедшем сбое.
    InternalBase(ErrorChapter<'a>),

    // Ошибка со статусом 429
    //
    // Ошибка при работе с ресурсом по вине пользователя. Стоит
    // использовать при валидационых ошибках, проверках и тд.
    Unprocessable(Error<'a>),

    // Ошибка со статусом 404
    //
    // Очевидно, запрашиваемый ресурс не был найден.
    NotFound(ErrorChapter<'a>),
}

impl<'a> Errors<'a> {
    fn create(status: Status) -> Self {
        Errors {
            details: HashMap::new(),
            status,
        }
    }

    pub fn add(&mut self, err: Error<'a>) -> Self {
        self.details
            .entry(err.0)
            .or_insert_with(|| Vec::new())
            .push(err.1);

        return self.clone();
    }

    pub fn new(kind: ErrorsKind<'a>) -> Self {
        match kind {
            ErrorsKind::Internal(err) => Errors::create(Status::InternalServerError).add(err),

            ErrorsKind::InternalBase(ch) => {
                let err = Error::new(ch.0, json!(ERR_INTERNAL_BASE.clone()));
                Errors::create(Status::InternalServerError).add(err)
            }

            ErrorsKind::Unprocessable(err) => Errors::create(Status::UnprocessableEntity).add(err),

            ErrorsKind::NotFound(ch) => {
                let err = Error::new(ch.0, json!(ERR_NOT_FOUND.clone()));
                Errors::create(Status::NotFound).add(err)
            }
        }
    }
}

impl<'a> RocketResponder<'a, 'static> for Errors<'_> {
    fn respond_to(self, req: &'a rocket::Request<'_>) -> rocket::response::Result<'static> {
        match Json(&self).respond_to(req) {
            Ok(resp) => RocketResponse::build_from(resp)
                .status(self.status)
                .header(ContentType::JSON)
                .ok(),
            Err(s) => RocketResponse::build()
                .status(s)
                .header(ContentType::JSON)
                .ok(),
        }
    }
}

impl<'a> From<VarError> for Errors<'a> {
    fn from(err: VarError) -> Self {
        match err {
            VarError::NotPresent => {
                let err = Error::new(CH_SYSTEM.clone(), json!(ERR_ENV_VAR_NOT_FOUND.clone()));
                Errors::new(ErrorsKind::Internal(err))
            }

            VarError::NotUnicode(_) => {
                let err = Error::new(CH_SYSTEM.clone(), json!(ERR_ENV_VAR_INVALID.clone()));
                Errors::new(ErrorsKind::Internal(err))
            }
        }
    }
}

impl From<MongoDbError> for Errors<'_> {
    fn from(err: MongoDbError) -> Self {
        match *err.kind {
            mongodb::error::ErrorKind::Write(err) => match err {
                mongodb::error::WriteFailure::WriteError(_) => {
                    let err = Error::new(CH_DATABASE.clone(), json!(ERR_ALREADY_EXISTS.clone()));
                    Errors::new(ErrorsKind::Unprocessable(err))
                }
                _ => {
                    let err = Error::new(CH_DATABASE.clone(), json!(ERR_INTERNAL_BASE.clone()));
                    Errors::new(ErrorsKind::Internal(err))
                }
            },

            _ => {
                let err = Error::new(CH_DATABASE.clone(), json!(format!("{:?}", err.kind)));
                Errors::new(ErrorsKind::Internal(err))
            }
        }
    }
}

impl From<bson::ser::Error> for Errors<'_> {
    fn from(err: bson::ser::Error) -> Self {
        match err {
            bson::ser::Error::SerializationError { message, .. } => {
                let err = Error::new(CH_SYSTEM.clone(), json!(message));
                Errors::new(ErrorsKind::Internal(err))
            }
            _ => {
                let err = Error::new(CH_DATABASE.clone(), json!(format!("{:?}", err)));
                Errors::new(ErrorsKind::Internal(err))
            }
        }
    }
}

impl From<ValidationErrors> for Errors<'_> {
    fn from(v_errs: ValidationErrors) -> Self {
        let mut errs = Errors::create(Status::UnprocessableEntity);

        for (k, v) in v_errs.field_errors() {
            for err in v {
                errs.add(Error::new(k, json!(err.message)));
            }
        }

        return errs;
    }
}


// Базовые разделы ошибок
lazy_static! {
    pub static ref CH_SYSTEM: &'static str = "system";
    pub static ref CH_DATABASE: &'static str = "database";
    pub static ref CH_SERVER: &'static str = "server";
}

// Базовые ошибки
lazy_static! {
    static ref ERR_INTERNAL_BASE: &'static str = "Looks like something went wrong.";
}

// Базовые ошибки БД
lazy_static! {
    pub static ref ERR_ALREADY_EXISTS: &'static str = "Record with these parameters already exists";
    pub static ref ERR_NOT_FOUND: &'static str = "Resource was not found";
}

// Ошибки env раздела
lazy_static! {
    static ref ERR_ENV_VAR_NOT_FOUND: &'static str = "The specified environment variable was not present in the current process's environment.";
    static ref ERR_ENV_VAR_INVALID: &'static str = "The specified environment variable was found, but it did not contain valid unicode data. The found data is returned as a payload of this variant.";
}
