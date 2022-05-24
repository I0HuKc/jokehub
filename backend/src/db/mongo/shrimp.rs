use mongodb::{bson::doc, sync::Collection};
use rocket::serde::DeserializeOwned;
use serde::Serialize;

use crate::{
    db::mongo::Crud,
    err_internal, err_not_found,
    errors::HubError,
    model::shrimp::{filter::Filter, Paws, Shrimp},
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
        filter: Filter,
    ) -> Result<Shrimp<T>, HubError> {
        let filter = filter.gen();
        let update = doc! {"$inc": {"_header.counter": 1}};

        match collection.find_one_and_update(filter.0, update.clone(), None) {
            Ok(result) => {
                if let Some(shrimp) = result {
                    Ok(shrimp)
                } else {
                    match collection.find_one_and_update(filter.1, update, None) {
                        Ok(result) => {
                            if let Some(shrimp) = result {
                                Ok(shrimp)
                            } else {
                                // Коллекция пуста или не содержит записей соответствующих указанным параметрам фильтрации
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
