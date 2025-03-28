use actix_web::{HttpResponse, Responder, web};

use crate::{
    app_state::AppState,
    services::{
        get_collection::{get_collection_headers, get_main_collections, get_single_collection},
        get_request::get_collection_requests,
    },
};

pub async fn new_load_test(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let mut ctx = tera::Context::new();
    let collection_id = id.into_inner();

    if let Ok(hdrs) = get_collection_headers(conn, collection_id).await {
        ctx.insert("default_headers", &hdrs);
    }

    let collections = get_main_collections(conn).await.unwrap();
    ctx.insert("collections", &collections);

    if let Ok(collection) = get_single_collection(conn, collection_id).await {
        ctx.insert("collection", &collection);
        ctx.insert("COLLECTION_ID", &collection_id);

        let requests = get_collection_requests(conn, collection_id).await.unwrap();
        ctx.insert("requests", &requests);

        let rendered = state
            .tera
            .render("pages/collection/new_request.html", &ctx)
            .unwrap();
        return HttpResponse::Ok().body(rendered);
    };
    HttpResponse::NotFound().body("404")
}
