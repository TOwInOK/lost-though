use actix_files as fs;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use back::{
    autentifications::auth::Auth,
    comments::{comment::Comment, comment_add, comment_delete},
    mongolinks::cget::{get_connection_posts, get_connection_users},
    posts::{post::PostCreate, *},
    sendcode::email::{check_code, send_password_code},
    users::{
        user::{User, UserMin},
        *,
    },
};
use log::{error, info, trace};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const INDEX_HTML: &str = "./static/about.html";
///Main doc page
#[get("/")]
async fn index() -> impl Responder {
    trace!("access to index!");
    fs::NamedFile::open_async(INDEX_HTML).await
}
///Doc page
#[get("/{path:.*\\.(html|css|js)}")]
async fn indexx(path: web::Path<String>) -> actix_web::Result<fs::NamedFile> {
    let path = format!("./static/{}", path);
    trace!("access to {}", &path);
    info!("{}", path);
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
            error!("{:?}", e);
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
            info!("{:?}", &user);
            HttpResponse::Ok().json(anonymus_user)
        }
        Ok(None) => {
            info!("User not found");
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
                info!("validation..fine [settings]");
                HttpResponse::Ok().json(i)
            } else {
                info!("fail validation [settings]");
                HttpResponse::Forbidden().body("Wrong password")
            }
        }
        Ok(None) => HttpResponse::NoContent().body(""),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
///Change pass
/// req: `<String>` user name + `UserChanger`
#[put("/{name}/settings/changepass")]
pub async fn pass_changer(
    name: web::Path<String>,
    mut auth: web::Json<UserChangerAuth>,
) -> HttpResponse {
    let collection = get_connection_users().await;
    if auth.auth.password.is_empty() {
        return HttpResponse::BadRequest().body("Password is empty");
    }
    if auth.newpassword.is_empty() {
        return HttpResponse::BadRequest().body("New Password is empty");
    }
    match user_get(collection.clone(), name.to_string()).await {
        Ok(Some(i)) => {
            if i.validate_anonimus(&auth.auth) {
                auth.auth.password = auth.newpassword.clone();
                match user_change_pass(&collection, auth.auth.clone()).await {
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

#[put("/{name}/settings/change")]
pub async fn user_changer(
    name: web::Path<String>,
    mut auth: web::Json<UserChanger>,
) -> HttpResponse {
    let collection = get_connection_users().await;
    if auth.user.password.is_empty() {
        return HttpResponse::BadRequest().body("Password is empty");
    }
    if auth.newpassword.is_empty() {
        return HttpResponse::BadRequest().body("New Password is empty");
    }
    match user_get(collection.clone(), name.to_string()).await {
        Ok(Some(i)) => {
            if i.validate(&auth.user) {
                auth.user.password = auth.newpassword.clone();
                match user_change(&collection, auth.user.clone()).await {
                    Ok(_) => HttpResponse::Ok().body("User changed"),
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
#[put("/edit")]
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
        match post_delete(&collection, id.to_string(), p.name.clone()).await {
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
        Ok(v) => match v {
            Some(i) => {
                match send_password_code(i.email, name.clone().to_string().to_lowercase()).await {
                    Ok(_) => HttpResponse::Ok().body("Code send"),
                    Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
                }
            }
            None => HttpResponse::NotFound().body("User not found"),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/code/{code}")]
pub async fn code_get(code: web::Path<usize>, auth: web::Json<Auth>) -> HttpResponse {
    if auth.password.is_empty() {
        return HttpResponse::BadRequest().body("Password is empty");
    }
    let collection = get_connection_users().await;
    match user_get(collection.clone(), auth.name.to_string()).await {
        Ok(v) => match v {
            Some(_) => match check_code(*code, auth.name.to_string().to_lowercase()).await {
                Ok(_) => match user_change_pass(&collection, auth.0).await {
                    Ok(_) => HttpResponse::Ok().body("password changed"),
                    Err(e) => HttpResponse::NotFound().body(e.to_string()),
                },
                Err(e) => HttpResponse::NotFound().body(e.to_string()),
            },
            None => HttpResponse::NoContent().body("User not found"),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("{post}/comment/add")]
pub async fn add_comment(
    post_id: web::Path<String>,
    comment: web::Json<CommentAdd>,
) -> HttpResponse {
    let collection = get_connection_posts().await;
    //check name in auth and name in comment, and validate it.
    if comment.0.auth.validate().await && comment.0.auth.name == comment.0.comment.author {
        let post_id = match ObjectId::from_str(&post_id) {
            Ok(oid) => oid,
            Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId"),
        };
        match comment_add(collection, post_id, comment.0.comment).await {
            Ok(v) => HttpResponse::Ok().body(v.to_string()),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    } else {
        HttpResponse::Forbidden().body("Auth fail")
    }
}

///Delete post
/// with `PostId <String>` + `<Auth>`
#[delete("{post_id}/comment/delete")]
pub async fn delete_comment(
    post_id: web::Path<String>,
    comment: web::Json<CommentDelete>,
) -> HttpResponse {
    let collection = get_connection_posts().await;
    //try to convert from String to ObjectID
    let post_id = match ObjectId::from_str(&post_id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid post ID"),
    };
    //try to convert from String to ObjectID
    let comment_id = match ObjectId::from_str(&comment.0.comment_id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid post ID"),
    };
    if comment.0.auth.validate().await {
        match comment_delete(collection, post_id, comment_id).await {
            Ok(_) => HttpResponse::Ok().body("comment deleted"),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserChangerAuth {
    pub auth: Auth,
    pub newpassword: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserChanger {
    pub user: User,
    pub newpassword: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentAdd {
    pub comment: Comment,
    pub auth: Auth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentDelete {
    pub comment_id: String,
    pub auth: Auth,
}
