use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

pub async fn home(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let header_data = hb
        .render(
            "carousel",
            &json!({
                "button": "voorbeeld",
            }),
        )
        .unwrap();

    let body_data = hb
        .render(
            "home",
            &json!({
                "button": "lees meer",
            }),
        )
        .unwrap();

    let home = json!({
        "header": header_data,
        "body": body_data,
    });

    let body = hb.render("layout", &home).unwrap();
    HttpResponse::Ok().body(body)
}
