use std::str::FromStr;

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use back::autentifications::auth::Auth;
use back::mongolinks::cget::{get_connection_posts, get_connection_users};
use back::posts::post::{Post, PostCreate};
use back::posts::*;
use back::user::user::{User, UserMin};
use back::user::*;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

//а почему нет
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
//создание пользователя
#[post("/create")]
pub async fn create_user(u: web::Json<User>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_create(&collection, u.into_inner()).await {
        Ok(_) => {
            println!("User created");
            HttpResponse::Created().body("User created")
        }
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}
//инфа о пользователе
#[get("/{name}")]
pub async fn user(name: web::Path<String>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_get(&collection, name.to_string()).await {
        Ok(Some(user)) => {
            let anonymus_user = UserMin {
                name: user.name.clone(),
                role: user.role.clone(),
            };
            println!("{:?}", &user);
            HttpResponse::Ok().json(anonymus_user)
        }
        //HttpResponse::Ok().json(user),
        Ok(None) => {
            println!("User not found");
            HttpResponse::NotFound().body("User not found")
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
//получение инфы о пользователе - вся котороя может быть в User
//нужен пароль
//Поменять чтобы принимала UserMin
#[get("/{name}/settings")]
pub async fn get_user_settings(name: web::Path<String>, u: web::Json<User>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_get(&collection, name.to_string()).await {
        Ok(Some(i)) => {
            if i.validate(&u.into_inner()) {
                HttpResponse::Ok().json(i)
            } else {
                HttpResponse::BadRequest().body("Wrong password")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
//Меняем пароль отправляя пользователя
//нужен пароль
//Функция не логичка.
#[post("/{name}/changepass")]
pub async fn user_changer(name: web::Data<String>, u: web::Json<User>) -> HttpResponse {
    let password = &u.password;
    let collection = get_connection_users().await;
    if password.is_empty() {
        return HttpResponse::BadRequest().body("Password is empty");
    }
    match user_get(&collection, name.to_string()).await {
        Ok(Some(i)) => {
            if i.validate(&u) {
                match user_change(&collection, u.into_inner()).await {
                    Ok(_) => HttpResponse::Ok().body("Password changed"),
                    Err(e) => HttpResponse::BadRequest().body(e.to_string()),
                }
            } else {
                HttpResponse::BadRequest().body("Wrong password")
            }
        }
        Ok(None) => HttpResponse::BadRequest().body("Wrong password"),
        Err(_e) => HttpResponse::BadRequest().body("Wrong password"),
    }
}
//выдаём все посты пользователя
#[get("/{name}/posts")]
pub async fn postall(name: web::Path<String>) -> HttpResponse {
    let collection = get_connection_posts().await;
    match post_getall(&collection, name.to_string()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
//удаляем пользоателя
//Конечно забавно что пользователь может удалять себя
//Но даже если структура по имени правильная, то пользователя не удалишь.
//Из минусов удаляться все посты.
//так что менять с проверкой.
#[delete("/{name}/delete")]
pub async fn delete_user(u: web::Json<User>) -> HttpResponse {
    let collection = get_connection_users().await;
    let collection2 = get_connection_posts().await;
    match user_delete(&collection, &collection2, u.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
//удаляем пост
#[delete("/{post}/delete")]
pub async fn post_deleter(p: web::Json<RequsetDataDelete>) -> HttpResponse {
    let collection = get_connection_posts().await;
    let post_id = p.0.id;
    let auth = p.0.auth;
    if auth.validate().await {
        match post_delete(&collection, post_id).await {
            Ok(_v) => HttpResponse::Ok().body("Post deleted"),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    } else {
        HttpResponse::BadRequest().body("Wrong password")
    }
}

//выдача поста по id
#[get("/{post_id}")]
pub async fn post(post_id: web::Path<String>) -> HttpResponse {
    let collection = get_connection_posts().await;
    let post_id = match ObjectId::from_str(&post_id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId"),
    };
    match post_get(&collection, post_id).await {
        Ok(None) => HttpResponse::NotFound().body("Post not found"), // Вернуть 404, если пост не найден
        Ok(Some(v)) => HttpResponse::Ok().json(v), // Вернуть данные поста, если он найден
        Err(e) => HttpResponse::BadRequest().body(e.to_string()), // Вернуть ошибку, если возникла проблема
    }
}
//Редактируем пост отправляя запрос.
//Заменть на сегментарное редактирование.
#[post("/{post}/edit")]
pub async fn post_editor(p: web::Json<RequsetDataDefault>) -> HttpResponse {
    let collection = get_connection_posts().await;
    let local_post = p.0.post;
    let auth = p.0.auth;
    if auth.validate().await {
        match post_edit(&collection, local_post, auth.name.to_string()).await {
            Ok(_v) => HttpResponse::Ok().body("Post edited"),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    } else {
        HttpResponse::BadRequest().body("Wrong password")
    }
}

#[post("/create")]
pub async fn create(p: web::Json<RequsetData>) -> HttpResponse {
    let collection = get_connection_posts().await;
    let local_post = p.0.post;
    let auth = p.0.auth;
    println!("{:?}", &local_post);
    if auth.validate().await {
        match post_create(&collection, local_post).await {
            Ok(v) => HttpResponse::Ok().json(v),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    } else {
        HttpResponse::BadRequest().body("Wrong password")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequsetData {
    pub post: PostCreate,
    pub auth: Auth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequsetDataDefault {
    pub post: Post,
    pub auth: Auth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequsetDataDelete {
    pub id: String,
    pub auth: Auth,
}
