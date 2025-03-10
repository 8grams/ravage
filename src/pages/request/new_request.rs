use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct QueryParams {
    collection_id: i32,
}

pub async fn new_request(
    state: web::Data<AppState>,
    params: web::Query<QueryParams>,
) -> impl Responder {
    let query_params = params.into_inner();
    let mut ctx = tera::Context::new();
    ctx.insert("COLLECTION_ID", &query_params.collection_id);
    let rendered = state.tera.render("pages/requests/new.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
