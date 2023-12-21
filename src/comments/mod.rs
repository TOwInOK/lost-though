pub mod comment;
use crate::posts::post::Post;
use comment::Comment;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    options::DeleteOptions,
    results::{DeleteResult, UpdateResult},
    Collection,
};
use std::error::Error;

///Добавляем комментарии к посту
pub async fn comment_add(
    collection: Collection<Post>,
    post_id: ObjectId,
    comment: Comment,
) -> Result<UpdateResult, Box<dyn Error>> {
    let filter = doc! {
        "_id": post_id,
    };
    let update = doc! {
        "$push": {
            "comments": to_document(&comment)?
        }
    };
    match collection.update_one(filter, update, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn comment_delete(
    collection: Collection<Post>,
    post_id: ObjectId,
    validated_username: String,
) -> Result<DeleteResult, Box<dyn Error>> {
    let filter = doc! {
        "_id": post_id,
        "author": validated_username,
    };
    match collection
        .delete_one(filter, DeleteOptions::builder().build())
        .await
    {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(e)),
    }
}
