use crate::{app_state::AppState, models::request::Request, schema::requests};
use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

pub async fn request_form(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let mut ctx = tera::Context::new();
    let req_id = path.into_inner();
    if let Ok(request) = requests::table
        .select(Request::as_select())
        .find(req_id)
        .get_result(conn)
    {
        ctx.insert("request", &request);
    }
    let rendered = state
        .tera
        .render("components/request_form.html", &ctx)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}
