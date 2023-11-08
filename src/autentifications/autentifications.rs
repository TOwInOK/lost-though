use serde::{Deserialize, Serialize};

use crate::mongolinks::cget::get_connection_users;
use crate::user::user_get;
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Auth {
    name: String,
    password: String,
}
//нет времени на реализацию jwt или любой человеческой реализации.
impl Auth {
    pub fn new(name: String, password: String) -> Self {
        Self {
            name,
            password,
        }
    }
    pub async fn validate(&self) -> bool {
        let collection = get_connection_users().await;
        match user_get(&collection, self.name.clone()).await {
            Ok(Some(user)) => user.password == self.password,
            Ok(None) => false,
            Err(_) => false,
        }
    }
}