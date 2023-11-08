use serde::{Serialize,Deserialize};
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::error::Error;
use crate::user::user::User;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Auth {
    #[serde(rename = "login")]
    pub name: String,
    #[serde(rename = "pass")]
    pub password: String,
}
pub async fn user_veryficate(collection: &Collection<User>, name: String, password: String) -> Result<(), Error> {
    let filter = doc! {
        "name": name,
        "password": password,
    };
    match collection.find_one(filter, None).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
