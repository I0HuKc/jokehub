use mongodb::bson::doc;
use rocket::serde::json::Json;
use serde_json::{json, Value};
use validator::Validate;

// #[macro_use]
// use crate::errors;

use crate::{
    db::mongo::MongoConn,
    db::mongo::{varys::Varys, Crud},
    errors::HubError, err_not_found
};
use crate::{
    model::{
        uuid_validation,
        punch::*,
        account::security::AuthGuard,
        shrimp::{Flags, Shrimp, Tail},
    },
};

#[post("/punch/new", data = "<jnp>")]
pub async fn create_punch<'f>(_auth: AuthGuard, client: MongoConn<'f>, jnp: Json<NewPunch>) -> Result<Value, HubError> {
    jnp.0.validate()?;
    
    let tail = Tail::new(
        Flags::default(), 
        &jnp.0.language, 
        _auth.0.get_username(), 
        &jnp.0.tags,
    );
    let body = Punch::from(jnp.0);

    let result = Shrimp::create(
        Varys::get(client, Varys::Punch),
        Shrimp::new(body, tail),
    )?;

    let resp = json!({"id": result.inserted_id});
    Ok(resp)
}

#[get("/punch/<id>")]
pub async fn get_punch<'f>(client: MongoConn<'f>, id: &str) -> Result<Json<Shrimp<Punch>>, HubError> {    
    uuid_validation(id)?;

    let result: Shrimp<Punch> = Shrimp::get_by_id(
        Varys::get(client, Varys::Punch), 
        id,
    )?;

    Ok(Json(result))
}

#[delete("/punch/<id>")]
pub async fn delete_punch<'f>(_auth: AuthGuard, client: MongoConn<'f>, id: &str) -> Result<(), HubError> {
    Shrimp::<Punch>::del_by_id(
        Varys::get(client, Varys::Punch),
        id
    ).and_then(|d_result| {
        if d_result.deleted_count < 1 {
            Err(err_not_found!("punch"))
        } else {
            Ok(())
        }
    })  
}