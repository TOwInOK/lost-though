use std::error::Error;
use crate::Cli;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::random;

//учетные данные



pub async fn send_password_code(email_to: String) -> Result<(), Box<dyn Error>> {
    let code: u16 = random();

    let login: String = Cli::smtp_login().await;
    let password: String = Cli::smtp_password().await;

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
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
