use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use crate::{
    db::{Conn, DbInit},
    model::joke::{Joke, NewJoke},
    Error,
};

pub trait Server {
    fn launch(self) -> Self;
}

#[post("/", data = "<nj>")]
async fn create(c: Conn, nj: Json<NewJoke>) -> Result<Json<Joke>, Error> {
    let joke = Joke::create(c, nj.0).await?;

    Ok(Json(joke))
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self.manage_db()
            .mount("/api/v1/joke", rocket::routes![create])
    }
}
