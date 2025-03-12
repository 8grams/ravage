use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    services::{
        get_collection::{get_main_collections, get_single_collection},
        get_request::get_collection_requests,
    },
};

#[derive(Deserialize)]
pub struct QueryParams {
    request: Option<i32>,
}

pub async fn new_load_test(
    params: web::Query<QueryParams>,
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let mut ctx = tera::Context::new();
    let collection_id = id.into_inner();
    let query_params = params.into_inner();

    let collections = get_main_collections(conn).await.unwrap();
    ctx.insert("collections", &collections);

    if let Ok(collection) = get_single_collection(conn, collection_id).await {
        ctx.insert("collection", &collection);
        ctx.insert("COLLECTION_ID", &collection_id);

        let requests = get_collection_requests(conn, collection_id).await.unwrap();
        ctx.insert("requests", &requests);

        let request_id = query_params.request.unwrap_or(if !requests.is_empty() {
            requests[0].id
        } else {
            0
        });
        ctx.insert("REQUEST_ID", &request_id);

        let rendered = state
            .tera
            .render("pages/collection/loadtest.html", &ctx)
            .unwrap();
        return HttpResponse::Ok().body(rendered);
    };
    HttpResponse::NotFound().body("404")
}
