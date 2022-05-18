pub mod accounts;
pub mod punch;

use once_cell::sync::OnceCell;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};
use serde_json::Value;
use std::sync::Mutex;

use accounts::TestUser;
use jokehub::{model::account::security::Tokens, server};

// Создать тестовый аккаунт
#[allow(dead_code)]
fn registration(client: &Client, username: &str, password: &str) -> Result<(), Value> {
    let resp = client
        .post("/v1/registration")
        .header(ContentType::JSON)
        .body(super::json_string!({
            "username": username,
            "password": password
        }))
        .dispatch();

    if resp.status() != Status::Ok {
        Err(response_json_value(resp))
    } else {
        Ok(())
    }
}

// Авторизоваться
#[allow(dead_code)]
fn login(client: &Client, username: &str, password: &str) -> Result<Tokens, (Status, Value)> {
    let resp = client
        .post("/v1/login")
        .header(ContentType::JSON)
        .body(super::json_string!({
            "username": username,
            "password": password
        }))
        .dispatch();

    if resp.status() != Status::Ok {
        Err((resp.status(), response_json_value(resp)))
    } else {
        let value = response_json_value(resp).to_string();
        let tokens: Tokens = serde_json::from_str(&value.as_str()).expect("login valid response");

        Ok(tokens)
    }
}

#[allow(dead_code)]
pub fn try_login(client: &Client, user: Box<dyn TestUser>) -> Result<Tokens, Value> {
    match login(client, user.get_username(), user.get_password()) {
        // Если удалось авторизоваться возвращаю токены
        Ok(tokens) => Ok(tokens),

        Err((status, value)) => {
            // Если ошибка не связана с тем, что пользователь не зареган
            if status != Status::NotFound {
                Err(value)
            } else {
                // Регистрирую пользователя
                registration(client, user.get_username(), user.get_password())?;

                // Пытаюсь авторизоваться еще раз
                match login(client, user.get_username(), user.get_password()) {
                    Ok(tokens) => Ok(tokens),
                    Err((_, v)) => Err(v),
                }
            }
        }
    }
}

#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

#[allow(dead_code)]
pub fn test_client() -> &'static Mutex<Client> {
    static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();

    INSTANCE.get_or_init(|| {
        let server = server::rocket();
        Mutex::from(Client::tracked(server).expect("invalid rocket instance"))
    })
}

#[allow(dead_code)]
pub fn response_json_value<'a>(response: LocalResponse<'a>) -> Value {
    let body = response.into_string().unwrap();
    serde_json::from_str(&body).expect("can't parse value")
}