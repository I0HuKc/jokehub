use mongodb::bson::doc;
use rocket::serde::json::Json;

use serde_json::{json, Value};
use validator::Validate;

use crate::{
    db::mongo::MongoConn,
    db::mongo::{varys::Varys, Crud},
    err_not_found,
    errors::HubError,
    model::{
        account::{
            security::{AuthGuard, LevelGuard, RefreshClaims, RefreshResp, Session, Tokens},
            validation::level_validation,
            *,
        },
        validation::query_validation,
    },
};

#[post("/registration", data = "<jnu>")]
pub async fn registration<'f>(
    client: MongoConn<'f>,
    jnu: Json<NewUser>,
) -> Result<Value, HubError> {
    jnu.0.validate()?;

    let result = User::create(
        Varys::get(client.0.as_ref(), Varys::Users),
        User::from(jnu.0).password_hashing()?,
    )?;

    let resp = json!({"id": result.inserted_id});
    Ok(resp)
}

#[post("/login", data = "<jnu>")]
pub async fn login<'f>(
    client: MongoConn<'f>,
    jnu: Json<NewUser>,
) -> Result<Json<Tokens>, HubError> {
    jnu.0.validate()?;

    let result = User::get_by_username(client.0.as_ref(), jnu.0.username)?;

    if result.password_verify(format!("{}", jnu.0.password).as_bytes())? {
        let tokens = Tokens::new(&result.username, result.level, result.tariff)?;

        // Сохранение токена обновления
        Session::new(&result.username, &tokens.refresh_token).set(client.0.as_ref())?;

        Ok(Json(tokens))
    } else {
        Err(err_not_found!("user"))
    }
}

#[get("/account")]
pub async fn account<'f>(
    client: MongoConn<'f>,
    _auth: AuthGuard,
) -> Result<Json<Account>, HubError> {
    let user = User::get_by_username(client.0.as_ref(), _auth.0.get_username())?;
    let sessions = Session::roll(user.username.as_str(), client.0.as_ref())?;

    Ok(Json(Account::new(user, sessions)))
}

#[post("/account/token/refresh", data = "<jrt>")]
pub fn refresh_token<'f>(
    client: MongoConn<'f>,
    jrt: Json<RefreshResp<'f>>,
) -> Result<Json<Tokens>, HubError> {
    // Валидирую входярий токен
    let refresh_claims = Tokens::decode_token::<RefreshClaims>(jrt.0.refresh_token)?.claims;

    // Удаляю старый токен
    Session::drop(jrt.0.refresh_token, client.0.as_ref())?;

    // Достаю пользователя из БД
    let result = User::get_by_username(client.0.as_ref(), refresh_claims.get_username())?;

    // Создаю новую пару токенов
    let new_tokens = Tokens::new(&result.username, result.level, result.tariff)?;

    // Сохраняю новые токены
    Session::new(
        &refresh_claims.get_username(),
        &new_tokens.clone().refresh_token,
    )
    .set(client.0.as_ref())?;

    Ok(Json(new_tokens))
}

#[post("/account/password/change", data = "<jcp>")]
pub fn change_password<'f>(
    _auth: AuthGuard,
    client: MongoConn<'f>,
    jcp: Json<ChangePassword>,
) -> Result<Json<Tokens>, HubError> {
    jcp.validate()?;

    // Проверяю что пароли отличаются
    if jcp.0.old_password == jcp.0.new_password {
        return Err(HubError::new_unprocessable(
            "The new password must be different from the old one",
            None,
        ));
    }

    // Достаю пользователя из БД
    let user = User::get_by_username(client.0.as_ref(), _auth.0.get_username())?;

    // Проверяю хеши паролей
    if user.password_verify(format!("{}", jcp.0.old_password).as_bytes())? {
        // Создаю хеш нового пароля
        let hash = User::password_hashing_apart(&jcp.0.new_password)?;

        // Дропаю все активные сессии
        Session::drop_all(_auth.0.get_username().as_str(), client.0.as_ref())?;

        // Создаю новые токены
        let tokens = Tokens::new(&_auth.0.get_username(), user.level, user.tariff)?;

        // Создаю новую сессию
        Session::new(&_auth.0.get_username(), &tokens.refresh_token).set(client.0.as_ref())?;

        // Обновляю запись в БД
        User::update_password(client.0.as_ref(), _auth.0.get_username(), hash)?;

        return Ok(Json(tokens));
    }

    Err(err_not_found!("user"))
}

#[post("/account/logout", data = "<jrt>")]
pub fn logout<'f>(
    _auth: AuthGuard,
    client: MongoConn<'f>,
    jrt: Json<RefreshResp<'f>>,
) -> Result<(), HubError> {
    // Валидирую входящий токен
    Tokens::decode_token::<RefreshClaims>(jrt.0.refresh_token)?;

    // Удаляю токен
    Session::drop(jrt.0.refresh_token, client.0.as_ref())
}

#[post("/account/logout/any")]
pub fn logout_any<'f>(_auth: AuthGuard, client: MongoConn<'f>) -> Result<(), HubError> {
    // Дропаю все активные сессии
    Session::drop_all(_auth.0.get_username().as_str(), client.0.as_ref())
}

#[delete("/account/delete")]
pub fn delete_account<'f>(_auth: AuthGuard, client: MongoConn<'f>) -> Result<(), HubError> {
    User::del_by_username(client.0.as_ref(), _auth.0.get_username())
}

#[put("/privilege/<username>/<level>")]
pub async fn privilege<'f>(
    _level: LevelGuard,
    client: MongoConn<'f>,
    username: &'f str,
    level: &str,
) -> Result<(), HubError> {
    if query_validation(username)? == _level.0.get_username() {
        return Err(HubError::new_unprocessable(
            "You can't change your role",
            None,
        ));
    }

    User::privilege_set(
        Varys::get(client.0.as_ref(), Varys::Users),
        query_validation(username)?,
        level_validation(level)?,
    )
}
