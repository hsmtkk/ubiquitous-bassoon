mod handle;
mod model;
mod post_repo;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = get_address();
    let port = get_port();
    let bind_addr = format!("{}:{}", addr, port);
    let repo = post_repo::PostRepository::new();
    HttpServer::new(move || {
        App::new()
            .data(repo.clone())
            .service(handle::handle_get)
            .service(handle::handle_post)
            .service(handle::handle_put)
            .service(handle::handle_delete)
    })
    .bind(bind_addr)?
    .run()
    .await
}

fn get_address() -> String {
    match std::env::var("LISTEN_ADDRESS") {
        Ok(addr) => addr,
        Err(_e) => "0.0.0.0".to_string(),
    }
}

const DEFAULT_PORT: u16 = 8000;

fn get_port() -> u16 {
    match std::env::var("LISTEN_PORT") {
        Ok(port_str) => match port_str.parse::<u16>() {
            Ok(port) => port,
            Err(_e) => DEFAULT_PORT,
        },
        Err(_e) => DEFAULT_PORT,
    }
}
