use serde::{Deserialize, Serialize};
use mongodb::bson::Bson;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    //why we need this for user???
    //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    //id: Option<ObjectId>,
    pub name: String,
    pub password: String,
    pub email: String,
    #[serde(default = "default_role")]
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    Admin,
    Paid,
    Default,
    // Add more roles as needed
}

fn default_role() -> Role {
    Role::Default // Установить роль по умолчанию на "User"
}
impl Role {
    pub fn from (s: String) -> Self {
        match s.as_str() {
            "Admin" => Role::Admin,
            "Paid" => Role::Paid,
            _ => Role::Default,
        }
    }
    //we try catch any type of string from frontend and convert this to bson
    pub fn convert_role_to_bson(role: Role) -> Bson {
        match role {
            Role::Admin => Bson::String("Admin".to_string()),
            Role::Paid => Bson::String("Paid".to_string()),
            Role::Default => Bson::String("Default".to_string()),
        }
    }
    
}

