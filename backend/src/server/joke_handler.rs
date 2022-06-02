use mongodb::bson::doc;
use rocket::serde::json::Json;
use serde_json::{json, Value};

use crate::server::lingua::Lingua;
use crate::{
    db::mongo::{varys::Varys, Crud, MongoConn},
    err_not_found,
    errors::HubError,
    model::{
        account::security::{AuthGuard, LevelGuard, TariffGuard},
        joke::*,
        shrimp::{Flags, Shrimp, Tail},
        validation::uuid_validation,
    },
};

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
    _tariff: TariffGuard,
    client: MongoConn<'f>,
    id: &str,
) -> Result<Value, HubError> {
    let result: Shrimp<Joke> = Shrimp::get_by_id(
        Varys::get(client.0.as_ref(), Varys::Joke),
        uuid_validation(id)?,
    )?;

    Ok(result.tariffing(&_tariff.0, &_tariff.1))
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
