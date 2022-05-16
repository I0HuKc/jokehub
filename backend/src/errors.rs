use lazy_static::lazy_static;
use serde::Serialize;

use mongodb::error::Error as MongoDbError;
use validator::ValidationErrors;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder as RocketResponder;
use rocket::response::Response as RocketResponse;
use rocket::serde::json::Json;

lazy_static! {
    pub static ref ERR_AUTH: &'static str = "Authorization is required to access the resource.";
}

pub enum ErrorKind<'a> {
    Internal(&'a str, Option<Vec<String>>),  
    Unprocessable(&'a str, Option<Vec<String>>),
    NotFound(&'a str, Option<Vec<String>>),

    Unauthorized,
}

#[derive(Clone, Serialize)]
pub struct HubError {
    error: String,
    details: Vec<String>,

    #[serde(skip_serializing)]
    status: Status,
}

impl<'a> HubError {
    fn create(err: &'a str, details: Option<Vec<String>>, status: Status) -> HubError {
        match details {
            Some(d) => HubError {
                error: err.to_string(),
                details: d,
                status,
            },

            None => HubError {
                error: err.to_string(),
                details: Vec::new(),
                status,
            },
        }
    }

    pub fn new(kind: ErrorKind) -> HubError {
        match kind {
            ErrorKind::Internal(err, d) => HubError::create(err, d, Status::InternalServerError),   
            ErrorKind::NotFound(err, d) => HubError::create(err, d, Status::NotFound),
            ErrorKind::Unprocessable(err, d) => HubError::create(err, d, Status::UnprocessableEntity),
            ErrorKind::Unauthorized => HubError::create(ERR_AUTH.clone(), None, Status::Unauthorized),                
        }
    }

    pub fn new_not_found(err: &str, d: Option<Vec<String>>) -> HubError {
        HubError::new(ErrorKind::NotFound(err, d))
    }

    pub fn new_internal(err: &str, d: Option<Vec<String>>) -> HubError {
        HubError::new(ErrorKind::Internal(err, d))
    }

    pub fn new_unprocessable(err: &str, d: Option<Vec<String>>) -> HubError {
        HubError::new(ErrorKind::Unprocessable(err, d))
    }

    // Добавить новый элемент в список деталей
    pub fn add(&mut self, d: String) -> HubError {
        self.details.push(d);
        return self.clone();
    }
}

impl<'a> RocketResponder<'a, 'static> for HubError {
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

impl From<ValidationErrors> for HubError {
    fn from(errs: ValidationErrors) -> Self {
        let mut error = HubError::new(ErrorKind::Unprocessable("Validation faild", Some(Vec::new())));

        for (k, v) in errs.field_errors() {
            for err in v {
                let mut e = serde_json::to_string(&err.message).unwrap();
                e = serde_json::from_str(e.as_str()).unwrap();
                error.add(format!("Field {}: {}", k, e));
            }
        }

        return error;
    }
}

impl From<MongoDbError> for HubError {
    fn from(err: MongoDbError) -> Self {
        match *err.kind {
            mongodb::error::ErrorKind::Write(err) => match err {
                mongodb::error::WriteFailure::WriteError(_) => {
                    HubError::new(ErrorKind::Unprocessable(ERR_ALREADY_EXISTS.clone(), None))
                }
                _ => HubError::new(ErrorKind::Internal(format!("{:?}", err).as_str(), None)),
            },

            _ => HubError::new(ErrorKind::Internal(
                format!("{:?}", err.kind).as_str(),
                None,
            )),
        }
    }
}

impl From<bson::ser::Error> for HubError {
    fn from(err: bson::ser::Error) -> Self {
        match err {
            bson::ser::Error::SerializationError { message, .. } => {
                HubError::new(ErrorKind::Internal(message.as_str(), None))
            }

            _ => HubError::new(ErrorKind::Internal(format!("{:?}", err).as_str(), None)),
        }
    }
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
