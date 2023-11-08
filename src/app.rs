//register
//create - требуется логин и пароль для создания. (можем снимать токены)
//user - информация о пользователе. Если же пароль не совпадает или же его нет отправляем просто логин
//post - информация о конкретном посте
//postall - все посты пользователя
//indexpost - все посты в базе данных
//delete - удалить пост
//edit - редактировать пост

use actix_web::{post, web, get, HttpResponse, Responder, web::Json, delete};
use back::autentifications::autentifications::Auth;
use back::posts::post::Post;
use back::user::user::{User, UserMin};
use back::user::*;
use back::posts::*;
use back::mongolinks::cget::{get_connection_users, get_connection_posts};
use mongodb::bson::oid::ObjectId;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/create")]
pub async fn create_user(u: web::Json<User>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_create(&collection, u.into_inner()).await {
        Ok(_) => HttpResponse::Created().body("User created"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
//инфа о пользователе
#[get("/{name}")]
pub async fn user(name: web::Path<String>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_get(&collection, name.to_string()).await {
        Ok(Some(user)) => {
            let anonymus_user = UserMin {
                name: user.name,
                role: user.role
            };
            HttpResponse::Ok().json(anonymus_user)
        },
        //HttpResponse::Ok().json(user),    
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
//получение инфы о пользователе - email
//нужен пароль
#[get("/{name}/settings")]
pub async fn get_user_settings(name: web::Path<String>, u: web::Json<User>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_get(&collection, name.to_string()).await {
        Ok(Some(i)) => {
            if i.validate(&u.into_inner()) {
                HttpResponse::Ok().json(i)
            }
            else {
                HttpResponse::BadRequest().body("Wrong password")
            }
        },
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

//нужен пароль
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
            }
            else {
                HttpResponse::BadRequest().body("Wrong password")
            }
        },
        Ok(None) => HttpResponse::BadRequest().body("Wrong password"), 
        Err(e) => HttpResponse::BadRequest().body("Wrong password"), 
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

#[delete("/{name}/delete")]
pub async fn delete_user(u: web::Json<User>) -> HttpResponse {
    let collection = get_connection_users().await;
    let collection2 = get_connection_posts().await;
    match user_delete(&collection, &collection2, u.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[delete("/{post}/delete")]
pub async fn post_deleter(post_id: web::Path<String>, auth: web::Json<Auth>) -> HttpResponse {
   let collection = get_connection_posts().await;
   if auth.validate().await {
    match post_delete(&collection, post_id.into_inner()).await {
        Ok(v) => HttpResponse::Ok().body("Post deleted"),
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
    match post_get(&collection, post_id.into_inner()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
#[post("/{post}/edit")]
pub async fn post_editor(local_post: web::Json<Post>, auth: web::Json<Auth>) -> HttpResponse {
    let collection = get_connection_posts().await;
    if auth.validate().await {
        match post_edit(&collection, local_post.into_inner(), auth.name.to_string()).await {
            Ok(v) => HttpResponse::Ok().body("Post edited"),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    } else {
        HttpResponse::BadRequest().body("Wrong password")
    }
}

