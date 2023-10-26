use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: String,
    pub label: String,
    pub underlabel: String,
    pub text: String,
    pub footer: String,
    pub tags: Vec<String>,
    pub comments: Vec<String>,
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
