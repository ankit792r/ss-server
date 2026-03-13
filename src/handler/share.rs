use actix_web::{HttpResponse, Responder, get, web};
use handlebars::Handlebars;
use serde::Serialize;



#[derive(Serialize)]
struct ShareData {}

#[get("/share")]
pub async fn share(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    match hb.render("share", &ShareData {}) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            eprintln!("Render error: {:?}", e);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}
