use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use super::comment::Comment;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: Vec<String>,
    pub date: DateTime,
    pub underlabel: String,
    pub label: String,
    pub text: String,
    pub footer: String,
    pub tags: Vec<String>,
    pub comments: Vec<Comment>,
}

// impl Post {
//     pub fn new(
//         id: Option<ObjectId>,
//         author: String,
//         label: String,
//         underlabel: String,
//         text: String,
//         footer: String,
//         tags: String,
//     ) -> Self {
//         let tags = tags.split(',').map(|s| s.to_string()).collect();
//         Self {
//             id,
//             author,
//             label,
//             underlabel,
//             text,
//             footer,
//             tags,
//         }
//     }
// }


