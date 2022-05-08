use diesel::RunQueryDsl;

use crate::{
    db::Conn,
    model::joke::{Joke, NewJoke},
    schema::jokes_tb,
    Error, 
};

impl Joke {
    pub async fn create(conn: Conn, nj: NewJoke) -> Result<Joke, Error> {
        let joke = conn
            .run(move |c| {
                diesel::insert_into(jokes_tb::table)
                    .values(nj)
                    .get_result::<Joke>(c)
            })
            .await?;

        Ok(joke)
    }
}
