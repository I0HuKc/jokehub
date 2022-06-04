use mongodb::bson::doc;
use serde_json::Value;

use crate::{
    db::mongo::{shrimp::aggregation::Qilter, varys::Varys, MongoConn},
    err_not_found,
    errors::HubError,
    model::{
        account::security::ApiKeyGuard,
        account::Tariff,
        anecdote::Anecdote,
        joke::Joke,
        punch::Punch,
        shrimp::{Category, Flag, Shrimp},
    },
};

#[macro_export]
macro_rules! shrimp_reaction_handler {
    ($f:ident, $path:literal, $category:tt) => {
        use crate::model::account::security::ApiKeyGuard;
        use crate::model::shrimp::{Category, ReactionKind};

        #[post($path)]
        pub fn $f<'f>(
            _api_key: ApiKeyGuard,
            client: MongoConn<'f>,
            record_id: &str,
            reaction_kind: ReactionKind,
        ) -> Result<(), HubError> {
            match _api_key.0 {
                Some(_) => Shrimp::<$category>::add_reaction(
                    &Varys::get(client.0.as_ref(), Category::$category.into()),
                    record_id,
                    reaction_kind,
                ),

                None => Err(crate::err_unauthorized!(
                    "Api-Key is not found",
                    "Api-Key must be set in the header with the name `Api-Key`"
                )),
            }
        }
    };
}

#[get("/random?<category>&<flag>&<tag>&<author>&<lang>")]
pub fn random<'f>(
    _api_key: ApiKeyGuard,
    client: MongoConn<'f>,
    category: Option<Vec<Category>>,
    flag: Option<Vec<Flag>>,
    tag: Option<Vec<&str>>,
    author: Option<&str>,
    lang: Option<&str>,
) -> Result<Value, HubError> {
    let (mut random_category, mut allowed_category) = Category::random(category, true);
    let qilter = Qilter::new(author, lang, flag, tag);
    let tariff: Tariff = match _api_key.0 {
        Some(data) => data.get_tariff(),
        None => Tariff::default(),
    };

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
                        .tariffing(&tariff, &None);

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
                        .tariffing(&tariff, &None);

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
                        .tariffing(&tariff, &None);

                    return Ok(resp);
                }
            }

            None => return Err(err_not_found!("record")),
        }
    }
}
