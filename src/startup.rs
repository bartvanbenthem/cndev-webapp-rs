use crate::routes::{about, contact, health_check, home};
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let mut hbars = Handlebars::new();
    hbars
        .register_templates_directory(".html", "./static/html/")
        .unwrap();
    let hbars_ref = web::Data::new(hbars);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(hbars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(home))
            .route("/about", web::get().to(about))
            .route("/contact", web::get().to(contact))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
