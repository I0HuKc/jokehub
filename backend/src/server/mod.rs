use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use std::env;
use validator::Validate;

use crate::db::ERR_ENV_MONGO_DB_NAME;
use crate::model::{
    anecdote::{Anecdote, NewAnecdote},
    joke::{Joke, NewJoke},
    shrimp::{Flags, HeadSlim, Shrimp, Tail},
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

    let hs = HeadSlim::new(String::from("I0HuKc"));
    let tail = Tail::new(Flags::default(), jna.0.language);
    let body = Anecdote::new(jna.0.tags, jna.0.text);
    let shrimp: Shrimp<Anecdote> = Shrimp::new(hs, body, tail);

    let doc = bson::to_document(&shrimp)?;
    collection.insert_one(doc, None)?;

    Ok(Json(shrimp))
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self.manage_postgres()
            .manage_mongodb()
            .mount("/api/v1", rocket::routes![create_joke, create_anecdote])
    }
}
