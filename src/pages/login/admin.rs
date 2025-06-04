use actix_web::{HttpResponse, Responder, web};

use crate::app_state::AppState;

pub async fn admin_login_page(
    state: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = state.tera.render("pages/login/admin.html", &ctx).unwrap();
    let session = session.get::<serde_json::Value>("session").unwrap();
    if session.is_some() {
        return HttpResponse::Found()
            .append_header(("Location", "/"))
            .append_header(("Hx-Location", "/"))
            .body("Ok");
    }
    HttpResponse::Ok().body(rendered)
}
