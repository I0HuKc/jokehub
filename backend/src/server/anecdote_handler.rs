use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};
use mongodb::{bson::doc, sync::Client};

use crate::model::{
    anecdote::{Anecdote, NewAnecdote},
    shrimp::{Flags, Shrimp, Tail},
};
use crate::{
    db::mongo::{varys::Varys, Crud},
    errors::Errors,
};

#[post("/anecdote/new", data = "<jna>")]
pub async fn create_anecdote<'f>(client: &State<Box<Client>>, jna: Json<NewAnecdote>) -> Result<Value, Errors<'f>> {
    let tail = Tail::new(Flags::default(), jna.0.language, String::from("I0HuKc"));
    let body = Anecdote::new(jna.0.tags, jna.0.text);

    let result = Shrimp::create(
        Varys::get(client, Varys::Anecdote)?,
        Shrimp::new(body, tail),
    )?;

    let resp = json!({"id": result.inserted_id});
    Ok(resp)
}

#[get("/anecdote/<id>")]
pub async fn get_anecdote<'f>(client: &State<Box<Client>>, id: &str) -> Result<Json<Shrimp<Anecdote>>, Errors<'f>> {    
    let result: Shrimp<Anecdote> = Shrimp::get_by_id(
        Varys::get(client, Varys::Anecdote)?, 
        id,
    )?;

    Ok(Json(result))
}