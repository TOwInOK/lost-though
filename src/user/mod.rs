pub mod user;
use crate::autentifications::auth::Auth;
use crate::posts::post::Post;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::options::DeleteOptions;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::Collection;
use user::User;

use self::user::Role;

///Выдаём пользователя;
///сравнивая его по имени;
///если пользователь есть, то выводим ошибку,
///нету создаём.
pub async fn user_create(
    collection: &Collection<User>,
    mut user: User,
) -> Result<InsertOneResult, Error> {
    let filter = doc! {
        "name": &user.name
    };
    user.role = Role::Default;
    match collection.find_one(filter, None).await {
        Ok(Some(_)) => Err(Error::from(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Пользователь уже существует в базе данных.",
        ))),
        Ok(None) => match collection.insert_one(user, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
///Фильтруем пользователя по имени ```collection.find_one``` и выдаём Some(user)
pub async fn user_get(collection: &Collection<User>, name: &String) -> Result<Option<User>, Error> {
    let filter = doc! {
        "name": name
    };
    match collection.find_one(filter, None).await {
        Ok(result) => {
            eprintln!("{:#?}", result);
            Ok(result)}
            ,
        Err(e) => Err(e),
    }
}

pub async fn user_change(collection: &Collection<User>, user: User) -> Result<UpdateResult, Error> {
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

pub async fn user_delete(
    collection_user: &Collection<User>,
    collection_post: &Collection<Post>,
    user: Auth,
) -> Result<DeleteResult, Error> {
    let filter_post = doc! {
        "author": {
            "$in": [&user.name]
        }
    };
    let filter_user = doc! {
        "name": &user.name,
        "password": user.password,
    };
    // Удаляем пользователя
    let result = collection_user
        .delete_one(filter_user, DeleteOptions::builder().build())
        .await?;
    // Удаляем посты пользователя
    collection_post
        .delete_many(filter_post, DeleteOptions::builder().build())
        .await?;
    Ok(result)
}
