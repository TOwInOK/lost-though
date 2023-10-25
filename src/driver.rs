mod post;
mod user;

use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    options::ClientOptions,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use post::Post;
use user::User;

const ADDRESS: &str = "mongodb://root:example@192.168.0.15:27017";

async fn get_connection_users() -> Collection<User> {
    let client_options = ClientOptions::parse_async(ADDRESS)
        .await
        .expect("Ошибка подключение к базе данных");
    let client = Client::with_options(client_options).expect("Ошибка создание клиента -> User");
    let database = client.database("Main");
    return database.collection::<User>("users");
}

async fn get_connection_posts() -> Collection<Post> {
    let client_options = ClientOptions::parse_async(ADDRESS)
        .await
        .expect("Ошибка подключение к базе данных");
    let client = Client::with_options(client_options).expect("Ошибка создание клиента -> Post");
    let database = client.database("Main");
    return database.collection::<Post>("posts");
}

async fn user_create(collection: &Collection<User>, user: User) -> Result<InsertOneResult, Error> {
    let filter = doc! {
        "name": &user.name
    };
    match collection.find_one(filter, None).await {
        Ok(Some(_)) => {
            return Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Пользователь уже существует в базе данных.",
            )));
        }
        Ok(None) => match collection.insert_one(user, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

pub async fn user_get(name: String, collection: &Collection<User>) -> Result<User, Error> {
    let filter = doc! {
        "name": name
    };
    if let Some(user) = collection.find_one(filter, None).await? {
        Ok(user)
    } else {
        Err(Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Пользователь не найден",
        )))
    }
}
// we can't change login :)
// get some user and change all atribute, but can't change login cause we need the stable name
async fn user_change(collection: &Collection<User>, user: User) -> Result<UpdateResult, Error> {
    let filter = doc! {
        "name": user.name
    };
    let update = doc! {
        "$set": {
            "password": user.password,
            "email": user.email,
        }
    };
    match collection.update_one(filter, update, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

async fn user_delete(collection: &Collection<User>, user: User) -> Result<DeleteResult, Error> {
    let filter = doc! {
        "name": user.name,
        "email": user.email,
    };
    Ok(collection.delete_one(filter, None).await?)
}

async fn post_create(collection: &Collection<Post>, post: Post) -> Result<InsertOneResult, Error> {
    let mut post = post.to_owned();
    post.id = Some(ObjectId::new());
    match collection.insert_one(post, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

async fn post_edit() {
    todo!()
}

async fn post_delete() {
    todo!()
}

async fn post_getall() {
    todo!()
}

async fn post_get() {
    todo!()
}
