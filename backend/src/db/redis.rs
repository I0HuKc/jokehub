use std::ops::{Deref, DerefMut};

use r2d2::PooledConnection;
use r2d2_redis::RedisConnectionManager;

use rocket::request::{self, FromRequest, Request};
use rocket::{outcome::Outcome, State};

type Pool = r2d2::Pool<RedisConnectionManager>;
type PooledConn = PooledConnection<RedisConnectionManager>;

pub struct RedisConn(pub PooledConn);

impl Deref for RedisConn {
    type Target = PooledConn;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RedisConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub(crate) fn connect() -> Box<Pool> {   
    let manager = RedisConnectionManager::new(dotenv!("REDIS_DB_URL")).expect("connection manager");
    //let manager = RedisConnectionManager::new(format!("redis://user:{}@{}:{}/{}", redis_password redis_address, redis_port, redis_db)).expect("connection manager");

    match r2d2::Pool::builder().max_size(15).build(manager) {
        Ok(pool) => Box::new(pool),
        Err(e) => panic!("Error: failed to create redis database pool {}", e),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RedisConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<RedisConn, Self::Error> {
        let outcome = request.guard::<&State<Box<Pool>>>().await;
        match outcome {
            Outcome::Success(db) => {
                let conn = db.get().expect("redis connection manager");
                Outcome::Success(RedisConn(conn))
            },

            Outcome::Failure(_) => todo!(),
            Outcome::Forward(_) => todo!(),
        }
    }
}
