use crate::db::PgConn;
use crate::model::joke::{Joke, NewJoke};
use crate::schema::jokes_tb;

// impl Joke {
//     pub fn cresate(conn: PgConn, nj: NewJoke) -> Self {
//         let mut joke = Joke::from(nj);

//         // conn
//         // .run(move |c| {
//         //     diesel::insert_into(jokes_tb::table)
//         //         .values(&joke)
//         //         .get_result(c)
//         // });


//         // let joke = diesel::insert_into(jokes_tb::table)
//         //     .values(joke)
//         //     .get_result::<Joke>(&conn)?;
//     }
// }
