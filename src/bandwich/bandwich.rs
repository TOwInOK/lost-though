use super::auth::*;
use crate::user::user::User;
use crate::user::user_create;
use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::to_string;
use crate::mongolinks::cget::get_connection_users;
use actix_web::cookie::Cookie;

const SECRET_KEY: &[u8; 10] = b"secret_key";

async fn create_token(auth: web::Data<Auth>) -> impl Responder {
    let collection = get_connection_users().await;

    match user_veryficate(&collection, auth.name.clone(), auth.password.clone()).await {
        Ok(user) => {
            // Создаем токен из структуры пользователя
            match to_string(&user) {
                Ok(user_json) => {
                    // Создаем заголовок токена (алгоритм подписи, тип токена и т.д.)
                    let header = Header::new(Algorithm::HS256);

                    // Подписываем JSON объект, создавая токен
                    match encode(&header, &user_json, &EncodingKey::from_secret(SECRET_KEY)) {
                        Ok(token) => {
                            // Включаем токен в куки
                            let cookie = Cookie::build("JWT", token)
                                .http_only(true)
                                .finish();
                            // Отправляем куки в ответе сервера
                            HttpResponse::Ok()
                                .cookie(cookie)
                                .finish()
                        }
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    }
                }
                Err(_) => HttpResponse::Unauthorized().finish(),
            }
        }
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

async fn register(auth: web::Data<User>) -> impl Responder {
    let collection = get_connection_users().await;
    match user_create(&collection, auth.get_ref().to_owned()).await {
        Ok(_) => {}
        Err(_) => {}
    }
   
    HttpResponse::Ok().json("User registered successfully")
}

// Обработчик для маршрута /authorize
async fn authorize() -> impl Responder {
    // Ваша логика аутентификации пользователя
    // Например, проверка токена, проверка данных пользователя и т.д.
    
    // Возвращаем успешный ответ (например, id аутентифицированного пользователя)
    HttpResponse::Ok().json("User authorized successfully")
}