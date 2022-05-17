use once_cell::sync::OnceCell;
use rocket::local::blocking::{Client, LocalResponse};
use serde_json::Value;
use std::sync::Mutex;

use jokehub::server;

#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

pub fn test_client() -> &'static Mutex<Client> {
    static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let server = server::rocket();
        Mutex::from(Client::tracked(server).expect("valid rocket instance"))
    })
}

pub fn response_json_value<'a>(response: LocalResponse<'a>) -> Value {
    let body = response.into_string().unwrap();
    serde_json::from_str(&body).expect("can't parse value")
}
