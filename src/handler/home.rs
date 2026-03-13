use actix_web::{HttpResponse, Responder, get, web};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct HomeData {}

#[get("/")]
pub async fn home(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let body = hb.render("index.html", &HomeData {}).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/about")]
pub async fn about() -> impl Responder {
    HttpResponse::Ok().body("about")
}
