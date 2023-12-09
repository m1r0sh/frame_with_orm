use actix_web::{get, HttpResponse, Responder, };
//HttpRequest
#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}