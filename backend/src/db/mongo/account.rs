use bson::Document;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{bson::doc, sync::Collection};

use crate::model::account::{security::Session, User};
use crate::{
    db::mongo::{varys::Varys, Crud},
    err_unauthorized,
    errors::HubError,
};

use super::MongoConn;

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

impl Session {
    pub fn set<'f>(&self, client: MongoConn<'f>) -> Result<InsertOneResult, HubError> {
        let collection: Collection<Document> = Varys::get(client, Varys::Sessions);
        let doc = bson::to_document(&self)?;
        let rersult = collection.insert_one(doc, None)?;

        Ok(rersult)
    }

    pub fn check<'f>(token: &str, client: MongoConn<'f>) -> Result<Session, HubError> {
        let collection: Collection<Session> = Varys::get(client, Varys::Sessions);
        match collection.find_one(doc! { "token":  token}, None)? {
            Some(value) => Ok(value),
            None => Err(err_unauthorized!("Session is not found")),
        }
    }

    pub fn drop<'f>(token: &str, client: MongoConn<'f>) -> Result<DeleteResult, HubError> {
        let collection: Collection<Session> = Varys::get(client, Varys::Sessions);
        match collection.delete_one(doc! { "token":  token}, None) {
            Ok(dr) => Ok(dr),
            Err(err) => Err(err_unauthorized!("Falid to drop token", err)),
        }
    }
}
