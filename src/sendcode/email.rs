use crate::cli::Cli;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use log::{debug, error, info};
use rand::random;
use redis::Commands;
use std::{error::Error, fmt};

///get req for creat code and put it in redis
pub async fn send_password_code(email_to: String, name: String) -> Result<(), Box<dyn Error>> {
    let code: u16 = random();
    let connection = Cli::redis_address_simple().await;
    let client = redis::Client::open(connection)?;
    let mut con = client.get_connection()?;
    let smtp = Cli::smtp().await;
    let (address_from, address, login, password) = smtp;

    //check code
    let check_code: Option<u16> = con.hget(&name, "code")?;
    info!("Check code into redis");
    if check_code.is_some() {
        error!("Code already exist");
        return Err(Box::new(CodeError::new("Code has already been created")));
    }
    info!("Code is't exist");
    debug!(
        "Starting email building | send from {} to {}",
        &address_from, &email_to
    );
    let email = Message::builder()
        .from(format!("monotipe. <{}>", address_from).parse()?)
        .to(email_to.clone().parse().unwrap())
        .subject("Code for suguest")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Your code {}", code))
        .unwrap();
    let creds = Credentials::new(login.to_owned(), password.to_owned());
    let mailer = SmtpTransport::relay(&address.to_string())
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            con.hset(&name, "code", code)?;
            con.expire(&name, 300)?;
            info!("Code send");
            Ok(())
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(Box::new(e))
        }
    }
}

///Check redis by code for name
pub async fn check_code(code: usize, name: String) -> Result<(), Box<dyn Error>> {
    let connection = Cli::redis_address_simple().await;
    let client = redis::Client::open(connection)?;
    let mut con = client.get_connection()?;
    let check_code: Option<u16> = con.hget(&name, "code")?;
    match check_code {
        Some(saved_code) if saved_code == code as u16 => {
            // Код совпал, удаляем его из Redis hash
            con.del(&name)?;
            info!("remove redis code");
            Ok(())
        }
        _ => Err(Box::new(CodeError::new("Code doesn't match"))),
    }
}

#[derive(Debug)]
struct CodeError {
    message: String,
}

impl CodeError {
    fn new(message: &str) -> CodeError {
        CodeError {
            message: message.to_string(),
        }
    }
}

impl Error for CodeError {}

impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
