pub mod account;
pub mod shrimp;
pub mod varys;

use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    results::{DeleteResult, InsertOneResult},
    sync::Client,
    sync::Collection,
};

use serde::Serialize;
use std::time::Duration;

use rocket::{
    outcome::Outcome,
    request::{self, FromRequest},
    serde::DeserializeOwned,
    Request, State,
};

use crate::{err_not_found, errors::HubError};

#[derive(Clone)]
pub struct MongoConn<'a>(pub &'a State<Box<Client>>);

pub fn connect() -> Option<Client> {
    let mut options = ClientOptions::parse(dotenv!("MONGO_DB_URL")).unwrap();

    // Параметры соединения
    let duration: Duration = Duration::new(60, 0);
    options.app_name = Some("Stuffy Krill".to_string());
    options.connect_timeout = Some(duration);

    // Получение дескриптора кластера
    let client: Result<Client, mongodb::error::Error> = Client::with_options(options);
    match client {
        Ok(c) => {
            let ping = c
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .run_command(doc! {"ping": 1}, None)
                .unwrap();
            println!("{}", ping);
            Some(c)
        }
        Err(_) => Option::None,
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MongoConn<'r> {
    type Error = ();

    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<MongoConn<'r>, Self::Error> {
        let outcome = request.guard::<&State<Box<Client>>>().await;
        match outcome {
            Outcome::Success(client) => Outcome::Success(MongoConn(client)),
            Outcome::Failure(_) => todo!(),
            Outcome::Forward(_) => todo!(),
        }
    }
}

pub trait Crud<'a, T>
where
    T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync,
{
    fn create(collection: Collection<Document>, data: T) -> Result<InsertOneResult, HubError> {
        let doc = bson::to_document(&data)?;
        let rersult = collection.insert_one(doc, None)?;

        Ok(rersult)
    }

    // -> Result<Vec<Favorite>, HubError>
    // fn slice(collection: Collection<T>, limit: u8, offset: u8, ff: Document ){
    //     let mut cursor = collection.find(ff, None)?;

    // }

    fn get_by_id(collection: Collection<T>, id: &str) -> Result<T, HubError> {
        match collection.find_one(doc! { "_id":  id}, None)? {
            Some(value) => Ok(value),
            None => Err(err_not_found!(collection.name())),
        }
    }

    fn del_by_id(collection: Collection<T>, id: &str) -> Result<DeleteResult, HubError> {
        let result = collection.delete_one(doc! {"_id": id}, None)?;

        Ok(result)
    }
}
