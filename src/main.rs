mod app;
use actix_web::{middleware::NormalizePath, web, App, HttpServer};
use app::*;
use back::cli::Cli;
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let web_port = Cli::web_port().await;
    info!("Version: 1");
    info!("Env values:");
    pretty_env_logger::init();
    for (key, value) in env::vars() {
        info!("VALUE: {}: {}", key, value);
    }
    info!("Server starting on 0.0.0.0:{:?}", web_port);
    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::trim())
            .service(index)
            .service(indexx)
            .service(
                web::scope("/user")
                    .service(user)
                    .service(postall)
                    .service(get_user_settings)
                    .service(pass_changer)
                    .service(user_changer)
                    .service(delete_user)
                    .service(create_user)
                    .service(code_send)
                    .service(code_get),
            )
            .service(
                web::scope("/post")
                    .service(post)
                    .service(post_editor)
                    .service(post_deleter)
                    .service(post_editor)
                    .service(create)
                    .service(post_all)
                    .service(post_all_page)
                    .service(add_comment)
                    .service(delete_comment),
            )
            .service(
                web::scope("/search")
                    .service(search_vague_scope)
                    .service(search_fair_scope)
                    .service(search_vague_scope_pages)
                    .service(search_fair_scope_pages),
            )
    })
    .bind(("0.0.0.0", web_port))?
    .run()
    .await
}
