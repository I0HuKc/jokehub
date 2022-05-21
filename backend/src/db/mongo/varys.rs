use mongodb::sync::Collection;

use crate::db::mongo::MongoConn;

// Заведующий всеми коллекциями
pub enum Varys {
    Users,

    Anecdote,
    Joke,
    Punch,
    // Story,
}

impl Varys {
    pub fn get<'a, T>(client: MongoConn, v: Varys) -> Collection<T> {
        match v {
            Varys::Anecdote => client
                .0
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("anecdote"),

            Varys::Punch => client
                .0
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("punch"),

            Varys::Joke => client
                .0
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("joke"),

            Varys::Users => client
                .0
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("users"),
        }
    }
}
