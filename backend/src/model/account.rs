use argon2::Config;
use mongodb::bson::DateTime as MongoDateTime;
use rand::Rng;
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::EnumIter;
use uuid::Uuid;
use validator::Validate;

use super::{account::security::Session, validation::validate_query};
use crate::errors::HubError;

/// Уровни доступа доступные в системе
#[derive(Clone, Serialize, PartialEq, EnumIter, Deserialize, Debug)]
pub enum Level {
    #[serde(rename = "padawan")]
    Padawan,

    #[serde(rename = "master")]
    Master,

    #[serde(rename = "sith")]
    Sith,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Виды тарифов доступных в системе
#[derive(Clone, Serialize, PartialEq, Deserialize, Debug)]
pub enum Tariff {
    #[serde(rename = "free")]
    Free,

    #[serde(rename = "basic")]
    Basic,

    #[serde(rename = "standart")]
    Standart,

    #[serde(rename = "enterprice")]
    Enterprice,
}

impl Default for Tariff {
    fn default() -> Self {
        Tariff::Free
    }
}

#[derive(Clone, Serialize, PartialEq, Deserialize, Debug)]
pub enum Theme {
    #[serde(rename = "light")]
    Light,

    #[serde(rename = "dark")]
    Dark,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> FromParam<'a> for Theme {
    type Error = HubError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),

            _ => Err(HubError::new_unprocessable("Theme type is invalid", None)),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

/// Тело запроса при регистрации пользователя
#[derive(Clone, Validate, Deserialize)]
pub struct NewUser {
    #[validate(
        length(min = 4, max = 10, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    pub username: String,

    #[validate(
        length(min = 8, max = 20, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    pub password: String,
}

#[derive(Clone, Validate, Deserialize)]
pub struct ChangePassword {
    #[validate(
        length(min = 8, max = 20, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    pub old_password: String,

    #[validate(
        length(min = 8, max = 20, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    pub new_password: String,
}

/// Нативная структура пользовательских данных
#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,

    pub username: String,
    pub hash: String,

    pub level: Level,
    pub tariff: Tariff,

    pub theme: Theme,

    pub created_at: MongoDateTime,
    pub updated_at: MongoDateTime,
}

impl From<NewUser> for User {
    fn from(nu: NewUser) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            username: nu.username,
            level: Level::Padawan,
            tariff: Tariff::Free,
            hash: nu.password,
            theme: Theme::default(),
            created_at: MongoDateTime::now(),
            updated_at: MongoDateTime::now(),
        }
    }
}

impl<'a> User {
    // Верификация пароля
    pub fn password_verify(&self, password: &[u8]) -> Result<bool, HubError> {
        argon2::verify_encoded(&self.hash, password).map_err(|err| {
            HubError::new_internal("Failed to verify password", Some(Vec::new()))
                .add(format!("{}", err))
        })
    }

    // Создание хеша пароля
    pub fn password_hashing(&mut self) -> Result<User, HubError> {
        self.hash = Self::password_hashing_apart(&self.hash)?;
        Ok(self.clone())
    }

    pub fn password_hashing_apart(password: &String) -> Result<String, HubError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config).map_err(|err| {
            HubError::new_internal("Failed create password hash", Some(Vec::new()))
                .add(format!("{}", err))
        })?;

        Ok(hash)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub tariff: Tariff,
    pub theme: Theme,
    pub sessions: Vec<Sinfo>,
    pub created_at: String,
    pub  updated_at: String,
}

// Session info
#[derive(Serialize, Deserialize)]
pub struct Sinfo {
    stamp: String,
}

impl From<Session> for Sinfo {
    fn from(s: Session) -> Self {
        Sinfo {
            stamp: s.get_stamp(),
        }
    }
}

impl Sinfo {
    fn vec_convert(ov: Vec<Session>) -> Vec<Self> {
        let mut nv: Vec<Self> = Vec::new();
        let _: Vec<_> = ov.into_iter().map(|value| nv.push(value.into())).collect();

        return nv;
    }
}

impl Account {
    pub fn new(user: User, sessions: Vec<Session>) -> Self {
        Account {
            username: user.username,
            tariff: user.tariff,
            theme: user.theme,
            sessions: Sinfo::vec_convert(sessions),
            created_at: user.created_at.to_rfc3339_string(),
            updated_at: user.updated_at.to_rfc3339_string(),
        }
    }

    pub fn get_username(&self) -> &str {
        self.username.as_str()
    }

    pub fn get_tariff(&self) -> &Tariff {
        &self.tariff
    }

    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }
}

pub mod notification {
    use bson::oid::ObjectId;
    use mongodb::bson::DateTime as MongoDateTime;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub enum NotifyKind {
        #[serde(rename = "success")]
        Success,

        #[serde(rename = "attention")]
        Attention,

        #[serde(rename = "error")]
        Error,
    }

    #[derive(Serialize, Deserialize)]
    pub enum ActionKind {
        #[serde(rename = "major")]
        Major,

        #[serde(rename = "minor")]
        Minor,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Action {
        pub kind: ActionKind,
        pub text: String,
        pub href: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Notification {
        #[serde(rename = "_id")]
        pub id: ObjectId,
        pub from: String,
        pub to: String,
        pub kind: NotifyKind,

        #[serde(flatten)]
        pub body: Body,

        #[serde(rename = "_meta-data")]
        pub meta_data: MetaData,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Body {
        pub title: String,
        pub description: Option<String>,
        pub actions: Option<Vec<Action>>,
    }

    impl Body {
        pub fn new(
            title: String,
            description: Option<String>,
            actions: Option<Vec<Action>>,
        ) -> Self {
            Self {
                title,
                description,
                actions,
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct MetaData {
        pub read: bool,
        pub archived: bool,
        pub created_at: MongoDateTime,
    }

    impl Default for MetaData {
        fn default() -> Self {
            Self {
                read: false,
                archived: false,
                created_at: MongoDateTime::now(),
            }
        }
    }

    impl Notification {
        pub fn new(from: String, to: String, kind: NotifyKind, body: Body) -> Self {
            Self {
                id: bson::oid::ObjectId::new(),
                from,
                to,
                kind,
                body,
                meta_data: MetaData::default(),
            }
        }
    }
}

pub mod security {
    use std::borrow::Cow;

    use chrono::prelude::*;
    use jsonwebtoken::TokenData;
    use jsonwebtoken::{errors::ErrorKind as JwtErrorKind, DecodingKey, EncodingKey, Validation};
    use rocket::http::Status;
    use rocket::Route;
    use rocket::{
        request, request::FromRequest, request::Outcome, serde::DeserializeOwned, Request,
    };
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::{
        err_forbidden, err_unauthorized,
        errors::{ErrorKind, HubError, UnauthorizedErrorKind},
        model::account::{Level, Tariff},
    };
    use mongodb::bson::DateTime as MongoDateTime;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Session {
        token: String,
        username: String,
        stamp: MongoDateTime,
    }

    impl Session {
        pub fn new(username: &str, token: &str) -> Self {
            Session {
                token: token.to_string(),
                username: username.to_string(),
                stamp: MongoDateTime::now(),
            }
        }

        pub fn get_username(&self) -> String {
            self.username.clone()
        }

        pub fn get_stamp(&self) -> String {
            self.stamp.to_rfc3339_string()
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    struct Fingerprint {
        ip: String,
        location: String,
        browser: String,
        os: String,
    }

    const SECRET: &str = "secret297152aebda7";

    /// Полезная нагрузка токена доступа
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AccessClaims {
        access_uuid: Uuid,
        username: String,
        level: Level,
        tariff: Tariff,

        #[serde(with = "jwt_numeric_date")]
        exp: DateTime<Utc>,
    }

    impl AccessClaims {
        fn new(username: String, level: Level, tariff: Tariff) -> Self {
            // Задаю срок жизни access токена
            let exp = Utc::now() + chrono::Duration::minutes(15);

            // Нормализация к временным меткам UNIX
            let exp = exp
                .date()
                .and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);

            AccessClaims {
                access_uuid: Uuid::new_v4(),
                username,
                level,
                tariff,
                exp,
            }
        }

        pub fn get_username(&self) -> String {
            return self.username.clone();
        }

        pub fn get_level(&self) -> Level {
            return self.level.clone();
        }
    }

    /// Полезная нагрузка токена обновления
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RefreshClaims {
        refresh_uuid: Uuid,
        username: String,

        #[serde(with = "jwt_numeric_date")]
        exp: DateTime<Utc>,
    }

    impl RefreshClaims {
        fn new(ac: &AccessClaims) -> Self {
            // Задаю срок жизни refresh токена
            let exp = Utc::now() + chrono::Duration::days(7);

            // Нормализация к временным меткам UNIX
            let exp = exp
                .date()
                .and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);

            RefreshClaims {
                refresh_uuid: Uuid::new_v4(),
                username: ac.get_username(),
                exp,
            }
        }

        pub fn get_username(&self) -> String {
            return self.username.clone();
        }
    }

    mod jwt_numeric_date {
        //! Сериализация DateTime<Utc> для соответствия спецификации JWT (RFC 7519 раздел 2, "Numeric Date")
        use chrono::{DateTime, TimeZone, Utc};
        use serde::{self, Deserialize, Deserializer, Serializer};

        /// Сериализирует DateTime<Utc> в отметку времени Unix (миллисекунды с 1970/1/1T00:00:00T)
        pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let timestamp = date.timestamp();
            serializer.serialize_i64(timestamp)
        }

        /// Попытки десериализовать i64 и использовать в качестве временной метки Unix
        pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
        {
            Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
                .single() // Если есть несколько или нет действительных значений DateTimes из метки времени, возвращаю None
                .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
        }
    }

    /// Базовый охранник авторизации
    /// Требует наличие токена доступа в заголовке
    #[derive(Debug)]
    pub struct AuthGuard(pub AccessClaims);

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for AuthGuard {
        type Error = HubError;

        async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
            match request.headers().get_one("Authorization") {
                Some(at) if split_token(at).len() == 2 => {
                    let token = Tokens::decode_token::<AccessClaims>(split_token(at)[1]);

                    match token {
                        Ok(t) => Outcome::Success(AuthGuard(t.claims)),
                        Err(err) => Outcome::Failure((err.get_status(), err)),
                    }
                }

                Some(_) => Outcome::Failure((
                    Status::Unauthorized,
                    err_unauthorized!("Token is in invalid format"),
                )),

                None => {
                    let kind = ErrorKind::Unauthorized(UnauthorizedErrorKind::TokenMissing);
                    let err = HubError::new(kind);

                    Outcome::Failure((err.get_status(), err))
                }
            }
        }
    }

    /// Индивидуальный охранник, необходим когда авторизация необязательна.
    /// Если в заголовке нет токена доступа, тогда по умолчанию применяется тариф Free
    /// Если токен присутствует, используется тариф содержащийся в токене
    pub struct TariffGuard(pub Tariff, pub Option<HubError>);

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for TariffGuard {
        type Error = HubError;

        async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
            match request.headers().get_one("Authorization") {
                Some(at) if split_token(at).len() == 2 => {
                    let token = Tokens::decode_token::<AccessClaims>(split_token(at)[1]);

                    match token {
                        Ok(t) => Outcome::Success(TariffGuard(t.claims.tariff, None)),
                        Err(err) => Outcome::Success(TariffGuard(Tariff::default(), Some(err))),
                    }
                }

                Some(_) => Outcome::Success(TariffGuard(
                    Tariff::default(),
                    Some(err_unauthorized!("Token is in invalid format")),
                )),

                None => Outcome::Success(TariffGuard(Tariff::default(), None)),
            }
        }
    }

    /// Охранник ресурсов требующих доступ не ниже чем Master
    pub struct LevelGuard(pub AccessClaims);

    impl LevelGuard {
        /// Проверка маршрута на соответствие запрашиваемого пользователя и его уровня доступа
        fn access_check(route: Option<&Route>, user_level: &Level) -> bool {
            let route = route.unwrap().name.clone().unwrap();

            // Маршруты которые защищены уровнем Master
            let master_level: Vec<Cow<str>> = vec![
                Cow::Borrowed("delete_joke"),
                Cow::Borrowed("delete_punch"),
                Cow::Borrowed("delete_anecdote"),
            ];
            // Маршруты которые защищены уровнем Sith
            let sith_level: Vec<Cow<str>> =
                [vec![Cow::Borrowed("privilege")], master_level.clone()].concat(); // маршруты доступные Master доступны и Sith

            match user_level {
                Level::Padawan => false,
                Level::Master => master_level.contains(&route),
                Level::Sith => sith_level.contains(&route),
            }
        }
    }

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for LevelGuard {
        type Error = HubError;

        async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
            request
                .guard::<AuthGuard>()
                .await
                .and_then(|d| match d.0.get_level() {
                    level @ Level::Padawan if Self::access_check(request.route(), &level) => {
                        Outcome::Success(LevelGuard(d.0))
                    }

                    level @ Level::Master if Self::access_check(request.route(), &level) => {
                        Outcome::Success(LevelGuard(d.0))
                    }

                    level @ Level::Sith if Self::access_check(request.route(), &level) => {
                        Outcome::Success(LevelGuard(d.0))
                    }

                    _ => Outcome::Failure((Status::Forbidden, err_forbidden!())),
                })
        }
    }

    /// Набор токенов для аутентификации
    /// Время жизн и токена доступа — 15 мин
    /// Время жизни токена обновления — 7д
    #[derive(Clone, Serialize, Deserialize)]
    pub struct Tokens {
        pub access_token: String,
        pub refresh_token: String,
    }

    impl<'a> Tokens {
        pub fn new(username: &str, level: Level, tariff: Tariff) -> Result<Tokens, HubError> {
            let access_claims = AccessClaims::new(username.to_string(), level, tariff);
            let refresh_claims = RefreshClaims::new(&access_claims);

            let tokens = Tokens {
                access_token: Tokens::encode_access_token(&access_claims)?,
                refresh_token: Tokens::encode_refresh_token(&refresh_claims)?,
            };

            Ok(tokens)
        }

        /// Создание токена доступа
        fn encode_access_token(ac: &AccessClaims) -> Result<String, HubError> {
            jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                ac,
                &EncodingKey::from_secret(SECRET.as_ref()),
            )
            .map_err(|err| {
                HubError::new_internal("Failed to create access token", Some(Vec::new()))
                    .add(format!("{}", err))
            })
        }

        /// Создание токена обновления
        fn encode_refresh_token(rc: &RefreshClaims) -> Result<String, HubError> {
            jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                rc,
                &EncodingKey::from_secret(SECRET.as_ref()),
            )
            .map_err(|err| {
                HubError::new_internal("Failed to create refresh token", Some(Vec::new()))
                    .add(format!("{}", err))
            })
        }

        /// Декодирование любого JWT токена, в зависимости от полезной нагрузки
        pub fn decode_token<T>(token: &'a str) -> Result<TokenData<T>, HubError>
        where
            T: DeserializeOwned,
        {
            match jsonwebtoken::decode::<T>(
                &token,
                &DecodingKey::from_secret(SECRET.as_ref()),
                &Validation::default(),
            ) {
                Ok(token_data) => Ok(token_data),
                Err(err) => match *err.kind() {
                    JwtErrorKind::ExpiredSignature => {
                        let kind = ErrorKind::Unauthorized(UnauthorizedErrorKind::TokenExpired);

                        Err(HubError::new(kind))
                    }
                    _ => Err(err_unauthorized!("Faild to decode token", err)),
                },
            }
        }
    }

    /// Тело запроса при при обновлении токена доступа
    #[derive(Debug, Clone, Deserialize)]
    pub struct RefreshResp<'a> {
        pub refresh_token: &'a str,
    }

    /// Вспомогательная функция
    /// Разрезает значение заголовка Authorization на части
    fn split_token(header_value: &str) -> Vec<&str> {
        let split = header_value.split(" ");
        split.collect::<Vec<&str>>()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn token_creation() {
            match super::Tokens::new("I0HuKc", super::Level::Padawan, super::Tariff::Basic) {
                Ok(tokens) => {
                    let at = super::Tokens::decode_token::<super::AccessClaims>(
                        tokens.access_token.as_str(),
                    )
                    .expect("valid access token");

                    assert_eq!(at.claims.username, String::from("I0HuKc"));
                    assert_eq!(at.claims.level, super::Level::Padawan);
                    assert_eq!(at.claims.tariff, super::Tariff::Basic);

                    let rt = super::Tokens::decode_token::<super::RefreshClaims>(
                        tokens.refresh_token.as_str(),
                    )
                    .expect("valid access token");

                    assert_eq!(rt.claims.username, String::from("I0HuKc"));
                }

                Err(err) => assert!(false, "{:?}", err),
            }
        }
    }
}

pub mod validation {
    use strum::IntoEnumIterator;

    pub fn level_validation(level: &str) -> Result<&str, super::HubError> {
        let result = super::Level::iter().find(|lev| {
            lev.to_string().to_lowercase() == level.to_string()
                && level.to_string() != super::Level::Sith.to_string().to_lowercase()
        });

        match result {
            Some(_) => Ok(level),
            None => Err(super::HubError::new_unprocessable(
                "Invalid format of level",
                None,
            )),
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;

        #[test_case("padawan", true ; "valid_level_padawan" )]
        #[test_case("master", true ; "valid_level_master" )]
        #[test_case("sith", false ; "invalid_level_sith" )]
        #[test_case("invalid", false ; "invalid_level" )]
        fn level_validation(level: &str, is_valid: bool) {
            match super::level_validation(level) {
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
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use validator::Validate;

    #[test_case("I0", "12344321e", false ; "username_lenght_min" )]
    #[test_case("1234567890123456", "12344321e", false ; "username_lenght_max" )]
    #[test_case("I0H uKc", "12344321e", false ; "invalid_format" )]
    #[test_case("I0HuKc", "1234", false ; "password_lenght_min" )]
    #[test_case("I0HuKc", "123456789012345678901234567890", false ; "password_lenght_max" )]
    #[test_case("I0HuKc", "12344321e", true ; "valid" )]
    fn new_user_validation(username: &str, password: &str, is_valid: bool) {
        let nu = super::NewUser {
            username: username.to_string(),
            password: password.to_string(),
        };

        match nu.validate() {
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
