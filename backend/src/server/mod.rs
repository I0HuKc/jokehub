mod helper;
mod response;

use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use crate::{
    db::{Conn, DbInit},
    model::joke::{Joke, NewJoke},
};

use response::Response;

pub trait Server {
    fn launch(self) -> Self;
}

#[post("/", data = "<nj>")]
async fn create(c: Conn, nj: Json<NewJoke>) -> Response<'static> {
    let (res, status) = helper::db_answer_handle(Joke::create(c, nj.0).await);
    match res {
        Ok(j) => Response::new(j, status),
        Err(err) => Response::new(err, status),
    }
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self.manage_db()
            .mount("/api/v1/joke", rocket::routes![create])
    }
}
