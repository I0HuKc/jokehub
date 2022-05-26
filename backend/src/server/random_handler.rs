use mongodb::bson::doc;
use serde_json::Value;

use crate::{
    db::mongo::{shrimp::aggregation::Qilter, varys::Varys, MongoConn},
    err_not_found,
    errors::HubError,
    model::{
        account::security::TariffGuard,
        anecdote::Anecdote,
        joke::Joke,
        punch::Punch,
        shrimp::{Category, Flag, Shrimp},
    },
};

#[get("/random?<category>&<flag>&<tag>&<author>&<lang>")]
pub fn random<'f>(
    _tariff: TariffGuard,
    client: MongoConn<'f>,
    category: Option<Vec<Category>>,
    flag: Option<Vec<Flag>>,
    tag: Option<Vec<&str>>,
    author: Option<&str>,
    lang: Option<&str>,
) -> Result<Value, HubError> {
    let (mut random_category, mut allowed_category) = Category::random(category, true);
    let qilter = Qilter::new(author, lang, flag, tag);

    loop {
        match random_category.as_ref() {
            Some(Category::Anecdote) => {
                let collection = Varys::get::<Shrimp<Anecdote>>(client.0.as_ref(), Varys::Anecdote);
                let result = Shrimp::<Anecdote>::get_random(&collection, &qilter)?;

                if result.is_none() {
                    (random_category, allowed_category) =
                        Category::random(Some(allowed_category), false);
                } else {
                    let resp = result
                        .as_ref()
                        .unwrap()
                        .inc_counter(&collection)?
                        .tariffing(&_tariff.0, &_tariff.1);

                    return Ok(resp);
                }
            }

            Some(Category::Joke) => {
                let collection = Varys::get::<Shrimp<Joke>>(client.0.as_ref(), Varys::Joke);
                let result = Shrimp::<Joke>::get_random(&collection, &qilter)?;

                if result.is_none() {
                    (random_category, allowed_category) =
                        Category::random(Some(allowed_category), false);
                } else {
                    let resp = result
                        .as_ref()
                        .unwrap()
                        .inc_counter(&collection)?
                        .tariffing(&_tariff.0, &_tariff.1);

                    return Ok(resp);
                }
            }

            Some(Category::Punch) => {
                let collection = Varys::get::<Shrimp<Punch>>(client.0.as_ref(), Varys::Punch);
                let result = Shrimp::<Punch>::get_random(&collection, &qilter)?;

                if result.is_none() {
                    (random_category, allowed_category) =
                        Category::random(Some(allowed_category), false);
                } else {
                    let resp = result
                        .as_ref()
                        .unwrap()
                        .inc_counter(&collection)?
                        .tariffing(&_tariff.0, &_tariff.1);

                    return Ok(resp);
                }
            }

            None => return Err(err_not_found!("record")),
        }
    }
}
