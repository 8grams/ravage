use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

use crate::{app_state::AppState, services::get_collection::get_main_collections};

#[derive(Deserialize)]
pub struct QueryParams {
    collection_id: i32,
}

pub async fn new_request(
    state: web::Data<AppState>,
    params: web::Query<QueryParams>,
    session: actix_session::Session,
) -> impl Responder {
    let query_params = params.into_inner();
    let conn = &mut state.pool.get().unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("COLLECTION_ID", &query_params.collection_id);
    let collections = get_main_collections(conn).await.unwrap();
    ctx.insert("collections", &collections);
    let rendered = state.tera.render("pages/requests/form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
