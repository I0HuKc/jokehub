use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
    db::mongo::MongoConn,
    db::mongo::{varys::Varys, Crud},
    errors::HubError,
    model::{
        account::{favorites::*, security::AuthGuard},
        validation::uuid_validation,
    },
};

#[post("/account/favorite/<record_id>")]
pub fn favorite_add<'f>(
    _auth: AuthGuard,
    client: MongoConn<'f>,
    record_id: &str,
) -> Result<(), HubError> {
    let fv = Favorite::new(
        uuid_validation(record_id)?.to_string(),
        _auth.0.get_username(),
    );

    Favorite::create(Varys::get(client.0.as_ref(), Varys::Favorite), fv)?;

    Ok(())
}

#[delete("/account/favorite/<record_id>")]
pub fn favorite_remove<'f>(
    _auth: AuthGuard,
    client: MongoConn<'f>,
    record_id: &str,
) -> Result<(), HubError> {
    Favorite::del_by_record_id(client.0.as_ref(), record_id)
}


// pub fn favorite_all<'f>(
//     _auth: AuthGuard,
//     client: MongoConn<'f>,
//     record_id: &str,
// ) -> Result<Json<Vec<Favorite>>, HubError> {

// }
