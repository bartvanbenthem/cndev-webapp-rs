use crate::configuration::AppState;
use crate::utils::{FileSystemCLient, FileSystemResponse};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
struct Service {
    id: String,
    title: String,
    description: String,
    content: String,
    category: String,
}

pub async fn services(app: web::Data<AppState>) -> HttpResponse {
    let path = format!("{}/services", &app.settings.content_path.clone());

    let services: Vec<FileSystemResponse> = match FileSystemCLient::list_files(&path).await {
        Ok(services) => services,
        Err(err) => {
            // Handle the error gracefully
            eprintln!("Error fetching services: {}", err);
            // Return an empty Vec or any default value depending on your logic
            Vec::new()
        }
    };

    let mut content: Vec<Service> = Vec::new();
    for service in services {
        let body_data = FileSystemCLient::get_file(&service.path).await;
        let home: Service = serde_json::from_value(body_data).expect("Failed to deserialize JSON");
        content.push(home);
    }

    let mut data_json = json!({});
    for i in 0..content.len() {
        let index = i + 1;
        let item = &content[i];
        data_json["id_".to_owned() + &index.to_string()] = json!(item.id);
        data_json["title_".to_owned() + &index.to_string()] = json!(item.title);
        data_json["description_".to_owned() + &index.to_string()] = json!(item.description);
    }

    let body_data = app.template.render("services", &data_json).unwrap();

    let service = json!({
        "body": body_data,
    });

    let body = app.template.render("layout", &service).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn service(app: web::Data<AppState>, id: web::Path<(String,)>) -> HttpResponse {
    let id = id.into_inner().0;

    let path = format!("{}/services", &app.settings.content_path.clone());

    let services: Vec<FileSystemResponse> = match FileSystemCLient::list_files(&path).await {
        Ok(posts) => posts,
        Err(err) => {
            // Handle the error gracefully
            eprintln!("Error fetching services: {}", err);
            // Return an empty Vec or any default value depending on your logic
            Vec::new()
        }
    };

    let mut content = Service::default();
    for service in services {
        let body_data = FileSystemCLient::get_file(&service.path).await;
        let s: Service = serde_json::from_value(body_data).expect("Failed to deserialize JSON");
        if s.id == id {
            content = s;
            break;
        }
    }

    let body_data = app
        .template
        .render(
            "service",
            &json!({
                "id": content.id,
                "title": content.title,
                "description": content.description,
                "content": content.content,
                "category": content.category,
            }),
        )
        .unwrap();

    let service = json!({
        "body": body_data.to_owned(),
    });

    let body = app.template.render("layout", &service).unwrap();
    HttpResponse::Ok().body(body)
}
