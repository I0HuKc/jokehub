use mongodb::bson::doc;
use rocket::serde::json::Json;
use serde_json::{json, Value};

use crate::{
    err_not_found,
    db::mongo::{varys::Varys, Crud, MongoConn},
    errors::HubError,
    model::{
        uuid_validation,
        account::security::{AuthGuard, TariffGuard, MasterGuard},
        anecdote::*,
        shrimp::{Flags, Shrimp, Tail},
    },
};

#[post("/anecdote/new", data = "<jna>")]
pub async fn create_anecdote<'f>(_auth: AuthGuard, client: MongoConn<'f>, jna: Json<NewAnecdote>) -> Result<Value, HubError> {
    let tail = Tail::new(
        Flags::default(),
        &jna.0.language,
        _auth.0.get_username(),
        &jna.0.tags,
    );
    let body = Anecdote::from(jna.0);

    let result = Shrimp::create(
        Varys::get(client, Varys::Anecdote), 
        Shrimp::new(body, tail),
    )?;

    let resp = json!({"id": result.inserted_id});
    Ok(resp)
}

#[get("/anecdote/<id>")]
pub async fn get_anecdote<'f>(_tariff: TariffGuard, client: MongoConn<'f>, id: &str) -> Result<Value, HubError> {
    let result: Shrimp<Anecdote> = Shrimp::get_by_id(
        Varys::get(client, Varys::Anecdote), 
        uuid_validation(id)?,
    )?;

    Ok(result.tariffing(_tariff.0, _tariff.1))
}

#[delete("/anecdote/<id>")]
pub async fn delete_anecdote<'f>(_level: MasterGuard, client: MongoConn<'f>, id: &str) -> Result<(), HubError> {
    Shrimp::<Anecdote>::del_by_id(
        Varys::get(client, Varys::Anecdote),
        uuid_validation(id)?,
    ).and_then(
        |d_result| {
            if d_result.deleted_count < 1 {
                Err(err_not_found!("anecdote"))
            } else {
                Ok(())
            }
        },
    )
}
