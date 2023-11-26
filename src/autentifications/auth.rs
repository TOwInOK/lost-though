use crate::mongolinks::cget::get_connection_users;
use crate::user::user::User;
use crate::user::user_get;
use serde::{Deserialize, Serialize};

//Структура приходящая от клиента, при минимальном запросе.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Auth {
    pub name: String,
    password: String,
}
//Не смог релизовать JWT в рамках проекта, поэтому просто храним пару имени и пароля
//В будующем заменить пароль на зашифрованный с солью
//Добавить Oauth2 и JWT (либо бисквит)

//Реализация для Auth
impl Auth {
    pub fn new(name: String, password: String) -> Self {
        Self { name, password }
    }
    ///Сравниваем полученные строки с стороками в базе
    pub async fn validate(&self) -> bool {
        let collection = get_connection_users().await;
        match user_get(&collection, &self.name).await {
            Ok(Some(user)) => user.password == self.password,
            Ok(None) => false,
            Err(_) => false,
        }
    }
}

//Доп имплементация чтобы не городить костыли.
impl User {
    pub fn validate(&self, another: &User) -> bool {
        self.password == another.password
    }
}
