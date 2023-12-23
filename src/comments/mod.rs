pub mod comment;
use crate::posts::post::Post;
use comment::Comment;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    results::UpdateResult,
    Collection,
};
use std::error::Error;

///Add commet to post throw UUID of post and get UUID of comment
pub async fn comment_add(
    collection: Collection<Post>,
    post_id: ObjectId,
    mut comment: Comment,
) -> Result<ObjectId, Box<dyn Error>> {
    let id = ObjectId::new();
    comment.id = Some(id);
    let filter = doc! {
        "_id": post_id,
    };
    let update = doc! {
        "$push": {
            "comments": to_document(&comment)?
        }
    };
    match collection.update_one(filter, update, None).await {
        Ok(_) => Ok(id),
        Err(e) => Err(Box::new(e)),
    }
}

///Use UUID of comment to remove it
pub async fn comment_delete(
    collection: Collection<Post>,
    post_id: ObjectId,
    comment_id: ObjectId,
) -> Result<UpdateResult, Box<dyn Error>> {
    let filter = doc! {
        "_id": post_id,
        "comments": {
            "$elemMatch": {
                "_id": comment_id
            }
        }
    };

    let update = doc! {
        "$pull": {
            "comments": {
                "_id": comment_id
            }
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(e)),
    }
}
