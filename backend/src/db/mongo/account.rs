use mongodb::{bson::doc, sync::Collection};

use crate::model::account::User;
use crate::{
    db::mongo::Crud,
    errors::HubError,
};

impl<'a> Crud<'a, User> for User {}

impl<'a> User {
    pub fn get_by_username(collection: Collection<User>, username: String) -> Result<User, HubError> {
        match collection.find_one(doc! { "username":  username.to_string()}, None)? {
            Some(value) => Ok(value),
            None => {
                Err(HubError::new_not_found("User is not found.", None))       
            }
        }
    }
}
