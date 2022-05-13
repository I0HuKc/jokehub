use mongodb::{bson::doc, sync::Client};
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};

use crate::model::{
    punch::*,
    shrimp::{Flags, Shrimp, Tail},
};
use crate::{
    db::mongo::{varys::Varys, Crud},
    errors::Errors,
};

#[post("/punch/new", data = "<jnp>")]
pub async fn create_punch<'f>(client: &State<Box<Client>>, jnp: Json<NewPunch>) -> Result<Value, Errors<'f>> {
    let tail = Tail::new(
        Flags::default(), 
        &jnp.0.language, 
        String::from("I0HuKc"), 
        &jnp.0.tags,
    );
    let body = Punch::new(&jnp);

    let result = Shrimp::create(
        Varys::get(client, Varys::Punch),
        Shrimp::new(body, tail),
    )?;

    let resp = json!({"id": result.inserted_id});
    Ok(resp)
}

#[get("/punch/<id>")]
pub async fn get_punch<'f>(client: &State<Box<Client>>, id: &str) -> Result<Json<Shrimp<Punch>>, Errors<'f>> {    
    let result: Shrimp<Punch> = Shrimp::get_by_id(
        Varys::get(client, Varys::Punch), 
        id,
    )?;

    Ok(Json(result))
}