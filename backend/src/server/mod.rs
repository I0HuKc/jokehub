use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use crate::{
    db::{Conn, DbInit},
    model::joke::{Joke, NewJoke},
    Outcome,
};

mod response;
use response::Response;

pub trait Server {
    fn launch(self) -> Self;
}

#[post("/", data = "<nj>")]
async fn create(c: Conn, nj: Json<NewJoke>) -> Response<'static> {
    match Joke::create(c, nj.0).await {
        Outcome::Ok(j) => Response::new(j, Status::Created),
        Outcome::AlreadyExists(err) => Response::new(err, Status::UnprocessableEntity),
        Outcome::Other(err) => Response::new(err, Status::InternalServerError),
    }
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self.manage_db()
            .mount("/api/v1/joke", rocket::routes![create])
    }
}
