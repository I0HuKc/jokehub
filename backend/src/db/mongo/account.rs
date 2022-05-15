use mongodb::{bson::doc, sync::Collection};

use crate::model::account::User;
use crate::{
    db::mongo::Crud,
    errors::{ErrorChapter, Errors, ErrorsKind, CH_DATABASE},
};

impl<'a> Crud<'a, User> for User {}

impl<'a> User {
    pub fn get_by_username(collection: Collection<User>, username: String) -> Result<User, Errors<'a>> {
        match collection.find_one(doc! { "username":  username.to_string()}, None)? {
            Some(value) => Ok(value),
            None => {
                let kind = ErrorsKind::NotFound(ErrorChapter(CH_DATABASE.clone()));

                Err(Errors::new(kind))
            }
        }
    }
}
