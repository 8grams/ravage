use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    services::{
        get_collection::{get_collection_headers, get_main_collections, get_single_collection},
        get_request::{get_collection_requests, get_request_headers},
    },
    utils::tera_context::base_context,
};

#[derive(Deserialize)]
pub struct QueryParams {
    request: Option<i32>,
    hide: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct Header {
    pub key: String,
    pub value: String,
}

pub async fn new_load_test(
    params: web::Query<QueryParams>,
    state: web::Data<AppState>,
    id: web::Path<i32>,
    session: actix_session::Session,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let mut ctx = base_context(&session);
    let collection_id = id.into_inner();
    let query_params = params.into_inner();

    if let Some(hide) = query_params.hide {
        ctx.insert("HIDE", &hide);
    }

    let mut headers: Vec<Header> = Vec::new();
    if let Some(request_id) = query_params.request {
        ctx.insert("REQUEST_ID", &request_id);
        if let Ok(hdrs) = get_request_headers(conn, request_id).await {
            for h in hdrs.into_iter() {
                headers.push(Header {
                    key: h.key,
                    value: h.value,
                });
            }
        }
    }
    if let Ok(hdrs) = get_collection_headers(conn, collection_id).await {
        for h in hdrs.into_iter() {
            if !headers.iter().any(|x| x.key == h.key) {
                headers.push(Header {
                    key: h.key,
                    value: h.value,
                });
            }
        }
    }
    ctx.insert("headers", &headers);

    let collections = get_main_collections(conn).await.unwrap();
    ctx.insert("collections", &collections);

    if let Ok(collection) = get_single_collection(conn, collection_id).await {
        ctx.insert("collection", &collection);
        ctx.insert("COLLECTION_ID", &collection_id);

        let requests = get_collection_requests(conn, collection_id).await.unwrap();
        ctx.insert("requests", &requests);

        let rendered = state
            .tera
            .render("pages/collection/loadtest.html", &ctx)
            .unwrap();
        return HttpResponse::Ok().body(rendered);
    };
    HttpResponse::NotFound().body("404")
}
