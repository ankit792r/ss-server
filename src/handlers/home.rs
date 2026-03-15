use actix_web::{HttpResponse, Responder, get, web};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct HomeData {}

#[get("/")]
pub async fn home(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    match hb.render("index", &HomeData {}) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            eprintln!("Render error: {:?}", e);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}

#[get("/about")]
pub async fn about() -> impl Responder {
    HttpResponse::Ok().body("about")
}
