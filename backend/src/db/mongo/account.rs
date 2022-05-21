use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{bson::doc, sync::Collection};

use crate::model::account::User;
use crate::{db::mongo::Crud, errors::HubError};

impl<'a> Crud<'a, User> for User {}

impl<'a> User {
    pub fn get_by_username(
        collection: Collection<User>,
        username: String,
    ) -> Result<User, HubError> {
        match collection.find_one(doc! { "username":  username}, None)? {
            Some(value) => Ok(value),
            None => Err(HubError::new_not_found("User is not found.", None)),
        }
    }

    pub fn del_by_username(
        collection: Collection<User>,
        username: String,
    ) -> Result<DeleteResult, HubError> {
        let res = collection.delete_one(doc! { "username":  username}, None)?;

        Ok(res)
    }

    pub fn privilege_set(
        collection: Collection<User>,
        username: &str,
        level: &str,
    ) -> Result<UpdateResult, HubError> {
        let filter = doc! {"username": username};
        let update = doc! {"$set": {"level": level}};

        let res = collection.update_one(filter, update, None)?;

        Ok(res)
    }
}
