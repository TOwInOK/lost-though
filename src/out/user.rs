use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct User {
    pub name: String,
    password: String,
    email: String,
    posts: Vec<ObjectId>,
}

impl User {
    pub fn new(name: String, password: String, email: String, posts: Vec<ObjectId>) -> Self {
        Self {
            name,
            password,
            email,
            posts,
        }
    }
}
