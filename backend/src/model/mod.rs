pub mod joke;

use crate::Errors;
use rocket::http::Status;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::Infallible;
use validator::{Validate, ValidationErrors};
use warp::{Filter, Rejection, Reply};

#[derive(Debug)]
enum Error {
    Validation(ValidationErrors),
}

// fn with_validated_json<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
// where
//     T: DeserializeOwned + Validate + Send,
// {
//     warp::body::content_length_limit(1024 * 16)
//         .and(warp::body::json())
//         .and_then(|value| async move { validate(value).map_err(warp::reject::custom) })
// }

fn validate<T>(value: T) -> Result<T, Error>
where
    T: Validate,
{
    value.validate().map_err(Error::Validation)?;

    Ok(value)
}

// impl warp::reject::Reject for Error {}

// async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
//     let response = if let Some(e) = err.find::<Error>() {
//         handle_crate_error(e)
//     } else {
//         HttpApiProblem::with_title_and_type_from_status(StatusCode::INTERNAL_SERVER_ERROR)
//     };

//     Ok(response.to_hyper_response())
// }

// fn handle_crate_error(e: &Error) -> HttpApiProblem {
//     match e {
//         Error::Validation(errors) => {
//             let mut problem =
//                 HttpApiProblem::with_title_and_type_from_status(StatusCode::BAD_REQUEST)
//                     .set_title("One or more validation errors occurred")
//                     .set_detail("Please refer to the errors property for additional details");

//             let _ = problem.set_value("errors", errors.errors());

//             problem
//         }
//     }
// }
