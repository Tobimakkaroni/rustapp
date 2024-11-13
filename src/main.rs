// cargo build
// cargo run
// ahhh

mod cors;
mod message;
mod routes;

use warp::Filter;
// use tokio::main;

#[tokio::main]
async fn main() {
    let cors = cors::cors();

    let hello = routes::hello_route();

    let routes = hello.with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}