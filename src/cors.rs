// use warp::Filter;

pub fn cors() -> warp::filters::cors::Cors {
    warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["content-type"])
        .build()
}