pub mod post;

use chrono::Utc;
use futures::StreamExt;
use mongodb::{bson::{doc, oid::ObjectId}, error::Error, options::{DeleteOptions, UpdateOptions}, results::{DeleteResult, InsertOneResult, UpdateResult}, Collection};
use post::Post;

//Создание поста
pub async fn post_create(
    collection: &Collection<Post>,
    post: Post,
) -> Result<InsertOneResult, Error> {
    let mut post = post;
    if post.id.is_none() {
        post.id = Some(ObjectId::new());
    }
    //Зачем нам конвертировать из TimesTamp в Utc и на оборот если можно хранить i64?
    //post.date = to_bson_date_time(Utc::now().timestamp_millis()).await?;
    post.date = Utc::now().timestamp_millis() as u64;
    match collection.insert_one(post, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}
//Редактирование поста
pub async fn post_edit(
    collection: &Collection<Post>,
    post: Post,
    author: String,
) -> Result<UpdateResult, Error> {
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

//Удаление поста
pub async fn post_delete(
    collection: &Collection<Post>,
    post_id: String,
) -> Result<DeleteResult, Error> {
    let filter = doc! {
        "_id": post_id,
    };
    collection
        .delete_one(filter, DeleteOptions::builder().build())
        .await
}

//Получаем все посты пользователя по имени. Если имя будет в посте в Vec(author) то оно будет возвращено
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
//Получение информации о посте по его id.
//Id получаем при фетче постов пользователя либо из первых постов главной страницы
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
