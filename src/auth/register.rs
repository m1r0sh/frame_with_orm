use actix_web::{
    post,
    HttpResponse,
    Responder,
};
use serde::{Deserialize, Serialize};
use bcrypt::hash;

use models::user;
use crate::models;

// надо подключить тут к бд и потом нужно как то импортировать модельки юзерс сюда и req почему не видит пароль ругается
#[derive(Debug, Serialize, Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[post("/login")]
async fn register(req: actix_web::web::Json<RegisterRequest>) -> impl Responder {
    // Хеширование пароля (замените на bcrypt или similar)
    let password_hash = bcrypt::hash(req.password, bcrypt::DEFAULT_COST);

    // Вставка нового пользователя в базу данных
    let user = diesel::insert_into(user::table)
        .values(&(&req.username, &req.email, &password_hash))
        .get_result(conn)?;

    // Ответ сгенерированным ID пользователя
    HttpResponse::Ok().json(&user.id)
}