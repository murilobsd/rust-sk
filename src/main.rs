use rust_sk::startup;

use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind 127.0.0.1:8080");
    startup::run(listener)?.await
}
