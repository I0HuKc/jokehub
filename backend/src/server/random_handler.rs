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
        shrimp::{Category, Flag, Shrimp},
    },
};

#[get("/random?<category>&<flag>&<author>&<lang>")]
pub fn random<'f>(
    _tariff: TariffGuard,
    client: MongoConn<'f>,
    category: Option<Vec<Category>>,
    flag: Option<Vec<Flag>>,
    author: Option<&str>,
    lang: Option<&str>,
) -> Result<Value, HubError> {
    match Category::random(&category) {
        Category::Anecdote => {
            let result = Shrimp::<Anecdote>::get_random(
                Varys::get(client.0.as_ref(), Varys::Anecdote),
                author,
                lang,
                flag,
            )?;

            Ok(result.tariffing(_tariff.0, _tariff.1))
        }
        Category::Joke => {
            let result = Shrimp::<Joke>::get_random(
                Varys::get(client.0.as_ref(), Varys::Joke),
                author,
                lang,
                flag,
            )?;

            Ok(result.tariffing(_tariff.0, _tariff.1))
        }
        Category::Punch => {
            let result = Shrimp::<Punch>::get_random(
                Varys::get(client.0.as_ref(), Varys::Punch),
                author,
                lang,
                flag,
            )?;

            Ok(result.tariffing(_tariff.0, _tariff.1))
        }
    }
}
