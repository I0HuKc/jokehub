use mongodb::{bson::doc, sync::Collection};
use rocket::serde::DeserializeOwned;
use serde::Serialize;

use crate::{
    db::mongo::Crud,
    err_internal, err_not_found,
    errors::HubError,
    model::shrimp::{Paws, Query, Shrimp},
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
    pub fn get_random(collection: Collection<Shrimp<T>>, q: Query) -> Result<Shrimp<T>, HubError> {
        let filter = doc! {"_meta-data.author": q.author};
        let update = doc! {"$inc": {"_header.counter": 1}};

        let pipeline = vec![
            // doc! {
            //    // filter on movie title:
            //    "$match": {
            //       "title": "A Star Is Born"
            //    }
            // },
            doc! {              
               "$sample": {
                  "size": 1
               }
            },
        ];

        // match collection.aggregate(pipeline, None) {
        //     Ok(v) => {
        //         let c = v.collect::<HubError>()?;

        //         // println!("{:?}", v.collect::<HubError>())
        //     },

        //     Err(_) => todo!(),
        // };

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
