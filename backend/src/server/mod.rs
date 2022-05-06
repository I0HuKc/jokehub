use rocket::{response::status::Created, serde::json::Json, Build, Rocket};

use crate::{
    db::{joke_repository, joke_repository::NewJokeOutcome, Conn, DbInit},
    model::joke::{Joke, NewJoke},
    Error,
};


mod errors;

pub trait Server {
    fn launch(self) -> Self;
}

#[post("/", data = "<nj>")]
async fn create(c: Conn, nj: Json<NewJoke>) -> Result<Created<Json<Joke>>, Json<Error>> {
    match joke_repository::create(c, nj.0).await {
        NewJokeOutcome::Ok(j) => Ok(Created::new("/").body(Json(j))),
        NewJokeOutcome::Other(err) => Err(Json(err)),
    }
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self.manage_db()
            .mount("/api/v1/joke", rocket::routes![create])
    }
}
