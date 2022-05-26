use mongodb::bson::doc;
use rocket::serde::json::Json;

use serde_json::{json, Value};
use validator::Validate;

use crate::{
    db::mongo::MongoConn,
    db::mongo::{varys::Varys, Crud},
    err_not_found, err_unauthorized,
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

    let result =
        User::get_by_username(Varys::get(client.0.as_ref(), Varys::Users), jnu.0.username)?;

    if result.password_verify(format!("{}", jnu.0.password).as_bytes())? {
        let tokens = Tokens::new(result.username.clone(), result.level, result.tariff)?;

        // Сохранение токена обновления
        Session::new(result.username.clone(), tokens.refresh_token.clone())
            .set(client.0.as_ref())?;

        Ok(Json(tokens))
    } else {
        Err(err_not_found!("user"))
    }
}

#[get("/account")]
pub async fn account<'f>(client: MongoConn<'f>, _auth: AuthGuard) -> Result<Value, HubError> {
    let result = User::get_by_username(
        Varys::get(client.0.as_ref(), Varys::Users),
        _auth.0.get_username(),
    )?;

    Ok(result.secure())
}

#[post("/account/token/refresh", data = "<jrt>")]
pub fn refresh_token<'f>(
    client: MongoConn<'f>,
    jrt: Json<RefreshResp<'f>>,
) -> Result<Json<Tokens>, HubError> {
    // Валидирую входярий токен
    let refresh_claims = Tokens::decode_token::<RefreshClaims>(jrt.0.refresh_token)?.claims;

    // Удаляю старый токен
    Session::drop(jrt.0.refresh_token, client.0.as_ref()).and_then(|res| {
        // Если токена не существовало
        if res.deleted_count != 1 {
            Err(err_unauthorized!("Token is not found"))
        } else {
            Ok(())
        }
    })?;

    // Достаю пользователя из БД
    let result = User::get_by_username(
        Varys::get(client.0.as_ref(), Varys::Users),
        refresh_claims.get_username(),
    )?;

    // Создаю новую пару токенов
    let new_tokens = Tokens::new(result.username.clone(), result.level, result.tariff)?;

    // Сохраняю новые токены
    Session::new(
        refresh_claims.get_username(),
        new_tokens.clone().refresh_token,
    )
    .set(client.0.as_ref())?;

    Ok(Json(new_tokens))
}

#[post("/account/logout", data = "<jrt>")]
pub fn logout<'f>(
    _auth: AuthGuard,
    client: MongoConn<'f>,
    jrt: Json<RefreshResp<'f>>,
) -> Result<(), HubError> {
    // Валидирую входярий токен
    Tokens::decode_token::<RefreshClaims>(jrt.0.refresh_token)?;

    // Удаляю токен
    Session::drop(jrt.0.refresh_token, client.0.as_ref()).and_then(|res| {
        // Если токена не существовало
        if res.deleted_count != 1 {
            Err(err_unauthorized!("Token is not found"))
        } else {
            Ok(())
        }
    })
}

#[delete("/account/delete")]
pub fn delete_account<'f>(_auth: AuthGuard, client: MongoConn<'f>) -> Result<(), HubError> {
    User::del_by_username(
        Varys::get(client.0.as_ref(), Varys::Users),
        _auth.0.get_username(),
    )
    .and_then(|d_result| {
        if d_result.deleted_count < 1 {
            Err(err_not_found!("user"))
        } else {
            Ok(())
        }
    })
}

#[put("/privilege/<username>/<level>")]
pub async fn privilege<'f>(
    _level: LevelGuard,
    client: MongoConn<'f>,
    username: &str,
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
    .and_then(|up_result| {
        if up_result.matched_count < 1 {
            Err(err_not_found!("user"))
        } else {
            Ok(())
        }
    })
}
