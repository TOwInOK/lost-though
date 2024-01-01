use clap::Parser;
use log::{debug, info, trace, warn};
//Parse args from env.

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
    ///sending from client of smtp if login is different
    #[arg(long = "smtp-adress-from", env = "SMTP_ADDRESS_FROM")]
    smtp_address_from: Option<String>,
}

impl Cli {
    //?? я кстати не помню зачем. Типо асинк вызов
    pub async fn push() -> Self {
        trace!("Pars args from env");
        Cli::parse()
    }
    pub async fn mongo_adress() -> String {
        let cli = Cli::push().await;
        let login = cli.mongo_login.unwrap_or_default();
        let password = cli.mongo_password.unwrap_or_default();
        let auth_part = if !login.is_empty() && !password.is_empty() {
            trace!("auth for mongo db is exist -> {}:{}@", &login, &password);
            format!("{}:{}@", login, password)
        } else {
            trace!("auth for mongo db is't exist");
            String::new()
        };
        let output = format!(
            "mongodb://{}{}:{}",
            auth_part, cli.mongo_address, cli.mongo_port
        );
        trace!("MongoDB address -> {}", output);
        output
    }
    pub async fn web_port() -> u16 {
        let cli = Cli::parse();
        trace!("Fetching: WEB port -> {}", &cli.web_port);
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
    pub async fn smtp() -> (String, String, String, String) {
        let cli = Cli::parse();

        let smtp_address_from = cli.smtp_address_from.map_or_else(
            || cli.smtp_login.clone(),
            |value| {
                if value.trim().is_empty() {
                    cli.smtp_login.clone()
                } else {
                    value
                }
            },
        );

        (
            smtp_address_from,
            cli.smtp_address,
            cli.smtp_login,
            cli.smtp_password,
        )
    }
}
