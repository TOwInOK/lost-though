pub mod autentifications;
pub mod comments;
pub mod mongolinks;
pub mod posts;
pub mod sendcode;
pub mod users;
use crate::users::user::Role;
use crate::users::user::User;
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

/// Get args from env
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    ///WEB PORT
    #[arg(short = 'p', long = "port", default_value_t = 8080)]
    web_port: u16,
    ///MongoDB
    ///Adress for mongo db
    #[arg(short = 'a', long = "mongo-address", default_value_t = format!("127.0.0.1"), env = "MONGO_ADDRESS")]
    mongo_address: String,
    ///Login for auth into db (mongo)
    #[arg(long = "mongo-login", env = "MONGO_LOGIN")]
    mongo_login: Option<String>,
    ///Password for auth into db (mongo)
    #[arg(long = "mongo-password", env = "MONGO_PASSWORD")]
    mongo_password: Option<String>,
    ///Port for db (mongo)
    #[arg(long = "mongo-port", default_value_t = 27017, env = "MONGO_PORT")]
    mongo_port: u16,

    ////REDIS
    ///Login for auth into db (redis)
    #[arg(long = "redis-login", env = "REDIS_LOGIN")]
    redis_login: Option<String>,
    ///Password for auth into db (mongo)
    #[arg(long = "redis-password", env = "REDIS_PASSWORD")]
    redis_password: Option<String>,
    ///Port for redis
    #[arg(long = "redis-port", default_value_t = 6379, env = "REDIS_PORT")]
    redis_port: u16,
    #[arg(long = "redis-address", default_value_t = format!("127.0.0.1"), env = "REDIS_ADDRESS")]
    redis_address: String,

    ////SMTP
    /// Login smpt
    #[arg(long = "smtp-login", env = "SMTP_LOGIN")]
    smtp_login: String,
    /// Password (or secure code) smtp
    #[arg(long = "smtp-password", env = "SMTP_PASSWORD")]
    smtp_password: String,
    /// adress smpt
    #[arg(long = "smtp-adress", env = "SMTP_ADDRESS")]
    smtp_address: String,
}


impl Cli {
    //?? я кстати не помню зачем. Типо асинк вызов
    pub async fn push() -> Self {
        Cli::parse()
    }
    pub async fn mongo_adress() -> String {
        let cli = Cli::push().await;
        let login = cli.mongo_login.unwrap_or_default();
        let password = cli.mongo_password.unwrap_or_default();
        let auth_part = if !login.is_empty() && !password.is_empty() {
            format!("{}:{}@", login, password)
        } else {
            String::new()
        };
    
        let output = format!(
            "mongodb://{}{}:{}",
            auth_part, cli.mongo_address, cli.mongo_port
        );
        println!("{output}");
        output
    }
    pub async fn web_port() -> u16 {
        let cli = Cli::parse();
        cli.web_port
    }
    pub async fn redis_adress() -> String {
        let cli = Cli::parse();
        let output = format!(
            "redis://{}:{}@{}:{}",
            cli.redis_login.unwrap_or_default(),
            cli.redis_password.unwrap_or_default(),
            cli.redis_address,
            cli.redis_port
        );
        output
    }
    pub async fn redis_adress_simple() -> String {
        let cli = Cli::parse();
        let output = format!("redis://{}/", cli.redis_address);
        output
    }
    pub async fn smtp_login() -> String {
        let cli = Cli::parse();
        cli.smtp_login
    }
    pub async fn smtp_password() -> String {
        let cli = Cli::parse();
        cli.smtp_password
    }
}
