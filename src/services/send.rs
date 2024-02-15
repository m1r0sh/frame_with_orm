use actix_web::{
    post,
    HttpResponse,
    Responder,
    HttpRequest,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyRequest {
    checker: String,
    some: i32,
    bool: bool,
}

#[derive(Serialize)]
struct MyResponse {
    mess: String,
    data: Vec<i32>,
    param: MyRequest
}
#[post("/send")]
async fn send(body: actix_web::web::Json<MyRequest>) -> impl Responder {
    let body = body.0;

    let response = MyResponse {
        mess: "Work it".to_string(),
        data: vec![3, 32, 67],
        param: body
    };

    let json = serde_json::json!(response);
    HttpResponse::Ok().json(json)
}