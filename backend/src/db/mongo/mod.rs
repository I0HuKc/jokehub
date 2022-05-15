pub mod shrimp;
pub mod varys;
pub mod account;

use serde::Serialize;
use rocket::serde::DeserializeOwned;
use mongodb::{
    bson::doc, 
    bson::Document,
    results::InsertOneResult,
    sync::Collection,
};

use crate::errors::{Errors, ErrorsKind, ErrorChapter, CH_DATABASE};

pub trait Crud<'a, T>
where
    T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync,
{
    fn create(collection: Collection<Document>, data: T) -> Result<InsertOneResult, Errors<'a>> {
        let doc = bson::to_document(&data)?;
        let rersult = collection.insert_one(doc, None)?;

        Ok(rersult)
    }

    fn get_by_id(collection: Collection<T>, id: &str) -> Result<T, Errors<'a>> {
        match collection.find_one(doc! { "_id":  id}, None)? {
            Some(value) => Ok(value),
            None => {
                let kind = ErrorsKind::NotFound(ErrorChapter(CH_DATABASE.clone()));
                
                Err(Errors::new(kind))
            },
        }
    }
}