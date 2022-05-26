use mongodb::sync::Client;
use mongodb::sync::Collection;

use crate::model::shrimp::Category;

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

impl From<Category> for Varys {
    fn from(category: Category) -> Self {
        match category {
            Category::Anecdote => Varys::Anecdote,
            Category::Joke => Varys::Joke,
            Category::Punch => Varys::Punch,
        }
    }
}
