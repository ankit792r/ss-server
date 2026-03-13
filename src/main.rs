use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use handlebars::{DirectorySourceOptions, Handlebars};

use crate::handler::home::{about, home};

mod defs;
mod handler;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory("template", DirectorySourceOptions::default())
        .unwrap();
    let hb_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(hb_ref.clone())
            .service(health)
            .service(home)
            .service(about)
    })
    .bind(("127.0.0.1", 4096))?
    .run()
    .await
}
