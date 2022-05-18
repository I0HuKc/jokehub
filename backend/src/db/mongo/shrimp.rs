use mongodb::{bson::doc, bson::Document, results::InsertOneResult, sync::Collection};
use rocket::serde::DeserializeOwned;
use serde::Serialize;

use crate::{
    db::mongo::Crud,
    errors::{message::ERR_NOT_FOUND, HubError},
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
        match collection.find_one(doc! { "_id":  id}, None)? {
            Some(value) => Ok(value),
            None => Err(HubError::new_not_found(ERR_NOT_FOUND.as_ref(), None)),
        }
    }
}
