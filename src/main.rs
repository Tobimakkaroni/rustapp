use actix_web::{get, web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

// Set up a simple route for testing
#[get("/")]
async fn hello() -> impl Responder {
    "Hello from Rust and Actix-web!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();  // Load environment variables
    let port = env::var("PORT").unwrap_or("8080".to_string());
    
    HttpServer::new(|| {
        App::new()
            .service(hello)  // Add our test route
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}
