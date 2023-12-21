use actix_web::{put, delete, get, post, web, HttpRequest, HttpResponse, Responder};
use back::{sendcode::email::send_password_code, autentifications::auth::Auth, comments::{comment_delete, comment::Comment, comment_add}, mongolinks::cget::{get_connection_posts, get_connection_users}, posts::{post::PostCreate, *}, users::{user::{User, UserMin}, *}};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use actix_files as fs;

const INDEX_HTML: &str = "static/about.html";
///Main doc page
#[get("/")]
async fn index() -> impl Responder {
    fs::NamedFile::open_async(INDEX_HTML).await
}
///Doc page
#[get("/{path:.*\\.(html|css|js)}")]
async fn indexx(path: web::Path<String>) -> actix_web::Result<fs::NamedFile> {
    let path = format!("static/{}", path);
    Ok(fs::NamedFile::open(path)?)
}
///создание пользователя | crate the user by `<User>`
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
///инфа о пользователе | get info by name of user
#[get("/{name}")]
pub async fn user(name: web::Path<String>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_get(collection, name.to_string()).await {
        Ok(Some(user)) => {
            let anonymus_user = UserMin {
                name: user.name.clone(),
                role: user.role.clone(),
            };
            println!("{:?}", &user);
            HttpResponse::Ok().json(anonymus_user)
        }
        Ok(None) => {
            println!("User not found");
            HttpResponse::NotFound().body("User not found")
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
///Get all user info by name + Auth
///Also you can auth throw this path
#[get("/{name}/settings")]
pub async fn get_user_settings(name: web::Path<String>, u: web::Json<Auth>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_get(collection, name.to_string()).await {
        Ok(Some(i)) => {
            if i.validate_anonimus(&u) {
                HttpResponse::Ok().json(i)
            } else {
                HttpResponse::Forbidden().body("Wrong password")
            }
        }
        Ok(None) => HttpResponse::NoContent().body(""),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
///Change pass
/// req: `<String>` user name + `UserChanger`
#[put("/{name}/changepass")]
pub async fn user_changer(name: web::Path<String>, mut u: web::Json<UserChanger>) -> HttpResponse {
    let collection = get_connection_users().await;
    if u.user.password.is_empty() {
        return HttpResponse::BadRequest().body("Password is empty");
    }
    if u.newpassword.is_empty() {
        return HttpResponse::BadRequest().body("New Password is empty");
    }
    match user_get(collection.clone(), name.to_string()).await {
        Ok(Some(i)) => {
            if i.validate(&u.user) {
                u.user.password = u.newpassword.clone();
                match user_change(&collection, u.user.clone()).await {
                    Ok(_) => HttpResponse::Ok().body("Password changed"),
                    Err(e) => HttpResponse::BadRequest().body(e.to_string()),
                }
            } else {
                HttpResponse::Forbidden().body("Wrong password")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Nothing of your user found"),
        Err(_e) => HttpResponse::BadRequest().body("This structure is't user"),
    }
}
///выдаём все посты пользователя
#[get("/{name}/posts")]
pub async fn postall(name: web::Path<String>) -> HttpResponse {
    let collection = get_connection_posts().await;
    match post_getall(&collection, name.to_string()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
///удаляем пользоателя
///Конечно забавно что пользователь может удалять себя
#[delete("/delete")]
pub async fn delete_user(u: web::Json<Auth>) -> HttpResponse {
    let collection = get_connection_users().await;
    let collection2 = get_connection_posts().await;
    if u.validate().await {
        match user_delete(&collection, &collection2, u.0).await {
            Ok(_) => HttpResponse::Ok().body("User deleted"),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    } else {
        HttpResponse::Forbidden().body("Wrong user data for auth")
    }
}

///выдача поста по id
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
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()), // Вернуть ошибку, если возникла проблема
    }
}
///Редактируем пост отправляя запрос.
///Заменть на сегментарное редактирование.
#[post("/{post}/edit")]
pub async fn post_editor(p: web::Json<RequsetPost>) -> HttpResponse {
    let collection = get_connection_posts().await;
    if p.0.auth.validate().await {
        match post_edit(&collection, p.0.post, p.0.auth.name.to_string()).await {
            Ok(_v) => HttpResponse::Ok().body("Post edited"),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    } else {
        HttpResponse::Forbidden().body("Wrong user data for auth")
    }
}

///Создаём пост принимая RequsetPost
#[post("/create")]
pub async fn create(p: web::Json<RequsetPost>) -> HttpResponse {
    let collection = get_connection_posts().await;
    if p.0.auth.validate().await {
        match post_create(&collection, p.0.post).await {
            Ok(v) => HttpResponse::Ok().json(v),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    } else {
        HttpResponse::Forbidden().body("Wrong user data for auth")
    }
}

///удаляем пост
#[delete("/{post}/delete")]
pub async fn post_deleter(id: web::Path<String>, p: web::Json<Auth>) -> HttpResponse {
    let collection = get_connection_posts().await;
    if p.0.validate().await {
        match post_delete(&collection, id.to_string()).await {
            Ok(_v) => HttpResponse::Ok().body("Post deleted"),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    } else {
        HttpResponse::Forbidden().body("Wrong user data for auth")
    }
}

///Выдача всех постов
#[get("/page/all")]
pub async fn post_all() -> HttpResponse {
    let collection = get_connection_posts().await;
    match post_getall_all(&collection).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

///Выдача постов от 0 до n
#[get("/page/{n}")]
pub async fn post_all_page(n: web::Path<usize>) -> HttpResponse {
    let collection = get_connection_posts().await;
    match post_get_page(&collection, *n).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
///Поиск поста по конкретной строке, но расплывчато.
///Строка делится на подстроки и выполняется поиск по подстрокам
#[get("/vague/{search}")]
pub async fn search_vague_scope(search: web::Path<String>) -> HttpResponse {
    let collection = get_connection_posts().await;
    match post_search_vague(&collection, search.to_string()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
///Поиск поста по конкретной строке, точно.
///Что ищём, то и находим.
#[get("/fair/{search}")]
pub async fn search_fair_scope(search: web::Path<String>) -> HttpResponse {
    let collection = get_connection_posts().await;
    match post_search_fair(&collection, search.to_string()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

///Поиск поста по конкретной строке, но расплывчато.
///Строка делится на подстроки и выполняется поиск по подстрокам
///По страницам
#[get("/vague/{search}/{page}")]
pub async fn search_vague_scope_pages(req: HttpRequest) -> HttpResponse {
    let search = req.match_info().get("search").unwrap();
    let page = req.match_info().get("page").unwrap();
    let page = page.parse::<usize>().unwrap();
    let collection = get_connection_posts().await;
    match post_search_vague_page(&collection, search.to_string(), page).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

///Поиск поста по конкретной строке, точно.
///Что ищём, то и находим.
/// По страницам
#[get("/fair/{search}/{page}")]
pub async fn search_fair_scope_pages(req: HttpRequest) -> HttpResponse {
    let search = req.match_info().get("search").unwrap();
    let page = req.match_info().get("page").unwrap();
    let page = page.parse::<usize>().unwrap();
    let collection = get_connection_posts().await;
    match post_search_fair_page(&collection, search.to_string(), page).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
// #[get("/code/{name}")]
// pub async fn get_code(name: web::Path<String>) -> HttpResponse {
//     let collection = get_connection_users().await;
//     match code_send(collection, name.to_string()).await {
//         Ok(_) => HttpResponse::Ok().json("Code send"),
//         Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
//     }
// }

///Send code to change pasword.
/// req `<String>` (name of user)
#[get("/code/{name}")]
pub async fn code_send(name: web::Path<String>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_get(collection, name.to_string()).await {
        Ok(v) => {
            match v {
                Some(i) => {
                    match send_password_code(i.email).await {
                        Ok(_) => HttpResponse::Ok().body("Code send"),
                        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
                    }
                }
                None => {
                    HttpResponse::NoContent().body("User not found")
                }
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// [Error] Добавить Auth!
///Add some comments into current `post_id` (post).
#[post("{post}/comment/add")]
pub async fn add_comment(post_id: web::Path<String>, comment: web::Json<Comment>) -> HttpResponse {
    let collection = get_connection_posts().await;
    let post_id = match ObjectId::from_str(&post_id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId"),
    };
    match comment_add(collection, post_id.to_owned(), comment.0).await {
        Ok(_) => HttpResponse::Ok().body("comment added"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

///Delete post
/// with `PostId <String>` + `<Auth>`
#[delete("{post}/comment/delete")]
pub async fn delete_comment(post_id: web::Path<String>, auth: web::Json<Auth>) -> HttpResponse {
    let collection = get_connection_posts().await;
    let post_id = match ObjectId::from_str(&post_id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId"),
    };
    if auth.validate().await {
        match comment_delete(collection, post_id, auth.name.to_string().to_lowercase()).await {
            Ok(_) => HttpResponse::Ok().body("comment added"),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    } else {
        HttpResponse::Forbidden().body("Wront data for auth")
    }
}

///Получаем логин и пароль реализуя создания поста.
#[derive(Serialize, Deserialize, Debug)]
pub struct RequsetPost {
    pub post: PostCreate,
    pub auth: Auth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserChanger {
    pub user: User,
    pub newpassword: String,
}