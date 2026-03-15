use actix_web::{App, HttpServer, web};
use handlebars::{DirectorySourceOptions, Handlebars};

mod database;
mod defs;
mod handlers;
mod services;

use crate::{
    defs::{KeyUtils, StringObject},
    handlers::{
        home::{about, home},
        share::{object_get, object_get_form, object_post, object_post_form, share},
    },
    services::string_object::StringObjectService,
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

    // let uri = std::env::var("MONGODB_URI")
    //     .unwrap_or_else(|_| "mongodb://root:root@localhost:27017/string-share".into());
    //
    // let mongo_client = Client::with_uri_str(uri)
    //     .await
    //     .expect("Failed to connect to MongoDB");
    //
    // // connection check
    // mongo_client
    //     .database("admin")
    //     .run_command(doc! { "ping": 1 })
    //     .await
    //     .expect("Failed to authenticate with MongoDB");

    let mongo_client = connect_db();

    let ss_db = mongo_client.database("string-share");
    let string_collection = ss_db.collection::<StringObject>("string_objects");

    let key_utils = KeyUtils::new();

    let string_object_service =
        web::Data::new(StringObjectService::new(string_collection, key_utils));

    HttpServer::new(move || {
        App::new()
            .app_data(hb_ref.clone())
            .app_data(string_object_service.clone())
            .service(home)
            .service(about)
            .service(share)
            .service(object_get)
            .service(object_get_form)
            .service(object_post)
            .service(object_post_form)
    })
    .bind(("127.0.0.1", 4096))?
    .run()
    .await
}
