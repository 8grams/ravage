use actix_web::{HttpResponse, Responder, web};

use crate::app_state::AppState;
pub async fn main_pages(data: web::Data<AppState>) -> impl Responder {
    let conn = data.pool.get();
    let ctx = tera::Context::new();
    let rendered = data.tera.render("pages/index.html", &ctx).unwrap();

    HttpResponse::Ok().body(rendered)
}
