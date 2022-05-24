use mongodb::sync::Client;
use mongodb::sync::Collection;

// Заведующий всеми коллекциями
pub enum Varys {
    Users,
    Sessions,

    Anecdote,
    Joke,
    Punch,
    // Story,
}

impl Varys {
    pub fn get<'a, T>(client: &Client, v: Varys) -> Collection<T> {
        match v {
            Varys::Anecdote => client
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("anecdote"),

            Varys::Punch => client
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("punch"),

            Varys::Joke => client
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("joke"),

            Varys::Users => client
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("users"),

            Varys::Sessions => client
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .collection("sessions"),
        }
    }
}
