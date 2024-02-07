use crate::cli::Cli;
use crate::posts::post::Post;
use crate::users::user::User;
use mongodb::{options::ClientOptions, Client, Collection};
use thiserror::Error;

///Создание соединения с базой данных к полям подбазе users в базе Main
pub async fn get_connection_users() -> Result<Collection<User>, IOErrors> {
    let adress = Cli::mongo_adress().await;
    let client_options = ClientOptions::parse_async(adress)
        .await
        .expect("Ошибка подключение к базе данных");
    match Client::with_options(client_options) {
        Ok(client) => {
            let database = client.database("Main");
            Ok(database.collection::<User>("users"))
        }
        Err(e) => Err(IOErrors::GetUser(e.to_string())),
    }
}

///Создание соединения с базой данных к полям подбазе posts в базе Main
pub async fn get_connection_posts() -> Result<Collection<Post>, IOErrors> {
    let adress = Cli::mongo_adress().await;
    let client_options = ClientOptions::parse_async(adress)
        .await
        .expect("Ошибка подключение к базе данных");
    match Client::with_options(client_options) {
        Ok(client) => {
            let database = client.database("Main");
            Ok(database.collection::<Post>("posts"))
        }
        Err(e) => Err(IOErrors::GetPost(e.to_string())),
    }
}

#[derive(Debug, Error, Clone)]
pub enum IOErrors {
    #[error("GetUser error: `{0}`")]
    GetUser(String),
    #[error("GetPost error: `{0}`")]
    GetPost(String),
}
