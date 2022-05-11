use rocket::serde::json::Json;
use validator::Validate;

use crate::model::joke::{Joke, NewJoke};
use crate::{
    db:: PgConn,
    Errors,
};



#[post("/joke/new", data = "<jnj>")]
pub async fn create_joke<'f>(c: PgConn, jnj: Json<NewJoke>) -> Result<Json<Joke>, Errors<'f>> {
    jnj.0.validate()?;
    let joke = Joke::create(c, jnj.0).await?;
    Ok(Json(joke))
}
