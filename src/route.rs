use crate::defs::{Action, DynStore, FormData};
use actix_web::{HttpResponse, Responder, get, post, web};
use handlebars::Handlebars;
use rand::{self, RngExt};
use serde::Serialize;

pub fn generate_key() -> String {
    let code: u32 = rand::rng().random_range(0..=999999);
    format!("{:06}", code)
}

pub fn parse_action(form: &FormData) -> Result<Action, &'static str> {
    match (&form.object, &form.key) {
        (Some(t), None) if !t.trim().is_empty() => Ok(Action::Send(t.clone())),
        (None, Some(k)) if !k.trim().is_empty() => Ok(Action::Retrieve(k.clone())),
        (Some(_), Some(_)) => Err("Provide either text OR key, not both."),
        _ => Err("Invalid input."),
    }
}

#[derive(Serialize)]
struct IndexTemplate {
    error: Option<String>,
    success: Option<String>,
    text: Option<String>,
    key: Option<String>,
}

#[post("/")]
pub async fn handle_form(
    form: web::Form<FormData>,
    hb: web::Data<Handlebars<'_>>,
    store: web::Data<DynStore>,
) -> impl Responder {
    println!("{:?}", &form);
    match parse_action(&form) {
        Ok(Action::Send(text)) => {
            let new_key = generate_key();
            store.set(new_key.clone(), text, Some(300)).await.unwrap();
            let body = hb
                .render(
                    "index",
                    &IndexTemplate {
                        key: Some(new_key.clone()),
                        error: None,
                        success: Some(String::from("")),
                        text: None,
                    },
                )
                .unwrap();
            HttpResponse::Ok().body(body)
        }
        Ok(Action::Retrieve(key)) => {
            if let Ok(Some(val)) = store.get(&key).await {
                store.delete(&key).await.unwrap();
                let body = hb
                    .render(
                        "index",
                        &IndexTemplate {
                            key: None,
                            error: None,
                            success: None,
                            text: Some(val.clone()),
                        },
                    )
                    .unwrap();
                HttpResponse::Ok().body(body)
            } else {
                let body = hb
                    .render(
                        "index",
                        &IndexTemplate {
                            key: None,
                            error: Some(String::from("Object not found")),
                            success: None,
                            text: None,
                        },
                    )
                    .unwrap();
                HttpResponse::Ok().body(body)
            }
        }
        Err(msg) => {
            let body = hb
                .render(
                    "index",
                    &IndexTemplate {
                        key: None,
                        error: Some(String::from(msg)),
                        success: None,
                        text: None,
                    },
                )
                .unwrap();
            HttpResponse::Ok().body(body)
        }
    }
}

#[get("/")]
pub async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = IndexTemplate {
        error: None,
        success: None,
        text: None,
        key: None,
    };

    let body = hb.render("index", &data).unwrap(); // FIXME: remove unwrap
    HttpResponse::Ok().body(body)
}
