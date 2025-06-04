use crate::{app_state::AppState, utils::tera_context::base_context};
use actix_web::{HttpResponse, Responder, web};

pub async fn main(data: web::Data<AppState>, session: actix_session::Session) -> impl Responder {
    let ctx = base_context(&session);
    let rendered = data.tera.render("pages/404.html", &ctx).unwrap();
    HttpResponse::NotFound().body(rendered)
}
