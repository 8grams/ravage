use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

use crate::{app_state::AppState, models::request::Request, schema::requests};

pub async fn get_collection_requests(
    state: web::Data<AppState>,
    c_id: web::Path<i32>,
) -> impl Responder {
    let collection_id = c_id.into_inner();
    let conn = &mut state.pool.get().unwrap();
    let mut ctx = tera::Context::new();
    let requests = requests::table
        .select(Request::as_select())
        .filter(requests::collection_id.assume_not_null().eq(collection_id))
        .get_results(conn)
        .unwrap();
    ctx.insert("requests", &requests);
    let rendered = state
        .tera
        .render("components/requests/list.html", &ctx)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}
