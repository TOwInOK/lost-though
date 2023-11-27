pub mod post;

use chrono::Utc;
use futures::StreamExt;
use mongodb::{bson::{doc, oid::ObjectId}, error::Error, options::{DeleteOptions, UpdateOptions, FindOptions}, results::{DeleteResult, InsertOneResult, UpdateResult}, Collection};
use post::Post;
use self::post::PostCreate;

///Создание поста
pub async fn post_create(
    collection: &Collection<Post>,
    post: PostCreate,
) -> Result<InsertOneResult, Error> {
    let post = Post {
        id: None,
        author: post.author,
        underlabel: post.underlabel,
        label: post.label,
        text: post.text,
        footer: post.footer,
        tags: post.tags,
        comments: vec![],
        date: Utc::now().timestamp_millis() as u64
    };
    match collection.insert_one(post, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}
///Редактирование поста
pub async fn post_edit(
    collection: &Collection<Post>,
    post: PostCreate,
    author: String,
) -> Result<UpdateResult, Box<Error>> {
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
        .map_err(|e| e.into())
}


///Удаление поста
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

///Получаем все посты пользователя по имени. Если имя будет в посте в Vec(author) то оно будет возвращено
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
///Получение информации о посте по его id.
///Id получаем при фетче постов пользователя либо из первых постов главной страницы
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

///Получаем все посты из базы данных
pub async fn post_getall_all(collection: &Collection<Post>) -> Result<Vec<Option<Post>>, Error> {
    let mut cursor = collection.find(doc! {}, None).await?;
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

///Получаем посты постранично.
///От 0 до n. Если 0 то выдаём 10 постов, если 1 то 20 и так далее
pub async fn post_get_page(
    collection: &Collection<Post>,
    page: usize,
) -> Result<Vec<Option<Post>>, Error> {
    let skiprange = match page {
        0 => 0,
        1 => 0,
        _ => (page - 1) * 10,
        
    };
    let limitrange = (skiprange + 10) as i64;
        
    let options = mongodb::options::FindOptions::builder()
    .skip(skiprange as u64)
    .limit(limitrange)
    .build();
    let mut cursor = collection
        .find(doc! {}, options)
        .await?;
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
    println!("{:#?}", posts);
    Ok(posts)
}