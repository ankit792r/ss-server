use actix_web::{App, HttpServer, web};
use handlebars::{DirectorySourceOptions, Handlebars};

use crate::handler::{home::{about, home}, share::share};

mod defs;
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    let mut options = DirectorySourceOptions::default();
    options.tpl_extension = ".html".to_string();

    handlebars
        .register_templates_directory("template", options)
        .unwrap();
    let hb_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(hb_ref.clone())
            .service(home)
            .service(about)
            .service(share)
    })
    .bind(("127.0.0.1", 4096))?
    .run()
    .await
}
