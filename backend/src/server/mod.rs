use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use serde_json::json;
use std::env;
use std::str::FromStr;
use uuid::Uuid;
use validator::Validate;

use crate::db::ERR_ENV_MONGO_DB_NAME;
use crate::model::{
    anecdote::{Anecdote, NewAnecdote},
    joke::{Joke, NewJoke},
    shrimp::{Flags, Shrimp, Tail},
};
use crate::{
    db::{mongo::DB_ANECDOTE, DbInit, PgConn},
    Errors,
};
use mongodb::{bson::doc, bson::Document, sync::Client, sync::Collection};

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
async fn create_anecdote<'f>(
    client: &State<Box<Client>>,
    jna: Json<NewAnecdote>,
) -> Result<Json<Shrimp<Anecdote>>, Errors<'f>> {
    let db_name = env::var("MONGO_DATABASE_NAME").expect(ERR_ENV_MONGO_DB_NAME.clone());

    let collection: Collection<Document> = client
        .database(db_name.as_str())
        .collection(DB_ANECDOTE.clone());

    let tail = Tail::new(Flags::default(), jna.0.language, String::from("I0HuKc"));
    let body = Anecdote::new(jna.0.tags, jna.0.text);
    let shrimp: Shrimp<Anecdote> = Shrimp::new(body, tail);

    let doc = bson::to_document(&shrimp)?;
    collection.insert_one(doc, None)?;

    Ok(Json(shrimp))
}

#[get("/anecdote/<id>")]
async fn get_anecdote<'f>(
    client: &State<Box<Client>>,
    id: &str,
) -> Result<Json<Shrimp<Anecdote>>, Errors<'f>> {
    let db_name = env::var("MONGO_DATABASE_NAME").expect(ERR_ENV_MONGO_DB_NAME.clone());
    let collection: Collection<Shrimp<Anecdote>> = client
        .database(db_name.as_str())
        .collection(DB_ANECDOTE.clone());

    match collection.find_one(doc! { "_id":  Uuid::from_str(id)?}, None)? {
        Some(a) => Ok(Json(a)),
        None => Err(Errors::new(Status::NotFound).add("mdb", json!("record not found"))),
    }
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self.manage_postgres().manage_mongodb().mount(
            "/v1",
            rocket::routes![create_joke, create_anecdote, get_anecdote],
        )
    }
}
