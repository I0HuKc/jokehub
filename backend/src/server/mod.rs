use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use validator::Validate;

use crate::{
    db::{PgConn, DbInit},
    model::joke::{Joke, NewJoke},
    Errors,
};

pub trait Server {
    fn launch(self) -> Self;
}

#[post("/", data = "<jnj>")]
async fn create<'f>(c: PgConn, jnj: Json<NewJoke>) -> Result<Json<Joke>, Errors<'f>> {
    jnj.0.validate()?;
    let joke = Joke::create(c, jnj.0).await?;
    Ok(Json(joke))
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self.manage_postgres()
            .manage_mongodb()
            .mount("/api/v1/joke", rocket::routes![create])
    }
}
