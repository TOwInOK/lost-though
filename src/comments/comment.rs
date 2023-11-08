use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: String,
    pub text: String,
    #[serde(rename = "reject", skip_serializing_if = "Option::is_none")]
    pub reject: Option<ObjectId>,
}
