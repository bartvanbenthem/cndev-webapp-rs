use crate::configuration::{AppState, MailSettings};
use crate::email::{send_email, FormData};
use crate::utils::FileSystemCLient;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug, Clone)]
struct Contact {
    title: String,
    header1: String,
}

pub async fn contact(app: web::Data<AppState>) -> HttpResponse {
    let file = format!("{}/contact/data.json", &app.settings.content_path.clone());

    let body_data = FileSystemCLient::get_file(&file).await;
    let about: Contact = serde_json::from_value(body_data).expect("Failed to deserialize JSON");

    let body_data = app
        .template
        .render(
            "contact",
            &json!({
                "title": about.title,
                "header1": about.header1,
            }),
        )
        .unwrap();

    let contact = json!({
        "body": body_data.to_owned(),
    });

    let body = app.template.render("layout", &contact).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn contact_post(
    form_data: web::Form<FormData>,
    app: web::Data<AppState>,
) -> HttpResponse {
    let config = app.settings.clone();

    // mail settings from config
    let config = MailSettings {
        smtp_host: String::from(config.mail.smtp_host.to_owned()),
        smtp_port: config.mail.smtp_port.to_owned(),
        smtp_user: String::from(config.mail.smtp_user.to_owned()),
        smtp_password: String::from(config.mail.smtp_password.to_owned()),
        mail_address: String::from(config.mail.mail_address.to_owned()),
        tls: config.mail.tls,
    };

    // Call the send_email function with the mock data
    let email_response = send_email(web::Form(form_data.clone()), config.clone()).await;

    if email_response.status().is_success() {
        // If sending email was successful, prepare the response with a success page
        let body_data = app
            .template
            .render(
                "contact.response",
                &json!({
                    "from": form_data.name,
                    "to": config.mail_address,
                }),
            )
            .unwrap();

        let contact = json!({
            "body": body_data.to_owned(),
        });

        let body = app.template.render("layout", &contact).unwrap();
        HttpResponse::Ok().body(body)
    } else {
        // If sending email failed, return an error page
        HttpResponse::InternalServerError().body("Failed to send email")
    }
}
