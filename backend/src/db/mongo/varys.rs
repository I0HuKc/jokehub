use mongodb::sync::{Client, Collection};
use rocket::State;

// Заведующий всеми коллекциями
pub enum Varys {
    Users,

    Anecdote,
    // Joke,
    Punch,
    // Story,
}

impl Varys {
    pub fn get<'a, T>(client: &State<Box<Client>>, v: Varys) -> Collection<T> {
        match v {
            Varys::Anecdote => client
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("anecdote"),

            Varys::Punch => client
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("punch"),

            Varys::Users => client
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("users"),
        }
    }
}
