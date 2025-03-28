use actix_web::{HttpResponse, Responder, web};

use crate::app_state::AppState;
pub async fn test_page(state: web::Data<AppState>) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = state.tera.render("pages/test.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
