use actix_web::web;
use cndevwebapp::configuration::{get_configuration, MailSettings};
use cndevwebapp::email::*;

#[tokio::test]
async fn test_mock_mail() {
    let config = get_configuration().expect("Failed to read configuration.");

    // Mock form data
    let form_data = FormData {
        name: String::from("John Doe"),
        from: String::from("sender@mailhog.example"),
        subject: String::from("Test Subject"),
        message: String::from("This is a test email"),
        phone: String::from("0600000010"),
    };

    println!("{:?}", form_data);

    // mail settings from config
    let config = MailSettings {
        smtp_host: String::from(config.mail.smtp_host.to_owned()),
        smtp_port: config.mail.smtp_port.to_owned(),
        smtp_user: String::from(config.mail.smtp_user.to_owned()),
        smtp_password: String::from(config.mail.smtp_password.to_owned()),
        mail_address: String::from(config.mail.mail_address.to_owned()),
        tls: config.mail.tls,
    };

    println!("{:?}", config);

    // Call the send_email function with the mock data
    let resp = cndevwebapp::email::send_email(web::Form(form_data), config).await;
    println!("{:?}", resp)
}
