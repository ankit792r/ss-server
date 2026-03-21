use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use handlebars::{DirectorySourceOptions, Handlebars};

mod defs;
mod route;
mod store;

use crate::{
    defs::DynStore,
    route::{about, count, faqs, index, retrieve_object, send_object},
    store::memory::InMemoryStore,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    let mut options = DirectorySourceOptions::default();
    options.tpl_extension = ".html".to_string();

    handlebars
        .register_templates_directory("template", options)
        .expect("Failed to register handlebars");

    let data_store: DynStore = Arc::new(InMemoryStore::new());

    HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/static", "statics"))
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(web::Data::new(data_store.clone()))
            .service(index)
            .service(about)
            .service(faqs)
            .service(count)
            .service(send_object)
            .service(retrieve_object)
    })
    .bind(("127.0.0.1", 4096))?
    .run()
    .await
}
