use bson::Document;
use mongodb::sync::Collection;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use serde_json::{json, Value};
use validator::Validate;

use crate::model::{
    anecdote::{Anecdote, NewAnecdote},
    joke::{Joke, NewJoke},
    shrimp::{Flags, Shrimp, Tail},
};
use crate::{
    db::mongo::{Varys, Crud},
    db::{DbInit, PgConn},
    Errors,
};
use mongodb::{bson::doc, sync::Client};

pub trait Server {
    fn launch(self) -> Self;
}

#[post("/joke/new", data = "<jnj>")]
async fn create_joke<'f>(c: PgConn, jnj: Json<NewJoke>) -> Result<Json<Joke>, Errors<'f>> {
    jnj.0.validate()?;
    let joke = Joke::create(c, jnj.0).await?;
    Ok(Json(joke))
}

#[post("/anecdote/new", data = "<jna>")]
async fn create_anecdote<'f>(client: &State<Box<Client>>, jna: Json<NewAnecdote>) -> Result<Value, Errors<'f>> {
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
async fn get_anecdote<'f>(client: &State<Box<Client>>, id: &str) -> Result<Json<Shrimp<Anecdote>>, Errors<'f>> {    
    let result: Shrimp<Anecdote> = Shrimp::get_by_id(
        Varys::get(client, Varys::Anecdote)?, 
        id,
    )?;

    Ok(Json(result))
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self.manage_postgres().manage_mongodb().mount(
            "/v1",
            rocket::routes![create_joke, create_anecdote, get_anecdote],
        )
    }
}
