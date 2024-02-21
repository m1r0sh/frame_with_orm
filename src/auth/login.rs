use actix_web::{
    post,
    HttpResponse,
    Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ClientRequest {
    name: String,
    email: String,
    password: i32
}

#[post("/login")]
async fn login(req: actix_web::web::Json<ClientRequest>) -> impl Responder {
    // let token = create_token(&req);
    // let req = req.0;

    let json = serde_json::json!(req);
    HttpResponse::Ok().json(json)
}