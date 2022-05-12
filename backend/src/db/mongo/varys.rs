use crate::errors::Errors;
use mongodb::sync::{Client, Collection};
use rocket::State;
use std::env;

// Заведующий всеми коллекциями
pub enum Varys {
    Anecdote,
    // Joke,
    // Punch,
    // Story,
}

impl Varys {
    pub fn get<'a, T>(client: &State<Box<Client>>, v: Varys) -> Result<Collection<T>, Errors<'a>> {
        match v {
            Varys::Anecdote => {
                let collection = client
                    .database(&env::var("MONGO_DATABASE_NAME")?)
                    .collection(&env::var("MONGO_ANECDOTE_COLLECTION")?);

                Ok(collection)
            }
        }
    }
}
