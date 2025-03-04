use crate::app_state::AppState;
use actix_web::{HttpResponse, Responder, web};

pub async fn main(data: web::Data<AppState>) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = data.tera.render("404.html", &ctx).unwrap();
    HttpResponse::NotFound().body(rendered)
}
