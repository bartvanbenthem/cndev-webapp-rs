use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

pub async fn contact(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let body_data = hb
        .render(
            "contact",
            &json!({
                "title": "Contact",
            }),
        )
        .unwrap();

    let contact = json!({
        "body": body_data.to_owned(),
    });

    let body = hb.render("layout", &contact).unwrap();
    HttpResponse::Ok().body(body)
}
