pub mod comment;
use crate::posts::post::Post;
use comment::Comment;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::to_document;
use mongodb::error::Error;
use mongodb::results::UpdateResult;
use mongodb::Collection;

//Добавляем комментарии к посту
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
    match collection.update_one(filter, update, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}
