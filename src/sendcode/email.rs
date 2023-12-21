use crate::Cli;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use rand::random;
use redis::Commands;
use std::{error::Error, fmt};

///get req for creat code and put it in redis
pub async fn send_password_code(email_to: String, name: String) -> Result<(), Box<dyn Error>> {
    let code: u16 = random();
    let connection = Cli::redis_adress_simple().await;
    let client = redis::Client::open(connection)?;
    let mut con = client.get_connection()?;
    let login: String = Cli::smtp_login().await;
    let password: String = Cli::smtp_password().await;

    //check code
    let check_code: Option<u16> = con.hget(&name, "code")?;

    if check_code.is_some() {
        return Err(Box::new(CodeError::new("Code has already been created")));
    }

    let email = Message::builder()
        .from("monotipe. <TOwInOK@nothub.ru>".parse().unwrap())
        .to(email_to.parse().unwrap())
        .subject("Code for suguest")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Your code {}", code))
        .unwrap();

    let creds = Credentials::new(login.to_owned(), password.to_owned());

    let mailer = SmtpTransport::relay("smtp.yandex.ru")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            con.hset(&name, "code", code)?;
            con.expire(&name, 300)?;
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}

///Check redis by code for name
pub async fn check_code(code: usize, name: String) -> Result<(), Box<dyn Error>> {
    let connection = Cli::redis_adress_simple().await;
    let client = redis::Client::open(connection)?;
    let mut con = client.get_connection()?;
    let code2: String = redis::cmd("GET")
        .arg(name)
        .query(&mut con)
        .expect("Redis DataBase error");
    if code2 == code.to_string() {
        Ok(())
    } else {
        Err(Box::new(CodeError::new("code is't same")))
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
