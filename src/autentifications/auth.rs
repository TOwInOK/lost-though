use crate::mongolinks::cget::get_connection_users;
use crate::users::user::User;
use crate::users::user_get;
use serde::{Deserialize, Serialize};

//Структура приходящая от клиента, при минимальном запросе.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Auth {
    pub name: String,
    pub password: String,
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
        match user_get(collection, self.name.clone()).await {
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
    pub fn validate_anonimus(&self, another: &Auth) -> bool {
        self.password == another.password
    }
}
