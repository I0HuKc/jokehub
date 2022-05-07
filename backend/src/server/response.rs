use rocket::http::ContentType;
use rocket::serde::json::Json;

use rocket::serde::Serialize;
use rocket::{http::Status, response};
use serde_json::json;
use serde_json::Value;
use std::marker::PhantomData;

pub struct Response<'a> {
    pub body: Value,
    pub status: Status,
    pub _marker: PhantomData<&'a ()>,
}

impl Response<'_> {
    pub fn new<T: Serialize>(v: T, s: Status) -> Self {
        Response {
            body: json!(v),
            status: s,
            _marker: PhantomData,
        }
    }
}

impl<'a> response::Responder<'a, 'static> for Response<'a> {
    fn respond_to(self, req: &'a rocket::Request<'_>) -> rocket::response::Result<'static> {
        match Json(self.body).respond_to(req) {
            Ok(resp) => response::Response::build_from(resp)
                .status(self.status)
                .header(ContentType::JSON)
                .ok(),
            Err(s) => response::Response::build()
                .status(s)
                .header(ContentType::JSON)
                .ok(),
        }
    }
}
