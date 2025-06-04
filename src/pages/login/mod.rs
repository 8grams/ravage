mod admin;
use crate::app_state::AppState;
use actix_web::{HttpResponse, Responder, Scope, web};
async fn main_login_page(
    state: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = state.tera.render("pages/login/index.html", &ctx).unwrap();
    let session = session.get::<serde_json::Value>("session").unwrap();
    if session.is_some() {
        return HttpResponse::Found()
            .append_header(("Location", "/"))
            .append_header(("Hx-Location", "/"))
            .body("Ok");
    }
    HttpResponse::Ok().body(rendered)
}
pub fn login_scope() -> Scope {
    web::scope("/login")
        .route("/admin", web::get().to(admin::admin_login_page))
        .route("", web::get().to(main_login_page))
}
