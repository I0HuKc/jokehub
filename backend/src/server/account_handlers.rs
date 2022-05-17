use mongodb::bson::doc;
use r2d2_redis::redis::Commands;
use rocket::serde::json::Json;

use serde_json::{json, Value};
use validator::Validate;

use crate::{
    db::mongo::MongoConn,
    db::mongo::{varys::Varys, Crud},
    db::redis::RedisConn,
    errors::HubError,
    model::{
        account::{
            security::{AuthGuard, Tokens},
            *,
        },
    },
};


#[post("/registration", data = "<jnu>")]
pub async fn registration<'f>(client: MongoConn<'f>, jnu: Json<NewUser>) -> Result<Value, HubError> {
    jnu.0.validate()?;

    let result = User::create(
        Varys::get(client, Varys::Users),
        User::from(jnu.0).password_hashing()?
    )?;

    let resp = json!({"id": result.inserted_id});
    Ok(resp)
}

#[post("/login", data = "<jnu>")]
pub async fn login<'f>(client: MongoConn<'f>, mut redis: RedisConn, jnu: Json<NewUser>) -> Result<Json<Tokens>, HubError> {
    jnu.0.validate()?;

    let result = User::get_by_username(
        Varys::get(client, Varys::Users),
        jnu.0.username,
    )?;

    if result.password_verify(format!("{}", jnu.0.password).as_bytes())? {  
        let tokens = Tokens::new(result.username.clone(), result.role)?;                        

        // Сохранение токена обновления в redis             
        redis.set_ex::<String, String, ()>(tokens.refresh_token.clone(), result.username.clone(), 60*60*24*7)
        .map_err(|err| {
            HubError::new_internal("Falid to set in redis", Some(Vec::new())).add(format!("{}", err))            
        })?;


        Ok(Json(tokens))
    } else {      
        Err(HubError::new_not_found("User was not found", None))
    }
}

#[get("/account")]
pub async fn account<'f>(client: MongoConn<'f>, _auth : AuthGuard) -> Result<Json<User>, HubError> {
    let result = User::get_by_username(
        Varys::get(client, Varys::Users),
        _auth.0.get_username()
    )?;

    Ok(Json(result))
}
