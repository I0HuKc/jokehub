use rocket::serde::json::{json, Json, Value};
use rocket::{routes, Build, Rocket};
use uuid::Uuid;
use chrono::Utc;

use crate::db::PgConn;

use crate::model::joke::{Joke, NewJoke};

#[get("/<language>/random", format = "json")]
fn random_category(language: &str) -> Option<Json<Joke>> {
    Some(Json(Joke {
        uuid: Uuid::new_v4(),
        category: "programming".to_string(),
        language: language.to_string(),
        setup: "Как называется бесполезная кожа вокруг вагины?".to_string(),
        punchline: Some("Женщина".to_string()),
        created_at: Utc::now().naive_utc(),
    }))
}

#[get("/<language>/<category>/random", format = "json")]
fn random(language: &str, category: &str) -> Option<Json<Joke>> {
    Some(Json(Joke {
        uuid: Uuid::new_v4(),
        category: category.to_string(),
        language: language.to_string(),
        setup: "Как называется бесполезная кожа вокруг вагины?".to_string(),
        punchline: Some("Женщина".to_string()),
        created_at: Utc::now().naive_utc(),
    }))
}

#[post("/new", format = "json", data = "<joke>")]
fn new_joke(conn: PgConn, joke: Json<NewJoke>) -> Option<Json<Joke>> {
    // conn.run(move |c| {
    //     diesel::insert_into(target)
    // });
    
    
    Some(Json(Joke::from(joke.0)))
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn launcher() -> Rocket<Build> {
    rocket::build()
        .attach(PgConn::fairing())
        .mount("/api/v1/joke", routes![random, random_category, new_joke])
        .register("/api/v1", rocket::catchers![not_found])
}
