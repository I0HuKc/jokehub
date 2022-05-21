use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde_json::Value;

use jokehub::model::account::security::Tokens;

use crate::json_string;

pub trait TestUser {
    fn get_username(&self) -> &str;
    fn get_password(&self) -> &str;
}

/// Тестовый пользователь с уровнем доступа Padawan
#[allow(dead_code)]
pub struct TestPadawan<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> TestPadawan<'a> {
    #[allow(dead_code)]
    pub fn new(username: &'a str, password: &'a str) -> Self {
        Self { username, password }
    }
}

impl<'a> Default for TestPadawan<'a> {
    fn default() -> Self {
        Self {
            username: "upadawan",
            password: "password2022",
        }
    }
}

impl<'a> TestUser for TestPadawan<'a> {
    fn get_username(&self) -> &str {
        return self.username;
    }

    fn get_password(&self) -> &str {
        return self.password;
    }
}

/// Тестовый пользователь с уровнем доступа Master
/// Пользователь создается автоматически при создании тестового окружения
#[allow(dead_code)]
pub struct TestMaster<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> Default for TestMaster<'a> {
    fn default() -> Self {
        Self {
            username: "tmaster",
            password: "12344321e",
        }
    }
}

impl<'a> TestUser for TestMaster<'a> {
    fn get_username(&self) -> &str {
        return self.username;
    }

    fn get_password(&self) -> &str {
        return self.password;
    }
}

/// Тестовый пользователь с уровнем доступа Sith
/// Пользователь создается автоматически при создании тестового окружения
#[allow(dead_code)]
pub struct TestSith<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> Default for TestSith<'a> {
    fn default() -> Self {
        Self {
            username: "tsith",
            password: "12344321e",
        }
    }
}

impl<'a> TestUser for TestSith<'a> {
    fn get_username(&self) -> &str {
        return self.username;
    }

    fn get_password(&self) -> &str {
        return self.password;
    }
}

/// Создать тестовый аккаунт
#[allow(dead_code)]
fn registration(client: &Client, username: &str, password: &str) -> Result<(), Value> {
    let resp = client
        .post("/v1/registration")
        .header(ContentType::JSON)
        .body(json_string!({
            "username": username,
            "password": password
        }))
        .dispatch();

    if resp.status() != Status::Ok {
        Err(super::response_json_value(resp))
    } else {
        Ok(())
    }
}

/// Авторизоваться
#[allow(dead_code)]
fn login(client: &Client, username: &str, password: &str) -> Result<Tokens, (Status, Value)> {
    let resp = client
        .post("/v1/login")
        .header(ContentType::JSON)
        .body(json_string!({
            "username": username,
            "password": password
        }))
        .dispatch();

    if resp.status() != Status::Ok {
        Err((resp.status(), super::response_json_value(resp)))
    } else {
        let value = super::response_json_value(resp).to_string();
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
