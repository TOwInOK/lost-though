use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    error::Error,
    options::{DeleteOptions, UpdateOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};
pub mod cget;
pub mod comment;
pub mod post;
pub mod user;
use chrono::prelude::*;
use comment::Comment;
use futures::StreamExt;
use post::Post;
use user::User;
//Реализовать oAuth2s
//Реализовать GraphQL
//Реализовать WebSocket

pub async fn user_create(
    collection: &Collection<User>,
    user: User,
) -> Result<InsertOneResult, Error> {
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

pub async fn user_get(collection: &Collection<User>, name: String) -> Result<Option<User>, Error> {
    let filter = doc! {
        "name": name
    };
    match collection.find_one(filter, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}
// we can't change login :)
// get some user and change all atribute, but can't change login cause we need the stable name
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
//удаляя пользователя, сначала удаляем его посты чтобы их никто не мог забрать.
pub async fn user_delete(collection_user: &Collection<User>, collection_post: &Collection<Post>, user: User) -> Result<DeleteResult, Error> {
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

pub async fn post_create(
    collection: &Collection<Post>,
    post: Post,
) -> Result<InsertOneResult, Error> {
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
pub async fn post_edit(collection: &Collection<Post>, post: Post) -> Result<UpdateResult, Error> {
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

pub async fn comment_to(
    collection: &Collection<Post>,
    post_id: ObjectId,
    comment: Comment,
) -> Result<UpdateResult, Error> {
    let filter = doc! {
        "_id": post_id,
    };
    let update = doc! {
        "$push": {
            "comments": to_document(&comment)?
        }
    };
    Ok(collection.update_one(filter, update, None).await?)
}

pub async fn post_delete(
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
pub async fn post_getall(
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
pub async fn post_get(
    collection: &Collection<Post>,
    post_id: ObjectId,
) -> Result<Option<Post>, Error> {
    let filter = doc! {
        "_id": post_id,
    };
    match collection.find_one(filter, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

