pub mod post;

use post::Post;
use futures::StreamExt;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{UpdateResult, InsertOneResult, DeleteResult};
use mongodb::options::{UpdateOptions, DeleteOptions};
use chrono::Utc;

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
pub async fn post_edit(collection: &Collection<Post>, post: Post, author: String) -> Result<UpdateResult, Error> {
    let filter = doc! {
        "_id": post.id,
        "author": author
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

pub async fn post_delete(
    collection: &Collection<Post>,
    post_id: String,
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
    post_id: String,
) -> Result<Option<Post>, Error> {
    let filter = doc! {
        "_id": post_id,
    };
    match collection.find_one(filter, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}