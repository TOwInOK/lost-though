pub mod autentifications;
pub mod comments;
pub mod mongolinks;
pub mod posts;
pub mod user;
use crate::user::user::Role;
use crate::user::user::User;
use clap::Parser;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::options::UpdateOptions;
use mongodb::results::UpdateResult;
use mongodb::Collection;

//Функции для взаимодействия с role. Пока никак не используются.
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
    } else {
        panic!("Invalid Key")
    }
}

pub async fn be_paid(collection: &Collection<User>, name: String) -> Result<UpdateResult, Error> {
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

pub async fn un_paid(collection: &Collection<User>, name: String) -> Result<UpdateResult, Error> {
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

pub async fn is_admin(collection: &Collection<User>, name: String) -> bool {
    let filter = doc! {
        "name": name,
        "role": Role::convert_role_to_bson(Role::Admin),
    };
    match collection.find_one(filter, None).await {
        Ok(_result) => true,
        Err(_e) => false,
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    ///Port for web
    #[arg(short = 'w', long = "port", default_value_t = 8080)]
    web_port: u16,
    ///Adress for mongo db
    #[arg(short = 'a', long = "adress", default_value_t = format!("0.0.0.0"))]
    adress: String,
    ///Login for auth into db (mongo)
    #[arg(short = 'l', long = "mlogin")]
    mongo_login: Option<String>,
    ///Password for auth into db (mongo)
    #[arg(short = 'p', long = "mpassword")]
    mongo_password: Option<String>,
    ///Port for db (mongo)
    #[arg(short = 'm', long = "port", default_value_t = 27017)]
    mongo_port: u16,
}

impl Cli {
    pub async fn push() -> Self {
        let cli = Cli::parse();
        cli
    }
    pub async fn mongoadress() -> String {
        let cli = Cli::parse();
        let output = format!(
            "mongodb://{}:{}@{}:{}",
            cli.mongo_login.unwrap_or_default(),
            cli.mongo_password.unwrap_or_default(),
            cli.adress,
            cli.mongo_port
        );
        output
    }
    pub async fn web_port() -> u16 {
        let cli = Cli::parse();
        cli.web_port
    }
}
