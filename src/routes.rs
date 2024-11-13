use warp::Filter;
use crate::message::Message;

pub fn hello_route() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("hello")
        .map(|| warp::reply::json(&Message {
            message: "Geilooo".to_string(),
        }))
}
