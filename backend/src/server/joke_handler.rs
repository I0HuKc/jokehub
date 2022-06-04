use mongodb::bson::doc;
use rocket::serde::json::Json;
use serde_json::{json, Value};

use crate::model::account::Tariff;
use crate::server::lingua::Lingua;
use crate::{
    db::mongo::{varys::Varys, Crud, MongoConn},
    err_not_found,
    errors::HubError,
    model::{
        account::security::{AuthGuard, LevelGuard},
        joke::*,
        shrimp::{Flags, Shrimp, Tail},
        validation::uuid_validation,
    },
    shrimp_reaction_handler,
};

shrimp_reaction_handler!(
    reaction_joke,
    "/joke/reaction/<record_id>/<reaction_kind>",
    Joke
);

#[post("/joke/new", data = "<jnj>")]
pub async fn create_joke<'f>(
    _auth: AuthGuard,
    client: MongoConn<'f>,
    lingua: Lingua<'f>,
    jnj: Json<NewJoke>,
) -> Result<Value, HubError> {
    let tail = Tail::new(
        Flags::default(),
        lingua.detected(jnj.clone().0.text)?,
        _auth.0.get_username(),
        &jnj.0.tags,
    );

    let body = Joke::from(jnj.0);

    let result = Shrimp::create(
        Varys::get(client.0.as_ref(), Varys::Joke),
        &Shrimp::new(body, tail),
    )?;
    let resp = json!({"id": result.inserted_id});

    Ok(resp)
}

#[get("/joke/<id>")]
pub async fn get_joke<'f>(
    _api_key: ApiKeyGuard,
    client: MongoConn<'f>,
    id: &str,
) -> Result<Value, HubError> {
    let result: Shrimp<Joke> = Shrimp::get_by_id(
        Varys::get(client.0.as_ref(), Varys::Joke),
        uuid_validation(id)?,
    )?;

    match _api_key.0 {
        Some(data) => Ok(result.tariffing(&data.get_tariff(), &None)),
        None => Ok(result.tariffing(&Tariff::default(), &None)),
    }
}

#[delete("/joke/<id>")]
pub async fn delete_joke<'f>(
    _level: LevelGuard,
    client: MongoConn<'f>,
    id: &str,
) -> Result<(), HubError> {
    Shrimp::<Joke>::del_by_id(
        Varys::get(client.0.as_ref(), Varys::Joke),
        uuid_validation(id)?,
    )
    .and_then(|d_result| {
        if d_result.deleted_count < 1 {
            Err(err_not_found!("joke"))
        } else {
            Ok(())
        }
    })
}
