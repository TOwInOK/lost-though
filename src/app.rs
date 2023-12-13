use actix_web::put;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use back::autentifications::auth::Auth;
use back::comments::comment_delete;
use back::comments::{comment::Comment, comment_add};
use back::mongolinks::cget::{get_connection_posts, get_connection_users};
use back::posts::post::PostCreate;
use back::posts::*;
use back::user::user::{User, UserMin};
use back::user::*;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;

//Добавь коментарий к посту

///а почему нет
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body(HTML)
}
///создание пользователя
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
///инфа о пользователе
#[get("/{name}")]
pub async fn user(name: web::Path<String>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_get(&collection, &name.to_string()).await {
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
///получение инфы о пользователе - вся котороя может быть в User
/// так же подходит для входа :)
///нужен пароль
#[get("/{name}/settings")]
pub async fn get_user_settings(name: web::Path<String>, u: web::Json<Auth>) -> HttpResponse {
    let collection = get_connection_users().await;
    match user_get(&collection, &name).await {
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
///Меняем пароль отправляя пользователя
///нужен пароль
#[put("/{name}/changepass")]
pub async fn user_changer(name: web::Path<String>, mut u: web::Json<UserChanger>) -> HttpResponse {
    let collection = get_connection_users().await;
    if u.user.password.is_empty() {
        return HttpResponse::BadRequest().body("Password is empty");
    }
    if u.newpassword.is_empty() {
        return HttpResponse::BadRequest().body("New Password is empty");
    }
    match user_get(&collection, &name).await {
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
#[get("/code/{name}")]
pub async fn get_code(name: web::Path<String>) -> HttpResponse {
    let collection = get_connection_users().await;
    match code_send(collection, name.to_string()).await {
        Ok(_) => HttpResponse::Ok().json("Code send"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn code_send(_collection: Collection<User>, _name: String) -> Result<(), Box<dyn Error>> {
    Ok((todo!("Сделать отсылку кодов")))
}

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

const HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.24.1/themes/prism-dark.min.css" />
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.24.1/components/prism-json.min.js"></script>
    <style>
        body {
            font-family: Arial, sans-serif;
            line-height: 1.6;
            margin: 20px;
            background-color: #1c1c1c;
            color: #e0e0e0;
        }

        h1 {
            color: #bb86fc;
        }

        h2 {
            color: #03dac6;
        }

        h3 {
            color: #03dac6;
        }

        h4 {
            color: #03dac6;
        }

        p {
            color: #bdbdbd;
        }

        code {
            background-color: #282a36;
            border: 1px solid #44475a;
            border-radius: 4px;
            display: block;
            margin: 10px 0;
            padding: 10px;
        }

        pre {
            background-color: #282a36;
            border: 1px solid #44475a;
            border-radius: 4px;
            display: block;
            margin: 10px 0;
            padding: 10px;
            overflow: auto;
        }
    </style>
</head>

<body>
    <h1>Api response and request of <code>api.lost-umbrella.com</code></h1>

    <h2>Scopes:</h2>

    <h3>/user</h3>

    <h4>Post: /&lt;name&gt; - for example, /anton</h4>
    <p>Use the USERNAME in the path to retrieve the UserMin structure.</p>

    <h4>GET: /&lt;name&gt;/posts</h4>
    <p>Get all posts by the USERNAME.</p>

    <h4>GET: /{name}/settings</h4>
    <p>Using the Auth structure to get user data in the form of the User structure for further use.</p>

    <h4>POST: /{name}/changepass</h4>
    <p>Requires a saved User structure for modification.</p>

    <h4>DELETE: /delete</h4>
    <p>Send the Auth structure.</p>

    <h4>POST: /create</h4>
    <p>Create a user using the User structure.</p>

    <pre>
      <code class="language-json">
        UserMin
        {
            "name": "xxx",
            "role": "default"
        }

        Auth
        {
            "name": "xxx",
            "password": "super_puper_password228"
        }

        User
        {
            "name": "xxx",
            "password": "super_puper_password228",
            "email": "xxx@xxx.x",
            "role": "default"
        }
      </code>
    </pre>

    <h3>/post</h3>

    <h4>GET: /&lt;post_id&gt; - for example, /6557b9f2417e299f07b8096a</h4>
    <p>Send the id<br> Get the Post.</p>

    <h4>POST: /&lt;post_id&gt;/edit</h4>
    <p>Send RequsetPost (id not required).</p>

    <h4>POST: /create</h4>
    <p>Send RequsetPost<br> Get Post.</p>


    <h4>POST: /page/all</h4>

    <p>Send nothing<br> Get all posts in bd ;)</p>

    <h4>POST: /page/{number}</h4>

    <p>Send number of page<br> 1 number = 10 posts<br> 2 number = 20 posts<br> end etc...</p>

    <h4>DELETE: /&lt;post&gt;/delete</h4>
    <p>Decided not to create another Api request type.<br> Send id<br> Send Auth<br> Get HttpResponse (we get this type
        of message everywhere, only the json is always custom if it is implied).</p>

    <pre>
      <code class="language-json">
        RequsetPost (id not specified)
        {
          "post": {
            "author": ["author_name1", "author_name2"],
            "underlabel": "underlabel_value",
            "label": "label_value",
            "text": "text_value",
            "footer": "footer_value",
            "tags": ["tag1", "tag2"]
          },
          "auth": {
            "name": "example_user",
            "password": "example_password"
          }
        }

        Response from /create:
        {
          "insertedId": {
            "$oid": "656387b65691e07a9f22cffd"
          }
        }

        //We get a response from MongoDB directly, along with its errors.

        Auth
        {
          "name": "xxx",
          "password": "super_puper_password228"
        }

        Post (Example) - what we get
        {
          "_id": {
            "$oid": "6557c5d77a925f835493442d"
          },
          "author": [
            "test21fd"
          ],
          "date": 1700251094838,
          "underlabel": "Example Underlabel",
          "label": "Example Label",
          "text": "Example Text",
          "footer": "Example Footer",
          "tags": [
            "Tag1",
            "Tag2"
          ],
          "comments": []
        }
      </code>
    </pre>

    <pre>
      <code class="language-json">
        comment - for now, you can't send it
        when there is no response
        {
          "author": "comment_author_name",
          "text": "comment_text",
          "reject": null
        }
        when there is a response (reject is still being worked on)
        {
          "author": "comment_author_name",
          "text": "comment_text",
          "reject": {
            "$oid": "6557c5d77a925f835493442d"
          }
        }
      </code>
    </pre>

    <h2>/search</h2>

    <h4>GET: /vague/{text}</h4>

    <p>Send text<br> Getting posts with vague searches</p>

    <h4>GET: /fair/{text}</h4>
    <p>Send text<br> Getting posts with accurate searches</p>

    <h4>GET: /vague/{text}/{number of page}</h4>
    <p>Send text<br> Send number of page<br> Getting posts with vague searches<br> 0 = 10 too<br> 1 number = 10 posts</p>

    <h4>GET: /fair/{text}/{number of page}</h4>
    <p>Send text<br> Send number of page<br> Getting posts with accurate searches<br> 0 = 10 too<br> 1 number = 10 posts</p>


</body>

</html>
"#;
