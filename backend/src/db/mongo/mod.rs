pub mod shrimp;
pub mod varys;

use serde::Serialize;
use mongodb::{
    bson::Document,
    results::InsertOneResult,
    sync::Collection,
};

use crate::errors::Errors;

pub trait Crud<'a, T>
where
    T: Serialize,
{
    fn create(collection: Collection<Document>, data: T) -> Result<InsertOneResult, Errors<'a>>;
    fn get_by_id(collection: Collection<T>, id: &str) -> Result<T, Errors<'a>>;
}