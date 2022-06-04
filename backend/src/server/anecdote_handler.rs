use mongodb::bson::doc;
use rocket::serde::json::Json;
use serde_json::{json, Value};

use crate::{
    db::mongo::{varys::Varys, Crud, MongoConn},
    err_not_found,
    errors::HubError,
    model::{
        account::{
            security::{AuthGuard, LevelGuard},
            Tariff,
        },
        anecdote::*,
        shrimp::{Flags, Shrimp, Tail},
        validation::uuid_validation,
    },
    server::lingua::Lingua,
    shrimp_reaction_handler,
};

shrimp_reaction_handler!(
    reaction_anecdote,
    "/anecdote/reaction/<record_id>/<reaction_kind>",
    Anecdote
);

#[post("/anecdote/new", data = "<jna>")]
pub async fn create_anecdote<'f>(
    _auth: AuthGuard,
    client: MongoConn<'f>,
    lingua: Lingua<'f>,
    jna: Json<NewAnecdote>,
) -> Result<Value, HubError> {
    let tail = Tail::new(
        Flags::default(),
        lingua.detected(jna.clone().0.text)?,
        _auth.0.get_username(),
        &jna.0.tags,
    );
    let body = Anecdote::from(jna.0);

    let result = Shrimp::create(
        Varys::get(client.0.as_ref(), Varys::Anecdote),
        &Shrimp::new(body, tail),
    )?;

    let resp = json!({"id": result.inserted_id});
    Ok(resp)
}

#[get("/anecdote/<id>")]
pub async fn get_anecdote<'f>(
    _api_key: ApiKeyGuard,
    client: MongoConn<'f>,
    id: &str,
) -> Result<Value, HubError> {
    let result: Shrimp<Anecdote> = Shrimp::get_by_id(
        Varys::get(client.0.as_ref(), Varys::Anecdote),
        uuid_validation(id)?,
    )?;

    match _api_key.0 {
        Some(data) => Ok(result.tariffing(&data.get_tariff(), &None)),
        None => Ok(result.tariffing(&Tariff::default(), &None)),
    }
}

#[delete("/anecdote/<id>")]
pub async fn delete_anecdote<'f>(
    _level: LevelGuard,
    client: MongoConn<'f>,
    id: &str,
) -> Result<(), HubError> {
    Shrimp::<Anecdote>::del_by_id(
        Varys::get(client.0.as_ref(), Varys::Anecdote),
        uuid_validation(id)?,
    )
    .and_then(|d_result| {
        if d_result.deleted_count < 1 {
            Err(err_not_found!("anecdote"))
        } else {
            Ok(())
        }
    })
}
