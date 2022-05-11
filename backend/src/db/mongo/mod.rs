pub mod shrimp;

use crate::Errors;
use mongodb::{
    bson::Document,
    results::InsertOneResult,
    sync::{Client, Collection},
};
use rocket::State;
use serde::Serialize;
use std::env;

pub trait Crud<'a, T>
where
    T: Serialize,
{
    fn create(collection: Collection<Document>, data: T) -> Result<InsertOneResult, Errors<'a>>;
    fn get_by_id(collection: Collection<T>, id: &str) -> Result<T, Errors<'a>>;
}

pub fn anecdote_collection<'a, T>(client: &State<Box<Client>>) -> Result<Collection<T>, Errors<'a>> {
    let collection = client
        .database(&env::var("MONGO_DATABASE_NAME")?)
        .collection(&env::var("MONGO_ANECDOTE_COLLECTION")?);

    Ok(collection)
}


pub enum Varys{
    Anecdote
}


impl Varys {
    pub fn get<'a, T>(client: &State<Box<Client>>, v: Varys) -> Result<Collection<T>, Errors<'a>> {
        match v {
            Varys::Anecdote => {
                let collection = client
                .database(&env::var("MONGO_DATABASE_NAME")?)
                .collection(&env::var("MONGO_ANECDOTE_COLLECTION")?);
        
                Ok(collection)
            },
        }
    }    
}
