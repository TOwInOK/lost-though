mod app;
use actix_web::{middleware::NormalizePath, web, App, HttpServer};
use app::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting on http://127.0.0.1:8080");
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
                    .service(create_user),
            )
            .service(
                web::scope("/post")
                    .service(post)
                    .service(post_editor)
                    .service(post_deleter)
                    .service(post_editor)
                    .service(create)
                    .service(post_all)
                    .service(post_all_page),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
