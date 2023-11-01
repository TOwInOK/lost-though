use super::comment::Comment;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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