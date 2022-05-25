use mongodb::{bson::doc, sync::Collection};
use rocket::serde::DeserializeOwned;
use serde::Serialize;

use crate::{
    db::mongo::{shrimp::aggregation::Qilter, Crud},
    err_internal, err_not_found,
    errors::HubError,
    model::shrimp::{Paws, Shrimp},
};

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
        qilter: &Qilter,
    ) -> Result<Option<Shrimp<T>>, HubError> {
        let data = collection
            .aggregate(qilter.pipeline(), None)
            .map_err(|err| err_internal!("Faild to take smaple", err))?
            .next();

        match data {
            Some(result) => Ok(bson::from_document(result?)?),
            None => Ok(None),
        }
    }
}

pub mod aggregation {
    use bson::{doc, Document};

    use crate::model::shrimp::Flag;

    pub struct Qilter<'a> {
        author: Option<&'a str>,
        language: Option<&'a str>,
        flags: Option<Vec<Flag>>,
        tags: Option<Vec<&'a str>>,
    }

    impl<'a> Qilter<'a> {
        pub fn new(
            author: Option<&'a str>,
            language: Option<&'a str>,
            flags: Option<Vec<Flag>>,
            tags: Option<Vec<&'a str>>,
        ) -> Self {
            Qilter {
                author,
                language,
                flags,
                tags,
            }
        }
        pub fn pipeline(&self) -> Vec<Document> {
            let mut pipeline: Vec<Document> = Vec::new();

            self.tags.as_ref().map(|vector| {
                for tag in vector.to_owned() {
                    pipeline.push(doc! {
                        "$match": {
                            "$expr": {
                              "$in": [tag, "$_meta-data.tags"],
                            },
                          }
                    })
                }
            });

            self.author.map(|a| {
                pipeline.push(doc! {
                    "$match": {
                        "_meta-data.author": a
                    }
                })
            });

            self.language.map(|l| {
                pipeline.push(doc! {
                    "$match": {
                        "_meta-data.language": l
                    }
                })
            });

            self.flags.as_ref().map(|vector| {
                let mut d = Document::new();

                for flag in vector.to_owned() {
                    d.insert(
                        format!("_meta-data.flags.{}", flag.to_string().to_ascii_lowercase()),
                        true,
                    );
                }

                pipeline.push(doc! {
                    "$match": d
                })
            });

            pipeline.push(doc! {
              "$sample": {
                "size": 1
              }
            });

            pipeline.push(doc! {
              "$set": {
                "_header.counter": {
                    "$add": ["$_header.counter", 1]
                }
              }
            });

            pipeline
        }
    }
}
