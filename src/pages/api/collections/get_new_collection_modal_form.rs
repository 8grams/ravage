use actix_web::{HttpResponse, Responder, web};

use crate::app_state::AppState;

pub async fn new_collection_modal_form(data: web::Data<AppState>) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = data
        .tera
        .render("components/collections/collection_modal.html", &ctx)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}
