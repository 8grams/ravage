use actix_web::{HttpResponse, Responder, Scope, web};

use crate::app_state::AppState;

async fn handler(state: web::Data<AppState>) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = state.tera.render("pages/login.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub fn login_page() -> Scope {
    web::scope("login").route("", web::get().to(handler))
}
