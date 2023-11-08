pub mod user;
use crate::posts::post::Post;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::options::DeleteOptions;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::Collection;
use user::User;

pub async fn user_create(
    collection: &Collection<User>,
    user: User,
) -> Result<InsertOneResult, Error> {
    let filter = doc! {
        "name": &user.name
    };
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

pub async fn user_get(collection: &Collection<User>, name: String) -> Result<Option<User>, Error> {
    let filter = doc! {
        "name": name
    };
    match collection.find_one(filter, None).await {
        Ok(result) => Ok(result),
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
    user: User,
) -> Result<DeleteResult, Error> {
    let filter_post = doc! {
        "author": {
            "$in": [&user.name]
        }
    };

    // Удаляем посты пользователя
    collection_post
        .delete_many(filter_post, DeleteOptions::builder().build())
        .await?;

    let filter_user = doc! {
        "name": &user.name,
        "email": user.email,
    };

    // Удаляем пользователя
    let result = collection_user
        .delete_one(filter_user, DeleteOptions::builder().build())
        .await?;
    Ok(result)
}
