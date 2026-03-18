use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use handlebars::{DirectorySourceOptions, Handlebars};

mod database;
mod defs;
mod handlers;
mod route;
mod services;
mod store;

use crate::{
    database::mongo_factory::connect_db,
    defs::{DataStore, DynStore, KeyUtils, StringObject},
    handlers::{
        home::{about, home},
        share::{object_get, object_get_form, object_post, object_post_form, share},
    },
    route::{handle_form, index},
    services::string_object::StringObjectService,
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
    let hb_ref = web::Data::new(handlebars);

    let mongo_client = connect_db().await.expect("Failed to connect mongodb");

    let ss_db = mongo_client.database("string-share");
    let string_collection = ss_db.collection::<StringObject>("string_objects");

    let string_object_service = web::Data::new(StringObjectService::new(string_collection));

    let data_store: DynStore = Arc::new(InMemoryStore::new());

    HttpServer::new(move || {
        App::new()
            .app_data(hb_ref.clone())
            .app_data(web::Data::from(data_store.clone()))
            // .app_data(string_object_service.clone())
            // .service(home)
            .service(index)
            .service(handle_form)

        // .service(about)
        // .service(share)
        // .service(object_get)
        // .service(object_get_form)
        // .service(object_post)
        // .service(object_post_form)
    })
    .bind(("127.0.0.1", 4096))?
    .run()
    .await
}
