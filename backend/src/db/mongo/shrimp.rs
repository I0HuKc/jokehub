use mongodb::{bson::doc, bson::Document, results::InsertOneResult, sync::Collection};
use rocket::serde::DeserializeOwned;
use serde::Serialize;

use crate::{
    db::mongo::Crud,
    err_internal, err_not_found,
    errors::HubError,
    model::shrimp::{Paws, Shrimp},
};

impl<'a, T> Crud<'a, Shrimp<T>> for Shrimp<T>
where
    T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync,
    T: Paws,
{
    fn create(
        collection: Collection<Document>,
        data: Shrimp<T>,
    ) -> Result<InsertOneResult, HubError> {
        let doc = bson::to_document(&data)?;
        let rersult = collection.insert_one(doc, None)?;

        Ok(rersult)
    }

    fn get_by_id(collection: Collection<Shrimp<T>>, id: &str) -> Result<Shrimp<T>, HubError> {
        let filter = doc! {"_id": id};
        let update = doc! {"$inc": {"_header.counter": 1}};

        match collection.find_one_and_update(filter, update, None) {
            Ok(result) => {
                if let Some(shrimp) = result {
                    Ok(shrimp)
                } else {
                    Err(err_not_found!(collection.name()))
                }
            }

            Err(err) => Err(err_internal!(err.to_string())),
        }
    }
}
