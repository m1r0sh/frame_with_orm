use actix_web::{App, HttpServer};
use std::io;

mod services;
use services::greet;
use services::greetest;
use services::send;


#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet::greet)
            .service(greetest::greetest)
            .service(send::send)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

