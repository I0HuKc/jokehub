use once_cell::sync::OnceCell;
use rand::{distributions::Alphanumeric, Rng};
use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};
use serde_json::Value;
use std::sync::Mutex;

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
pub fn try_login(client: &Client) -> Result<Tokens, Value> {
    let username: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let password = "12344321e".to_string();

    match login(client, username.as_str(), password.as_str()) {
        // Если удалось авторизоваться возвращаю токены
        Ok(tokens) => Ok(tokens),

        Err((status, value)) => {
            // Если ошибка не связана с тем, что пользователь не зареган
            if status != Status::NotFound {
                Err(value)
            } else {
                // Регистрирую пользователя
                registration(client, username.as_str(), password.as_str())?;

                // Пытаюсь авторизоваться еще раз
                match login(client, username.as_str(), password.as_str()) {
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
