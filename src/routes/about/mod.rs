use crate::configuration::AppState;
use crate::utils::FileSystemCLient;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug, Clone)]
struct About {
    title: String,
    content: String,
}

pub async fn about(app: web::Data<AppState>) -> HttpResponse {
    let file = format!("{}/about/data.json", &app.settings.content_path.clone());

    let body_data = FileSystemCLient::get_file(&file).await;
    let about: About = serde_json::from_value(body_data).expect("Failed to deserialize JSON");

    let body_data = app
        .template
        .render(
            "about",
            &json!({
                "title": about.title,
                "content": about.content,
            }),
        )
        .unwrap();

    let about = json!({
        "body": body_data.to_owned(),
    });

    let body = app.template.render("layout", &about).unwrap();
    HttpResponse::Ok().body(body)
}
