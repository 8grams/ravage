use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

use crate::{app_state::AppState, utils::tera_context::base_context};

#[derive(Deserialize)]
pub struct QueryParams {
    collection_id: i32,
    request_id: Option<i32>,
}

pub async fn get_load_test_from(
    state: web::Data<AppState>,
    params: web::Query<QueryParams>,
    session: actix_session::Session,
) -> impl Responder {
    let query_params = params.into_inner();
    let mut ctx = base_context(&session);
    ctx.insert("COLLECTION_ID", &query_params.collection_id);
    if let Some(request_id) = query_params.request_id {
        ctx.insert("REQUEST_ID", &request_id);
    }
    let rendered = state
        .tera
        .render("components/load_tests/modal_form.html", &ctx)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}
