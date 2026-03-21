use crate::defs::DynStore;
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self},
};
use handlebars::Handlebars;
use rand::{self, RngExt};
use serde::{Deserialize, Serialize};

pub fn generate_key() -> String {
    let code: u32 = rand::rng().random_range(0..=999999);
    format!("{:06}", code)
}

#[derive(Serialize)]
pub struct HomePageData {}

#[get("/")]
pub async fn index(hb: web::Data<Handlebars<'static>>) -> impl Responder {
    let body = hb.render("index", &HomePageData {}).unwrap(); // FIXME: remove unwrap
    HttpResponse::Ok().body(body)
}

#[get("/about")]
pub async fn about(hb: web::Data<Handlebars<'static>>) -> impl Responder {
    let body = hb.render("about", &HomePageData {}).unwrap(); // FIXME: remove unwrap
    HttpResponse::Ok().body(body)
}

#[get("/faqs")]
pub async fn faqs(hb: web::Data<Handlebars<'static>>) -> impl Responder {
    let body = hb.render("faqs", &HomePageData {}).unwrap(); // FIXME: remove unwrap
    HttpResponse::Ok().body(body)
}

#[derive(Serialize, Deserialize)]
pub struct SendObjectJsonRequestData {
    object: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendObjectJsonResponseData {
    success: bool,
    key: Option<String>,
}

#[post("/send")]
pub async fn send_object(
    json: web::Json<SendObjectJsonRequestData>,
    store: web::Data<DynStore>,
) -> impl Responder {
    let object = json.object.clone();
    let new_key = generate_key();
    store.set(new_key.clone(), object, Some(300)).await.unwrap();
    HttpResponse::Ok().json(SendObjectJsonResponseData {
        key: Some(new_key),
        success: true,
    })
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveObjectJsonRequestData {
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveObjectJsonResponseData {
    success: bool,
    object: Option<String>,
    error: Option<String>,
}
#[post("/retrieve")]
pub async fn retrieve_object(
    json: web::Json<RetrieveObjectJsonRequestData>,
    store: web::Data<DynStore>,
) -> impl Responder {
    let key = json.key.clone();
    if let Ok(Some(o)) = store.get(&key).await {
        HttpResponse::Ok().json(&RetrieveObjectJsonResponseData {
            success: true,
            object: Some(o),
            error: None,
        })
    } else {
        HttpResponse::NotFound().json(&RetrieveObjectJsonResponseData {
            success: false,
            object: None,
            error: Some(format!("Object not found")),
        })
    }
}

#[get("/count")]
pub async fn count(store: web::Data<DynStore>) -> impl Responder {
    let total = store.count().await.unwrap();
    HttpResponse::Ok().body(format!("{}", total))
}
