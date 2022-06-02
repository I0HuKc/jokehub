use bson::Document;
use mongodb::bson::DateTime as MongoDateTime;
use mongodb::results::InsertOneResult;
use mongodb::sync::Client;
use mongodb::{bson::doc, sync::Collection};

use crate::model::account::favorites::Favorite;
use crate::model::account::notification::Notification;
use crate::model::account::{security::api_key::ApiKey, security::Session, User};
use crate::model::account::{Tariff, Theme};
use crate::{
    db::mongo::{varys::Varys, Crud},
    err_internal, err_not_found, err_unauthorized,
    errors::HubError,
    macro_crud,
};

macro_crud!(User);
impl<'a> User {
    pub fn get_by_username(client: &Client, username: String) -> Result<User, HubError> {
        let collection: Collection<User> = Varys::get(client, Varys::Users);
        match collection.find_one(doc! { "username":  username}, None)? {
            Some(value) => Ok(value),
            None => Err(err_not_found!("user")),
        }
    }

    pub fn del_by_username(client: &Client, username: String) -> Result<(), HubError> {
        let collection: Collection<User> = Varys::get(client, Varys::Users);

        match collection.delete_one(doc! { "username":  username}, None) {
            Ok(dr) if dr.deleted_count > 0 => Ok(()),
            Ok(_) => Err(err_not_found!("user")),
            Err(err) => Err(err_internal!("Faild to delete account", err.to_string())),
        }
    }

    pub fn update_password(
        client: &Client,
        username: String,
        new_password_hash: String,
    ) -> Result<(), HubError> {
        let collection: Collection<User> = Varys::get(client, Varys::Users);

        let filter = doc! {"username": username};
        let update = doc! {"$set": {"hash": new_password_hash, "updated_at": MongoDateTime::now()}};

        match collection.update_one(filter, update, None) {
            Ok(ur) if ur.modified_count > 0 => Ok(()),
            Ok(_) => Err(err_internal!(
                "Faild to update password",
                "User was not found"
            )),
            Err(err) => Err(err_internal!("Faild to update password", err.to_string())),
        }
    }

    pub fn privilege_set(
        collection: Collection<User>,
        username: &str,
        level: &str,
    ) -> Result<(), HubError> {
        let filter = doc! {"username": username};
        let update = doc! {"$set": {"level": level, "updated_at": MongoDateTime::now()}};

        match collection.update_one(filter, update, None) {
            Ok(ur) if ur.modified_count > 0 => Ok(()),
            Ok(_) => Err(err_not_found!("user")),
            Err(err) => Err(err_internal!("Faild to update user level", err.to_string())),
        }
    }

    pub fn change_theme(client: &Client, theme: Theme, username: &str) -> Result<(), HubError> {
        let collection: Collection<User> = Varys::get(client, Varys::Users);

        let filter = doc! {"username": username};
        let update = doc! {"$set": {"theme": theme.to_string().to_lowercase(), "updated_at": MongoDateTime::now()}};

        match collection.update_one(filter, update, None) {
            Ok(ur) if ur.modified_count > 0 => Ok(()),
            Ok(_) => Err(err_unauthorized!("Faild to find such user")),
            Err(err) => Err(err_internal!("Faild to change theme", err.to_string())),
        }
    }
}

macro_crud!(ApiKey);
impl ApiKey {
    pub fn roll(client: &Client, owner: &str) -> Result<Vec<ApiKey>, HubError> {
        let collection: Collection<ApiKey> = Varys::get(client, Varys::ApiKeys);
        let mut cursor = collection.find(doc! {"owner": owner}, None)?;
        let mut result: Vec<ApiKey> = Vec::new();

        while let Some(doc) = cursor.next() {
            result.push(doc?);
        }

        Ok(result)
    }

    pub fn get_by_key(client: &Client, key: &str) -> Result<ApiKey, HubError> {
        let collection: Collection<ApiKey> = Varys::get(client, Varys::ApiKeys);
        let filter = doc! {"key": key};
        let update = doc! {"$inc": {"nonce": 1}};

        match collection.find_one_and_update(filter, update, None) {
            Ok(Some(data)) => Ok(data),
            Ok(None) => Err(err_not_found!("api key")),
            Err(err) => Err(err_internal!("Faiild to get api key", err.to_string())),
        }
    }

    pub fn update_tariff(
        client: &Client,
        owner: &str,
        new_tariff: Tariff,
    ) -> Result<ApiKey, HubError> {
        let collection: Collection<ApiKey> = Varys::get(client, Varys::ApiKeys);
        let filter = doc! {"owner": owner};
        let update = doc! {"$set": {
            "tariff": new_tariff.to_string().to_lowercase()
        }};

        match collection.update_one(filter.clone(), update, None) {
            Ok(ur) if ur.modified_count > 0 => {
                let data = collection.find_one(filter, None)?.unwrap();

                Ok(data)
            }
            Ok(_) => Err(err_not_found!("api key")),
            Err(err) => Err(err_internal!(
                "Faild to update api key tariff",
                err.to_string()
            )),
        }
    }

    pub fn del(client: &Client, key: &str, owner: &str) -> Result<(), HubError> {
        let collection: Collection<ApiKey> = Varys::get(client, Varys::ApiKeys);
        let filter = doc! {"owner": owner, "key": key};

        match collection.delete_one(filter, None) {
            Ok(dr) if dr.deleted_count > 0 => Ok(()),
            Ok(_) => Err(err_not_found!("api key")),
            Err(err) => Err(err_internal!("Faild to delete api key", err.to_string())),
        }
    }
}

impl Session {
    pub fn set<'f>(&self, client: &Client) -> Result<InsertOneResult, HubError> {
        let collection: Collection<Document> = Varys::get(client, Varys::Sessions);
        let doc = bson::to_document(&self)?;
        let rersult = collection.insert_one(doc, None)?;

        Ok(rersult)
    }

    pub fn check<'f>(token: &'f str, client: &'f Client) -> Result<Session, HubError> {
        let collection: Collection<Session> = Varys::get(client, Varys::Sessions);
        match collection.find_one(doc! { "token":  token}, None)? {
            Some(value) => Ok(value),
            None => Err(err_unauthorized!("Session is not found")),
        }
    }

    pub fn roll<'f>(username: &'f str, client: &'f Client) -> Result<Vec<Session>, HubError> {
        let collection: Collection<Session> = Varys::get(client, Varys::Sessions);

        let mut cursor = collection.find(doc! {"username": username}, None)?;
        let mut result: Vec<Session> = Vec::new();

        while let Some(doc) = cursor.next() {
            result.push(doc?);
        }

        Ok(result)
    }

    pub fn drop<'f>(token: &str, client: &Client) -> Result<(), HubError> {
        let collection: Collection<Session> = Varys::get(client, Varys::Sessions);
        match collection.delete_one(doc! { "token":  token}, None) {
            Ok(dr) if dr.deleted_count > 0 => Ok(()),
            Ok(_) => Err(err_unauthorized!("Token is not found")),
            Err(err) => Err(err_unauthorized!("Falid to drop token", err)),
        }
    }

    pub fn drop_all<'f>(username: &'f str, client: &Client) -> Result<(), HubError> {
        let collection: Collection<Session> = Varys::get(client, Varys::Sessions);
        match collection.delete_many(doc! { "username":  username}, None) {
            Ok(dr) if dr.deleted_count > 0 => Ok(()),
            Ok(_) => Err(err_unauthorized!("Sessions not found")),
            Err(err) => Err(err_internal!("Falid to drop sessions", err)),
        }
    }
}

macro_crud!(Notification);
impl Notification {}

macro_crud!(Favorite);
impl Favorite {
    pub fn del_by_record_id(client: &Client, record_id: &str) -> Result<(), HubError> {
        let collection: Collection<Favorite> = Varys::get(client, Varys::Favorite);

        match collection.delete_one(doc! {"content_id" : record_id}, None) {
            Ok(dr) if dr.deleted_count > 0 => Ok(()),
            Ok(_) => Err(err_not_found!("favorite")),
            Err(err) => Err(err_internal!("Falid to remove from favorite", err)),
        }
    }
}
