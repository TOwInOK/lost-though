mod app;
use app::*;
use actix_web::{web,App,HttpServer, middleware::NormalizePath};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(NormalizePath::default())
        .service(hello)
        .service(
            web::scope("/user")
                .service(user)
                .service(postall)
                .service(get_user_settings)
                .service(user_changer)
                .service(delete_user)
                .service(create_user)
            )
        .service(
            web::scope("/post")
            .service(post)
            .service(post_editor)
            .service(post_deleter)
            .service(post_editor)
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


