use crate::configuration::{build_app_state, get_configuration};
use crate::routes;
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let mut hbars = Handlebars::new();
    hbars
        .register_templates_directory(".html", "./static/html/")
        .unwrap();

    let app_state = build_app_state(configuration, hbars);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(Files::new("/static", "static").show_files_listing())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/", web::get().to(routes::home))
            .route("/about", web::get().to(routes::about))
            .route("/services", web::get().to(routes::services))
            .route("/service/{id}", web::get().to(routes::service))
            .route("/contact", web::get().to(routes::contact))
            .route("/contact", web::post().to(routes::contact_post))
            .route("/post/{id}", web::get().to(routes::post))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
