use crate::configuration::AppState;
use crate::utils::{FileSystemCLient, FileSystemResponse, Helpers};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
struct Post {
    id: String,
    title: String,
    content: String,
}

pub async fn home(app: web::Data<AppState>) -> HttpResponse {
    let header_data = app
        .template
        .render(
            "carousel",
            &json!({
                "variable": null,
            }),
        )
        .unwrap();

    let path = format!("{}/posts", &app.settings.content_path.clone());

    let posts: Vec<FileSystemResponse> = match FileSystemCLient::list_files(&path).await {
        Ok(posts) => posts,
        Err(err) => {
            // Handle the error gracefully
            eprintln!("Error fetching posts: {}", err);
            // Return an empty Vec or any default value depending on your logic
            Vec::new()
        }
    };

    let recent = Helpers::get_last_number_of_items(posts, 4);

    let mut content: Vec<Post> = Vec::new();
    for post in recent {
        let body_data = FileSystemCLient::get_file(&post.path).await;
        let home: Post = serde_json::from_value(body_data).expect("Failed to deserialize JSON");
        content.push(home);
    }

    let mut data_json = json!({});

    for i in (0..4).rev() {
        let index = 4 - i;
        let item = &content[i];
        data_json["id_".to_owned() + &index.to_string()] = json!(item.id);
        data_json["title_".to_owned() + &index.to_string()] = json!(item.title);
        data_json["content_".to_owned() + &index.to_string()] = json!(format!(
            "{}{}",
            Helpers::get_first_number_of_words(&item.content, 30),
            "..."
        ));
    }

    let body_data = app.template.render("home", &data_json).unwrap();

    let home = json!({
        "header": header_data,
        "body": body_data,
    });

    let body = app.template.render("layout", &home).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn post(app: web::Data<AppState>, id: web::Path<(String,)>) -> HttpResponse {
    let id = id.into_inner().0;

    let path = format!("{}/posts", &app.settings.content_path.clone());

    let posts: Vec<FileSystemResponse> = match FileSystemCLient::list_files(&path).await {
        Ok(posts) => posts,
        Err(err) => {
            // Handle the error gracefully
            eprintln!("Error fetching posts: {}", err);
            // Return an empty Vec or any default value depending on your logic
            Vec::new()
        }
    };

    let mut content = Post::default();
    for post in posts {
        let body_data = FileSystemCLient::get_file(&post.path).await;
        let p: Post = serde_json::from_value(body_data).expect("Failed to deserialize JSON");
        if p.id == id {
            content = p;
            break;
        }
    }

    let body_data = app
        .template
        .render(
            "post",
            &json!({
                "content": content.content,
            }),
        )
        .unwrap();

    let post = json!({
        "body": body_data.to_owned(),
    });

    let body = app.template.render("layout", &post).unwrap();
    HttpResponse::Ok().body(body)
}
