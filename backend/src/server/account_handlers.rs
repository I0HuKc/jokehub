use mongodb::bson::doc;
use r2d2_redis::redis::Commands;
use rocket::serde::json::Json;

use serde_json::{json, Value};
use validator::Validate;

use crate::{
    db::mongo::MongoConn,
    db::mongo::{varys::Varys, Crud},
    db::redis::RedisConn,
    err_internal, err_not_found, err_unauthorized,
    errors::HubError,
    model::account::{
        security::{AuthGuard, RefreshClaims, RefreshResp, Tokens},
        *,
    },
};

#[post("/registration", data = "<jnu>")]
pub async fn registration<'f>(
    client: MongoConn<'f>,
    jnu: Json<NewUser>,
) -> Result<Value, HubError> {
    jnu.0.validate()?;

    let result = User::create(
        Varys::get(client, Varys::Users),
        User::from(jnu.0).password_hashing()?,
    )?;

    let resp = json!({"id": result.inserted_id});
    Ok(resp)
}

#[post("/login", data = "<jnu>")]
pub async fn login<'f>(
    client: MongoConn<'f>,
    mut redis: RedisConn,
    jnu: Json<NewUser>,
) -> Result<Json<Tokens>, HubError> {
    jnu.0.validate()?;

    let result = User::get_by_username(Varys::get(client, Varys::Users), jnu.0.username)?;

    if result.password_verify(format!("{}", jnu.0.password).as_bytes())? {
        let tokens = Tokens::new(result.username.clone(), result.level, result.tariff)?;

        // Сохранение токена обновления в redis
        redis
            .set_ex::<String, String, ()>(
                tokens.refresh_token.clone(),
                result.username.clone(),
                60 * 60 * 24 * 7,
            )
            .map_err(|err| err_internal!("Falid to set in redis", err))?;

        Ok(Json(tokens))
    } else {
        Err(err_not_found!("user"))
    }
}

#[get("/account")]
pub async fn account<'f>(
    client: MongoConn<'f>,
    _auth: AuthGuard,
) -> Result<Json<UserResp>, HubError> {
    let result = User::get_by_username(Varys::get(client, Varys::Users), _auth.0.get_username())?;

    Ok(Json(UserResp::from(result)))
}

#[post("/account/token/refresh", data = "<jrt>")]
pub fn refresh_token<'f>(
    client: MongoConn<'f>,
    mut redis: RedisConn,
    jrt: Json<RefreshResp<'f>>,
) -> Result<Json<Tokens>, HubError> {
    // Валидирую входярий токен
    let refresh_claims = Tokens::decode_token::<RefreshClaims>(jrt.0.refresh_token)?.claims;

    // Удаляю старый токен
    redis
        .del::<&str, usize>(jrt.0.refresh_token)
        .map_err(|err| err_unauthorized!("Falid to drop token", err))
        .and_then(|res| {
            // Если токена не существовало
            if res != 1 {
                Err(err_unauthorized!("Token is not found"))
            } else {
                Ok(())
            }
        })?;

    // Достаю пользователя из БД
    let result = User::get_by_username(
        Varys::get(client, Varys::Users),
        refresh_claims.get_username(),
    )?;

    // Создаю новую пару токенов
    let new_tokens = Tokens::new(result.username.clone(), result.level, result.tariff)?;

    Ok(Json(new_tokens))
}

#[post("/account/logout", data = "<jrt>")]
pub fn logout<'f>(
    _auth: AuthGuard,
    mut redis: RedisConn,
    jrt: Json<RefreshResp<'f>>,
) -> Result<(), HubError> {
    // Валидирую входярий токен
    Tokens::decode_token::<RefreshClaims>(jrt.0.refresh_token)?;

    // Удаляю токен
    redis
        .del::<&str, usize>(jrt.0.refresh_token)
        .map_err(|err| err_unauthorized!("Falid to drop token", err))
        .and_then(|res| {
            // Если токена не существовало
            if res != 1 {
                Err(err_unauthorized!("Token is not found"))
            } else {
                Ok(())
            }
        })?;

    Ok(())
}

#[delete("/account/delete")]
pub fn delete_account<'f>(_auth: AuthGuard, client: MongoConn<'f>) -> Result<(), HubError> {
    User::del_by_username(Varys::get(client, Varys::Users), _auth.0.get_username()).and_then(
        |d_result| {
            if d_result.deleted_count < 1 {
                Err(err_not_found!("user"))
            } else {
                Ok(())
            }
        },
    )
}
