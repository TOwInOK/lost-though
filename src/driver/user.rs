use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    //why we need this for user???
    //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    //id: Option<ObjectId>,
    pub name: String,
    pub password: String,
    pub email: String,
    // posts: Vec<Option<ObjectId>>,
}

impl User {
    pub fn new(
        //id: Option<ObjectId>,
        name: String,
        password: String,
        email: String,
        // posts: Vec<Option<ObjectId>>,
    ) -> Self {
        Self {
            //id,
            name,
            password,
            email,
            // posts,
        }
    }
}
