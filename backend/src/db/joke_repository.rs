use diesel::RunQueryDsl;

use crate::{
    db::Conn,
    model::joke::{Joke, NewJoke},
    schema::jokes_tb,
};
use diesel::result::Error;



impl Joke {
    // pub async fn create(conn: Conn, nj: NewJoke) -> Outcome<Joke> {
    //     conn.run(move |c| {
    //         match diesel::insert_into(jokes_tb::table)
    //             .values(nj)
    //             .get_result::<Joke>(c)
    //         {
    //             Ok(j) => Outcome::Ok(j),
    //             Err(diesel::result::Error::DatabaseError(
    //                 diesel::result::DatabaseErrorKind::UniqueViolation,
    //                 _,
    //             )) => Outcome::AlreadyExists(Error::new(ERR_ALREADY_EXISTS.to_string())),
    //             Err(e) => Outcome::Other(Error::from(e)),
    //         }
    //     })
    //     .await
    // }

    pub async fn create(conn: Conn, nj: NewJoke) -> Result<Joke, Error> {
        conn.run(move |c| {
            diesel::insert_into(jokes_tb::table)
                .values(nj)
                .get_result::<Joke>(c)
        })
        .await
    }
}
