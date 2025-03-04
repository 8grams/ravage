use actix_web::{HttpResponse, Responder, Scope, web};

use crate::app_state::AppState;

async fn handler(state: web::Data<AppState>, session: actix_session::Session) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = state.tera.render("pages/login.html", &ctx).unwrap();
    let session = session.get::<serde_json::Value>("session").unwrap();
    if session.is_some() {
        return HttpResponse::Found()
            .append_header(("Location", "/"))
            .append_header(("Hx-Location", "/"))
            .body("Ok");
    }
    HttpResponse::Ok().body(rendered)
}

pub fn login_page() -> Scope {
    web::scope("login").route("", web::get().to(handler))
}
