use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

use crate::{
    app_state::AppState, models::request::Request, schema::requests,
    services::get_collection::get_main_collections,
};

pub async fn request_detail(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let mut ctx = tera::Context::new();
    let collections = get_main_collections(conn).await;
    ctx.insert("collections", &collections);
    if let Ok(request) = requests::table
        .select(Request::as_select())
        .find(path.into_inner())
        .get_result(conn)
    {
        ctx.insert("request", &request);
        ctx.insert("COLLECTION_ID", &request.collection_id);
        let rendered = state.tera.render("pages/requests/form.html", &ctx).unwrap();
        return HttpResponse::Ok().body(rendered);
    }
    HttpResponse::NotFound().body("404")
}
