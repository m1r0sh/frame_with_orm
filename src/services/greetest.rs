use actix_web::{get, HttpResponse, Responder, HttpRequest};

#[get("/{id}")]
async fn greetes(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello, as!")
}