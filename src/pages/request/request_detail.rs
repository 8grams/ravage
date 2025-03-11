use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

use crate::{
    app_state::AppState,
    models::request::Request,
    schema::requests,
    services::{get_collection::get_main_collections, get_request::get_request_headers},
};

pub async fn request_detail(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let mut ctx = tera::Context::new();
    let req_id = path.into_inner();

    if let Ok(headers) = get_request_headers(conn, req_id).await {
        ctx.insert("headers", &headers);
    };
    let collections = get_main_collections(conn).await.unwrap();

    ctx.insert("collections", &collections);
    if let Ok(request) = requests::table
        .select(Request::as_select())
        .find(req_id)
        .get_result(conn)
    {
        ctx.insert("request", &request);
        ctx.insert("COLLECTION_ID", &request.collection_id);
        let rendered = state.tera.render("pages/requests/form.html", &ctx).unwrap();
        return HttpResponse::Ok().body(rendered);
    }
    HttpResponse::NotFound().body("404")
}
