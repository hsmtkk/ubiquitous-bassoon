mod handle;
mod model;
mod post_repo;

use actix_web::{App, HttpServer};
use r2d2::Pool;
use redis::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen_address = get_listen_address();
    let redis_host = get_redis_host();
    let client = Client::open(format!("redis://{}", redis_host)).unwrap();
    let pool = Pool::builder().build(client).unwrap();
    let repo = post_repo::PostRepository::new(pool);

    HttpServer::new(move || {
        App::new()
            .data(repo.clone())
            .service(handle::handle_get)
            .service(handle::handle_post)
            .service(handle::handle_put)
            .service(handle::handle_delete)
    })
    .bind(listen_address)?
    .run()
    .await
}

fn get_listen_address() -> String {
    match std::env::var("LISTEN_ADDRESS") {
        Ok(addr) => addr,
        Err(_e) => "0.0.0.0:8000".to_string(),
    }
}

fn get_redis_host() -> String {
    std::env::var("REDIS_HOST").expect("environment variable REDIS_HOST must be defined")
}
