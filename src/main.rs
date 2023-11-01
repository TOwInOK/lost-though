// mod driver;
// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

use chrono::Utc;
use driver::cget::get_connection_posts;
use driver::cget::get_connection_users;
use driver::comment::Comment;
use driver::comment_to;
use driver::post::Post;
use driver::post_get;
use driver::post_getall;
use driver::user::User;
use driver::user_delete;
use driver::{user_change, user_create, user_get};
use mongodb::bson::oid::ObjectId;
mod driver;

#[tokio::main]
async fn main() {
    //User Monog Test
    let collection = get_connection_users().await;
    let user: User = User {
        name: "123zzz".to_string(),
        password: "fff".to_string(),
        email: "fff@fff.mail".to_string(),
    };
    println!("-----------------------");
    match user_create(&collection, user.clone()).await {
        Ok(i) => println!("{:#?}", i),
        Err(e) => println!("{:#?}", e),
    }
    println!("-----------------------");
    match user_get(&collection, "123zzz".to_string()).await {
        Ok(i) => println!("{:#?}", i),
        Err(e) => println!("{:#?}", e),
    }
    println!("-----------------------");
    let user_changed: User = User {
        name: "123zzz".to_string(),
        password: "ffffdfdfdfdd".to_string(),
        email: "fff@fff.mail".to_string(),
    };
    match user_change(&collection, user_changed).await {
        Ok(i) => println!("{:#?}", i),
        Err(e) => println!("{:#?}", e),
    }
    println!("-----------------------");
    // match user_delete(&collection, user).await {
    //     Ok(i) => println!("{:#?}", i),
    //     Err(e) => println!("{:#?}", e),
    // }
    println!("-----------------------");
    match user_get(&collection, "123zzz".to_string()).await {
        Ok(i) => println!("{:#?}", i),
        Err(e) => println!("{:#?}", e),
    }
    println!("-----------------------");
    //User Monog Test
    //Post Monog Test
    let commentc: Comment = Comment {
        id: None,
        author: "123zzz".to_string(),
        text: "eee".to_string(),
        reject: None,
    };
    let post: Post = Post {
        id: None,
        author: vec!["123zzz".to_string()],
        date: Utc::now().timestamp_millis() as u64,
        underlabel: "ulabel".to_string(),
        label: "Lable".to_string(),
        text: "sometext".to_string(),
        footer: "someFooter".to_string(),
        tags: vec!["1".to_string()],
        comments: vec![],
    };
    let collection = get_connection_posts().await;
    // match post_create(&collection, post).await {
    //     Ok(i) => println!("{:#?}", i),
    //     Err(e) => println!("{:#?}", e),
    // }
    //Post Monog Test
    let obj = ObjectId::parse_str("6542218e99a717d885401647").expect("ffff");
    match post_get(&collection, obj.clone()).await {
        Ok(i) => println!("{:#?}", i),
        Err(e) => println!("{:#?}", e),
    }
    println!("-----------------------");
    match comment_to(&collection, obj, commentc).await {
        Ok(i) => println!("{:#?}", i),
        Err(e) => println!("{:#?}", e),
    }
    println!("-----------------------");
    match post_getall(&collection, "123zzz".to_string()).await {
        Ok(i) => println!("{:#?}", i),
        Err(e) => println!("{:#?}", e),
    }
    let collection_user = get_connection_users().await;
    match user_delete(&collection_user,&collection, user).await {
        Ok(i) => println!("{:#?}", i),
        Err(e) => println!("{:#?}", e),
    }
}
