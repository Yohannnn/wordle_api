use warp::Filter;

use crate::{handlers, models::Answer};

pub fn guess(
    answer: Answer,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("guess")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_answer(answer))
        .and_then(handlers::guess)
        .recover(handlers::guess_rejection)
}

pub fn with_answer(
    answer: Answer,
) -> impl Filter<Extract = (Answer,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || answer.clone())
}
