use mongodb::bson::doc;
use serde_json::Value;

use crate::{
    db::mongo::{varys::Varys, MongoConn},
    errors::HubError,
    model::{
        account::security::TariffGuard,
        anecdote::Anecdote,
        joke::Joke,
        punch::Punch,
        shrimp::{Category, Query, Shrimp},
    },
};

#[get("/random?<category>&<uniq>&<author>&<lang>")]
pub fn random<'f>(
    _tariff: TariffGuard,
    client: MongoConn<'f>,
    category: Option<Vec<Category>>,
    uniq: Option<bool>,
    author: Option<&str>,
    lang: Option<&str>,
) -> Result<Value, HubError> {
    let q = Query { author, uniq, lang };

    match Category::random(&category) {
        Category::Anecdote => {
            let result = Shrimp::<Anecdote>::get_random(Varys::get(client, Varys::Anecdote), q)?;

            Ok(result.tariffing(_tariff.0, _tariff.1))
        }
        Category::Joke => {
            let result = Shrimp::<Joke>::get_random(Varys::get(client, Varys::Joke), q)?;

            Ok(result.tariffing(_tariff.0, _tariff.1))
        }
        Category::Punch => {
            let result = Shrimp::<Punch>::get_random(Varys::get(client, Varys::Punch), q)?;

            Ok(result.tariffing(_tariff.0, _tariff.1))
        }
    }
}
