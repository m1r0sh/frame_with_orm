use actix_web::{App, HttpServer};
use std::io;

mod services;
mod auth;
mod db;
mod models;

use services::greet;
use services::greetest;
use services::send;
use auth::login;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet::greet)
            .service(greetest::greetest)
            .service(send::send)
            .service(login::login)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

