use mongodb::bson::doc;
use rocket::serde::json::Json;
use serde_json::{json, Value};
use validator::Validate;

use crate::model::{
    account::{
        security::{AuthGuard, LevelGuard},
        Tariff,
    },
    punch::*,
    shrimp::{Flags, Shrimp, Tail},
    validation::uuid_validation,
};
use crate::{
    db::mongo::MongoConn,
    db::mongo::{varys::Varys, Crud},
    err_not_found,
    errors::HubError,
    server::lingua::Lingua,
    shrimp_reaction_handler,
};

shrimp_reaction_handler!(
    reaction_punch,
    "/punch/reaction/<record_id>/<reaction_kind>",
    Punch
);

#[post("/punch/new", data = "<jnp>")]
pub async fn create_punch<'f>(
    _auth: AuthGuard,
    client: MongoConn<'f>,
    lingua: Lingua<'f>,
    jnp: Json<NewPunch>,
) -> Result<Value, HubError> {
    jnp.0.validate()?;

    let tail = Tail::new(
        Flags::default(),
        lingua.detected(jnp.clone().0.setup)?,
        _auth.0.get_username(),
        &jnp.0.tags,
    );
    let body = Punch::from(jnp.0);

    let result = Shrimp::create(
        Varys::get(client.0.as_ref(), Varys::Punch),
        &Shrimp::new(body, tail),
    )?;

    let resp = json!({"id": result.inserted_id});
    Ok(resp)
}

#[get("/punch/<id>")]
pub async fn get_punch<'f>(
    _api_key: ApiKeyGuard,
    client: MongoConn<'f>,
    id: &str,
) -> Result<Value, HubError> {
    let result: Shrimp<Punch> = Shrimp::get_by_id(
        Varys::get(client.0.as_ref(), Varys::Punch),
        uuid_validation(id)?,
    )?;

    match _api_key.0 {
        Some(data) => Ok(result.tariffing(&data.get_tariff(), &None)),
        None => Ok(result.tariffing(&Tariff::default(), &None)),
    }
}

#[delete("/punch/<id>")]
pub async fn delete_punch<'f>(
    _level: LevelGuard,
    client: MongoConn<'f>,
    id: &str,
) -> Result<(), HubError> {
    Shrimp::<Punch>::del_by_id(
        Varys::get(client.0.as_ref(), Varys::Punch),
        uuid_validation(id)?,
    )
    .and_then(|d_result| {
        if d_result.deleted_count < 1 {
            Err(err_not_found!("punch"))
        } else {
            Ok(())
        }
    })
}
