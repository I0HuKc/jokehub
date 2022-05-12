use crate::errors::Errors;
use mongodb::sync::{Client, Collection};
use rocket::State;

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
                    .database(dotenv!("MONGO_DATABASE_NAME"))
                    .collection("anecdote");

                Ok(collection)
            }
        }
    }
}
