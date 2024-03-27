use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

pub async fn about(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let body_data = hb
        .render(
            "about",
            &json!({
                "title": "About",
            }),
        )
        .unwrap();

    let about = json!({
        "body": body_data.to_owned(),
    });

    let body = hb.render("layout", &about).unwrap();
    HttpResponse::Ok().body(body)
}
