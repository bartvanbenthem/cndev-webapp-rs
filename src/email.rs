use actix_web::{web, HttpResponse};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;

use crate::configuration::*;

#[derive(Deserialize, Debug, Clone)]
pub struct FormData {
    pub name: String,
    pub from: String,
    pub subject: String,
    pub message: String,
    pub phone: String,
}

pub async fn send_email(form: web::Form<FormData>, config: MailSettings) -> HttpResponse {
    let mail = format!(
        "Name: {}\nSender: {}\nSubject: {}\n\n{}",
        &form.name, &form.from, &form.subject, &form.message
    );

    let email = Message::builder()
        .from(form.from.parse().expect("Invalid from address"))
        .to(config.mail_address.parse().expect("Invalid to address"))
        .subject(&form.subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(&mail))
        .expect("Failed to build email");

    let creds = Credentials::new(config.smtp_user.to_owned(), config.smtp_password.to_owned());

    let mailer: SmtpTransport;
    if !config.tls {
        mailer = match SmtpTransport::relay(&config.smtp_host) {
            Ok(smtp) => smtp
                .tls(Tls::None)
                .port(config.smtp_port)
                .credentials(creds)
                .build(),
            Err(err) => {
                eprintln!("Failed to create SMTP transport: {}", err);
                return HttpResponse::InternalServerError().body("Failed to send email");
            }
        };
    } else {
        // TODO: Configure TLS !!!!!!!!!!!!!!!!!!!
        mailer = match SmtpTransport::relay(&config.smtp_host) {
            Ok(smtp) => smtp.port(config.smtp_port).credentials(creds).build(),
            Err(err) => {
                eprintln!("Failed to create SMTP transport: {}", err);
                return HttpResponse::InternalServerError().body("Failed to send email");
            }
        };
    }

    // Send the email
    match mailer.send(&email) {
        Ok(_) => HttpResponse::Ok().body("Email sent successfully"),
        Err(err) => {
            eprintln!("Failed to send email: {}", err);
            HttpResponse::InternalServerError().body("Failed to send email")
        }
    }
}
