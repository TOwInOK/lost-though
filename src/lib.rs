pub mod autentifications;
pub mod comments;
pub mod mongolinks;
pub mod posts;
pub mod user;

use crate::user::user::User;
use crate::user::user::Role;
use mongodb::results::UpdateResult;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::options::UpdateOptions;
use mongodb::Collection;

pub async fn change_pid(
    collection: &Collection<User>,
    name: String,
    role: Role,
    access_key: String,
) -> Result<UpdateResult, Error> {
    const PID_KEY: &str = "123test123";
    let filter = doc! {
        "name": name
    };
    let update = doc! {
        "$set": {
            "role": Role::convert_role_to_bson(role)
        }
    };
    if access_key == PID_KEY {
        collection
        .update_one(filter, update, UpdateOptions::builder().build())
        .await
    }else {
        panic!("Invalid Key")
    }
}

pub async fn be_paid(
    collection: &Collection<User>,
    name: String,
) -> Result<UpdateResult, Error> {
    let filter = doc! {
        "name": name
    };
    let update = doc! {
        "$set": {
            "role": Role::convert_role_to_bson(Role::Paid)
        }
    };
    collection
    .update_one(filter, update, UpdateOptions::builder().build())
    .await
}

pub async fn un_paid(
    collection: &Collection<User>,
    name: String,
) -> Result<UpdateResult, Error> {
    let filter = doc! {
        "name": name
    };
    let update = doc! {
        "$set": {
            "role": Role::convert_role_to_bson(Role::Default)
        }
    };
    collection
    .update_one(filter, update, UpdateOptions::builder().build())
    .await
}

pub async fn is_admin(
    collection: &Collection<User>,
    name: String,
) -> bool {
    let filter = doc! {
        "name": name,
        "role": Role::convert_role_to_bson(Role::Admin),
    };
    match collection.find_one(filter, None).await {
        Ok(result) => true,
        Err(e) => false,
    }
}