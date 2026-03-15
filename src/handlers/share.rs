use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, Redirect},
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::services::string_object::StringObjectService;

#[get("/share")]
pub async fn share() -> impl Responder {
    Redirect::to("/share/post").permanent()
}

#[derive(Serialize)]
struct PostTemplateData {}

#[get("/share/post")]
pub async fn object_post(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    match hb.render("ss-post", &PostTemplateData {}) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            eprintln!("Render error: {:?}", e);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ObjectPostFormData {
    raw_content: String,
}

#[derive(Serialize, Debug, Clone)]
struct FormTemplateData {
    created: bool,
    key: Option<String>,
}

#[post("/share/post")]
pub async fn object_post_form(
    fd: web::Form<ObjectPostFormData>,
    ss: web::Data<StringObjectService>,
    hb: web::Data<Handlebars<'_>>,
) -> impl Responder {
    let raw = &fd.raw_content;

    println!("{}", raw);

    let result = ss.create_new_object(raw.into()).await;

    if let Ok(s) = result {
        println!("{}", s);
        match hb.render(
            "ss-post",
            &FormTemplateData {
                created: true,
                key: Some(s),
            },
        ) {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(e) => {
                eprintln!("Render error: {:?}", e);
                HttpResponse::InternalServerError().body("Template error")
            }
        }
    } else {
        match hb.render(
            "ss-post",
            &FormTemplateData {
                created: false,
                key: None,
            },
        ) {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(e) => {
                eprintln!("Render error: {:?}", e);
                HttpResponse::InternalServerError().body("Template error")
            }
        }
    }
}

#[derive(Serialize)]
struct GetTemplateData {}

#[get("/share/get")]
pub async fn object_get(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    match hb.render("ss-get", &GetTemplateData {}) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            eprintln!("Render error: {:?}", e);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}

#[derive(Serialize)]
struct ObjectGetTemplateData {
    found: bool,
    content: Option<String>, //decoded string object value
}

#[derive(Serialize, Deserialize)]
struct ObjectGetFormData {
    key: String,
}

#[post("/share/get")]
pub async fn object_get_form(
    fd: web::Form<ObjectGetFormData>,
    ss: web::Data<StringObjectService>,
    hb: web::Data<Handlebars<'_>>,
) -> impl Responder {
    let key = &fd.key;

    let result = ss.get_object_from_key(key.to_owned()).await;

    if let Ok(val) = result {
        match hb.render(
            "ss-get",
            &ObjectGetTemplateData {
                found: true,
                content: Some(val.content),
            },
        ) {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(e) => {
                eprintln!("Render error: {:?}", e);
                HttpResponse::InternalServerError().body("Template error")
            }
        }
    } else {
        match hb.render(
            "ss-get",
            &ObjectGetTemplateData {
                found: false,
                content: None,
            },
        ) {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(e) => {
                eprintln!("Render error: {:?}", e);
                HttpResponse::InternalServerError().body("Template error")
            }
        }
    }
}
