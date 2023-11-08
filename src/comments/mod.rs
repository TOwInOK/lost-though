pub mod comment;
use comment::Comment;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::bson::oid::ObjectId;
use mongodb::results::UpdateResult;
use mongodb::bson::to_document;
use crate::posts::post::Post;

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