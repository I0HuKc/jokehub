use rocket::serde::json::{json, Json, Value};
use rocket::{routes, Build, Rocket};
use uuid::Uuid;

use crate::model::joke::{Joke, NewJoke};

#[get("/<language>/random", format = "json")]
fn random_category(language: &str) -> Option<Json<Joke>> {
    Some(Json(Joke {
        uuid: Uuid::new_v4().to_string(),
        category: "programming".to_string(),
        language: language.to_string(),
        setup: "Как называется бесполезная кожа вокруг вагины?".to_string(),
        punchline: Some("Женщина".to_string()),
        created_at: "date".to_string(),
    }))
}

#[get("/<language>/<category>/random", format = "json")]
fn random(language: &str, category: &str) -> Option<Json<Joke>> {
    Some(Json(Joke {
        uuid: Uuid::new_v4().to_string(),
        category: category.to_string(),
        language: language.to_string(),
        setup: "Как называется бесполезная кожа вокруг вагины?".to_string(),
        punchline: Some("Женщина".to_string()),
        created_at: "date".to_string(),
    }))
}

#[post("/new", format = "json", data = "<joke>")]
fn new_joke(joke: Json<NewJoke>) -> Option<Json<Joke>> {
    Some(Json(Joke {
        uuid: Uuid::new_v4().to_string(),
        category: joke.category.to_string(),
        language: joke.language.to_string(),
        setup: joke.setup.to_string(),
        punchline: Some(joke.punchline.unwrap_or_default().to_owned()),
        created_at: "date".to_string(),
    }))
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
        .mount("/api/v1/joke", routes![random, random_category, new_joke])
        .register("/api/v1", catchers![not_found])
}
