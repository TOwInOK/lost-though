use crate::comments::comment::Comment;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
///Создание поста
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: Vec<String>,
    pub date: u64,
    pub underlabel: String,
    pub label: String,
    pub text: String,
    pub footer: String,
    pub tags: Vec<String>,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
///Создание поста не включает в себя:
///ID, comments<Vec>
pub struct PostCreate {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: Vec<String>,
    pub underlabel: String,
    pub label: String,
    pub text: String,
    pub footer: String,
    pub tags: Vec<String>,
}
