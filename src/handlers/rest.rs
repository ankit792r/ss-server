use actix_web::{
    HttpResponse, Responder, post,
    web::{self, Json},
};
use serde::{Deserialize, Serialize};

use crate::services::string_object::StringObjectService;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ObjectPostData {
    content: String,
}

#[post("/api/string/post")]
pub async fn create_object(
    dt: Json<ObjectPostData>,
    ss: web::Data<StringObjectService>,
) -> impl Responder {
    let raw = &dt.content;
    let res = ss.create_new_object(raw.into()).await;

    match res {
        Ok(d) => HttpResponse::Created().json(d),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}


#[post("/api/string/list")]
pub async fn list_objects(
    ss: web::Data<StringObjectService>,
) -> impl Responder {
    ss.get
}
