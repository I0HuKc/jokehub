use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use validator::Validate;

use crate::{
    db::{Conn, DbInit},
    model::joke::{Joke, NewJoke},
    Errors,
};

pub trait Server {
    fn launch(self) -> Self;
}

#[post("/", data = "<jnj>")]
async fn create(c: Conn, jnj: Json<NewJoke>) -> Result<Json<Joke>, Errors> {
    jnj.0.validate()?;
    let joke = Joke::create(c, jnj.0).await?;
    Ok(Json(joke))
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self.manage_db()
            .mount("/api/v1/joke", rocket::routes![create])
    }
}
