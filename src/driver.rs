use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    error::Error,
    options::{DeleteOptions, UpdateOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};
mod cget;
mod comment;
mod post;
mod user;
mod timeconvertor;
use comment::Comment;
use post::Post;
use futures::StreamExt;
use user::User;
use chrono::prelude::*;
//Реализовать oAuth2s
//Реализовать GraphQL
//Реализовать WebSocket

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
//удаляя пользователя, сначала удаляем его посты чтобы их никто не мог забрать.
async fn user_delete(collection: &Collection<User>, user: User) -> Result<DeleteResult, Error> {
    let filter_post = doc! {
        "author": &user.name,
    };

    // Удаляем посты пользователя
    collection
        .delete_many(filter_post, DeleteOptions::builder().build())
        .await?;

    let filter_user = doc! {
        "name": &user.name,
        "email": user.email,
    };

    // Удаляем пользователя
    let result = collection
        .delete_one(filter_user, DeleteOptions::builder().build())
        .await?;
    Ok(result)
}

async fn post_create(collection: &Collection<Post>, post: Post) -> Result<InsertOneResult, Error> {
    let mut post = post;
    if post.id.is_none() {
        post.id = Some(ObjectId::new());
    }
    //Зачем нам конвертировать из TimesTamp в Utc и на оборот если можно хранить u64 или i64?
    //post.date = to_bson_date_time(Utc::now().timestamp_millis()).await?;
    post.date = Utc::now().timestamp_millis() as u64;
    match collection.insert_one(post, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}
//раздели комментарии
async fn post_edit(collection: &Collection<Post>, post: Post) -> Result<UpdateResult, Error> {
    let filter = doc! {
        "_id": post.id
    };
    let update = doc! {
        "$set": {
            "label": &post.label,
            "underlabel": &post.underlabel,
            "text": &post.text,
            "footer": &post.footer,
            "tags": &post.tags,
        }
    };

    collection
        .update_one(filter, update, UpdateOptions::builder().build())
        .await
}

async fn comment_to(
    collection: &Collection<Post>,
    post_id: ObjectId,
    comment: Comment,
) -> Result<UpdateResult, Error> {
    let filter = doc! {
        "id": post_id,
    };
    let update = doc! {
        "$push": {
            "comments": to_document(&comment)?
        }
    };
    Ok(collection.update_one(filter, update, None).await?)
}

async fn post_delete(
    collection: &Collection<Post>,
    post_id: ObjectId,
) -> Result<DeleteResult, Error> {
    let filter = doc! {
        "_id": post_id,
    };
    Ok(collection
        .delete_one(filter, DeleteOptions::builder().build())
        .await?)
}
//get all user's posts
async fn post_getall(
    collection: &Collection<Post>,
    author: String,
) -> Result<Vec<Option<Post>>, Error> {
    let filter = doc! {
      "author": author
    };
    let mut cursor = collection.find(filter, None).await?;
    // Используем `StreamExt` для асинхронного перебора результатов
    let mut posts = Vec::new();
    while let Some(post) = cursor.next().await {
        match post {
            Ok(post) => {
                posts.push(Some(post));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(posts)
}
//for single post
async fn post_get(collection: &Collection<Post>, post_id: ObjectId) -> Result<Option<Post>, Error> {
    let filter = doc! {
        "id": post_id,
    };
    match collection.find_one(filter, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}
