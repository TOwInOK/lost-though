use crate::driver::Post;
use crate::driver::User;
use mongodb::{options::ClientOptions, Client, Collection};
const ADDRESS: &str = "mongodb://root:example@192.168.0.15:27017";

pub async fn get_connection_users() -> Collection<User> {
    let client_options = ClientOptions::parse_async(ADDRESS)
        .await
        .expect("Ошибка подключение к базе данных");
    let client = Client::with_options(client_options).expect("Ошибка создание клиента -> User");
    let database = client.database("Main");
    return database.collection::<User>("users");
}
pub async fn get_connection_posts() -> Collection<Post> {
    let client_options = ClientOptions::parse_async(ADDRESS)
        .await
        .expect("Ошибка подключение к базе данных");
    let client = Client::with_options(client_options).expect("Ошибка создание клиента -> Post");
    let database = client.database("Main");
    return database.collection::<Post>("posts");
}
