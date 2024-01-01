use crate::cli::Cli;
use crate::posts::post::Post;
use crate::users::user::User;
use mongodb::{options::ClientOptions, Client, Collection};

///Создание соединения с базой данных к полям подбазе users в базе Main
#[allow(unused)]
pub async fn get_connection_users() -> Collection<User> {
    let adress = Cli::mongo_adress().await;
    let client_options = ClientOptions::parse_async(adress)
        .await
        .expect("Ошибка подключение к базе данных");
    let client = Client::with_options(client_options).expect("Ошибка создание клиента -> User");
    let database = client.database("Main");

    database.collection::<User>("users")
}

///Создание соединения с базой данных к полям подбазе posts в базе Main
#[allow(unused)]
pub async fn get_connection_posts() -> Collection<Post> {
    let adress = Cli::mongo_adress().await;
    let client_options = ClientOptions::parse_async(adress)
        .await
        .expect("Ошибка подключение к базе данных");
    let client = Client::with_options(client_options).expect("Ошибка создание клиента -> Post");
    let database = client.database("Main");

    database.collection::<Post>("posts")
}
