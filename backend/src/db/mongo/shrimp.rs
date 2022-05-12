use mongodb::{bson::doc, bson::Document, results::InsertOneResult, sync::Collection};
use rocket::serde::DeserializeOwned;
use serde::Serialize;

use crate::db::mongo::Crud;
use crate::{model::shrimp::Shrimp};
use crate::errors::{Errors, ErrorsKind, ErrorChapter, CH_DATABASE};

impl<'a, T> Crud<'a, Shrimp<T>> for Shrimp<T>
where
    T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync,
{
    fn create(collection: Collection<Document>, data: Shrimp<T>) -> Result<InsertOneResult, Errors<'a>> {
        let doc = bson::to_document(&data)?;
        let rersult = collection.insert_one(doc, None)?;

        Ok(rersult)
    }

    fn get_by_id(collection: Collection<Shrimp<T>>, id: &str) -> Result<Shrimp<T>, Errors<'a>> {
        match collection.find_one(doc! { "_id":  id}, None)? {
            Some(value) => Ok(value),
            None => {
                let kind = ErrorsKind::NotFound(ErrorChapter(CH_DATABASE.clone()));
                
                Err(Errors::new(kind))
            },
        }
    }
}
