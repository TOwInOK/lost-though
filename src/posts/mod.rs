pub mod post;

use self::post::PostCreate;
use chrono::Utc;
use futures::StreamExt;
use log::{debug, error, info};
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    options::{DeleteOptions, FindOptions, UpdateOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};
use post::Post;

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
        date: Utc::now().timestamp_millis() as u64,
    };
    match collection.insert_one(post, None).await {
        Ok(result) => {
            info!("Post created successfully");
            debug!("{:?}", result);
            Ok(result)
        }
        Err(e) => {
            error!("Error creating post: {:?}", e);
            Err(e)
        }
    }
}
///Редактирование поста
pub async fn post_edit(
    collection: &Collection<Post>,
    post: PostCreate,
    author: String,
) -> Result<UpdateResult, Box<Error>> {
    let filter = doc! {
        "_id": &post.id,
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

    info!("Updating post {:#?}", &post.id);
    debug!("Filter: {:?}", filter);
    debug!("Update: {:?}", update);

    collection
        .update_one(filter, update, UpdateOptions::builder().build())
        .await
        .map_err(|e| e.into())
}
///Удаление поста
pub async fn post_delete(
    collection: &Collection<Post>,
    post_id: ObjectId,
    author: String,
) -> Result<DeleteResult, Error> {
    let filter = doc! {
        "_id": &post_id,
        "author": author
    };

    info!("Deleting post {}", post_id);
    debug!("Filter: {:?}", &filter);

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
        "author": &author
    };
    let mut cursor = collection.find(filter, None).await?;

    info!("Fetching all posts for author {}", author);

    let mut posts = Vec::new();
    while let Some(post) = cursor.next().await {
        match post {
            Ok(post) => {
                posts.push(Some(post));
            }
            Err(e) => {
                error!("Error fetching post: {:?}", e);
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

    info!("Fetching post with id: {:?}", post_id);

    match collection.find_one(filter, None).await {
        Ok(result) => Ok(result),
        Err(e) => {
            error!("Error fetching post: {:?}", e);
            Err(e)
        }
    }
}
///Получаем все посты из базы данных
pub async fn post_getall_all(collection: &Collection<Post>) -> Result<Vec<Option<Post>>, Error> {
    let mut cursor = collection.find(doc! {}, None).await?;

    info!("Fetching all posts");

    let mut posts = Vec::new();
    while let Some(post) = cursor.next().await {
        match post {
            Ok(post) => {
                posts.push(Some(post));
            }
            Err(e) => {
                error!("Error fetching post: {:?}", e);
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

    info!("Fetching posts for page: {}", page);

    let mut cursor = collection.find(doc! {}, options).await?;
    let mut posts = Vec::new();
    while let Some(post) = cursor.next().await {
        match post {
            Ok(post) => {
                posts.push(Some(post));
            }
            Err(e) => {
                error!("Error fetching post: {:?}", e);
                return Err(e);
            }
        }
    }
    Ok(posts)
}
///Получаем все результаты содержащие слова из строки
pub async fn post_search_vague(
    collection: &Collection<Post>,
    search_string: String,
) -> Result<Vec<Option<Post>>, Error> {
    let search_string = search_string
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let filter = doc! {
        "$or": [
            { "label": { "$regex": search_string.join("|"), "$options": "i" } },
            { "underlabel": { "$regex": search_string.join("|"), "$options": "i" } },
            { "text": { "$regex": search_string.join("|"), "$options": "i" } },
            { "footer": { "$regex": search_string.join("|"), "$options": "i" } },
        ]
    };

    info!(
        "Searching vaguely for posts with: {}",
        search_string.join(" ")
    );

    let mut cursor = collection
        .find(filter, FindOptions::builder().build())
        .await?;
    let mut posts = Vec::new();
    while let Some(post) = cursor.next().await {
        match post {
            Ok(post) => {
                posts.push(Some(post));
            }
            Err(e) => {
                error!("Error searching for posts: {:?}", e);
                return Err(e);
            }
        }
    }
    Ok(posts)
}
///Поиск поста по конкретной строчке
pub async fn post_search_fair(
    collection: &Collection<Post>,
    search_string: String,
) -> Result<Vec<Option<Post>>, Error> {
    let filter = doc! {
        "$text": { "$search": &search_string }
    };

    info!("Fair search for posts with: {}", search_string);

    let mut cursor = collection
        .find(filter, FindOptions::builder().build())
        .await?;
    let mut posts = Vec::new();
    while let Some(post) = cursor.next().await {
        match post {
            Ok(post) => {
                posts.push(Some(post));
            }
            Err(e) => {
                error!("Error fair searching for posts: {:?}", e);
                return Err(e);
            }
        }
    }
    Ok(posts)
}

/// Поиск поста расплывчато по страницам
pub async fn post_search_vague_page(
    collection: &Collection<Post>,
    search_string: String,
    page: usize,
) -> Result<Vec<Option<Post>>, Error> {
    let skiprange = match page {
        0 => 0,
        1 => 0,
        _ => (page - 1) * 10,
    };
    let limitrange = (skiprange + 10) as i64;

    let search_string = search_string
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let filter = doc! {
        "$or": [
            { "label": { "$regex": search_string.join("|"), "$options": "i" } },
            { "underlabel": { "$regex": search_string.join("|"), "$options": "i" } },
            { "text": { "$regex": search_string.join("|"), "$options": "i" } },
            { "footer": { "$regex": search_string.join("|"), "$options": "i" } },
        ]
    };

    let options = mongodb::options::FindOptions::builder()
        .skip(skiprange as u64)
        .limit(limitrange)
        .build();

    info!(
        "Vague search for posts with: {}, page: {}",
        search_string.join(" "),
        page
    );

    let mut cursor = collection.find(filter, options).await?;
    let mut posts = Vec::new();
    while let Some(post) = cursor.next().await {
        match post {
            Ok(post) => {
                posts.push(Some(post));
            }
            Err(e) => {
                error!("Error vague searching for posts: {:?}", e);
                return Err(e);
            }
        }
    }
    Ok(posts)
}

///Поиск поста по конкретной строчке постранично
pub async fn post_search_fair_page(
    collection: &Collection<Post>,
    search_string: String,
    page: usize,
) -> Result<Vec<Option<Post>>, Error> {
    let skiprange = match page {
        0 => 0,
        1 => 0,
        _ => (page - 1) * 10,
    };
    let limitrange = (skiprange + 10) as i64;

    let filter = doc! {
        "$text": { "$search": &search_string }
    };

    let options = mongodb::options::FindOptions::builder()
        .skip(skiprange as u64)
        .limit(limitrange)
        .build();

    info!(
        "Fair search for posts with: {}, page: {}",
        search_string, page
    );

    let mut cursor = collection.find(filter, options).await?;
    let mut posts = Vec::new();
    while let Some(post) = cursor.next().await {
        match post {
            Ok(post) => {
                posts.push(Some(post));
            }
            Err(e) => {
                error!("Error fair searching for posts: {:?}", e);
                return Err(e);
            }
        }
    }
    Ok(posts)
}
