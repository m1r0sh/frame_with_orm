use actix_web::{get, HttpResponse, Responder, HttpRequest};
use actix_web::web::Path;

#[get("/greetest/{id}")]
async fn greetest(_req: HttpRequest, id: Path<i32>) -> impl Responder {
    let body = format!("Hello, as! Your ID is {} .", id);
    HttpResponse::Ok().body(body)
}