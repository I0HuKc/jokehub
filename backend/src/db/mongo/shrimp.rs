use bson::Document;
use mongodb::{bson::doc, sync::Collection};
use rand::Rng;
use rocket::serde::DeserializeOwned;
use serde::Serialize;

use crate::{
    db::mongo::Crud,
    err_internal, err_not_found,
    errors::HubError,
    model::shrimp::{Flag, Paws, Shrimp},
};

macro_rules! macro_filter {
    ($co:literal, $ri:expr, $( ($k:literal, $v:expr)), *) => {
        {
            let mut filter = doc! {"_header.rfd": {$co: $ri}};
            $(
                if $v.is_some() {
                    filter.insert($k, $v);
                }
            )*

            filter
        }
    };
}

impl<'a, T> Crud<'a, Shrimp<T>> for Shrimp<T>
where
    T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync,
    T: Paws,
{
    fn get_by_id(collection: Collection<Shrimp<T>>, id: &str) -> Result<Shrimp<T>, HubError> {
        let filter = doc! {"_id": id};
        let update = doc! {"$inc": {"_header.counter": 1}};

        match collection.find_one_and_update(filter, update, None) {
            Ok(result) => {
                if let Some(shrimp) = result {
                    Ok(shrimp)
                } else {
                    Err(err_not_found!(collection.name()))
                }
            }

            Err(err) => Err(err_internal!(err.to_string())),
        }
    }
}

impl<T> Shrimp<T>
where
    T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync,
    T: Paws,
{
    pub fn get_random(
        collection: Collection<Shrimp<T>>,
        author: Option<&str>,
        lang: Option<&str>,
        flags: Option<Vec<Flag>>,
    ) -> Result<Shrimp<T>, HubError> {
        let r = rand::thread_rng().gen::<u32>();
        let update = doc! {"$inc": {"_header.counter": 1}};
        let filter = macro_filter!(
            "$gt",
            r,
            ("_meta-data.language", lang),
            ("_meta-data.author", author)
        );

        match collection.find_one_and_update(add_flags(filter, flags.clone()), update.clone(), None)
        {
            Ok(result) => {
                if let Some(shrimp) = result {
                    Ok(shrimp)
                } else {
                    let filter = macro_filter!(
                        "$lte",
                        r,
                        ("_meta-data.language", lang),
                        ("_meta-data.author", author)
                    );

                    match collection.find_one_and_update(add_flags(filter, flags), update, None) {
                        Ok(result) => {
                            if let Some(shrimp) = result {
                                Ok(shrimp)
                            } else {
                                let error = err_not_found!("records");

                                Err(error)
                            }
                        }

                        Err(err) => Err(err_internal!(err.to_string())),
                    }
                }
            }

            Err(err) => Err(err_internal!(err.to_string())),
        }
    }
}

fn add_flags(mut base: Document, flags: Option<Vec<Flag>>) -> Document {
    if flags.is_some() {
        for f in flags.unwrap().iter() {
            base.insert(format!("_meta-data.flags.{}", f.to_string().to_lowercase()), true);
        }

        base.clone()
    } else {
        base.clone()
    }
}
