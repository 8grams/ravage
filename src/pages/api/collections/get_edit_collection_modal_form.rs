use crate::{
    app_state::AppState,
    models::{collection::Collection, collection_header::CollectionHeader},
    schema::{collection_headers, collections},
    utils::tera_context::base_context,
};
use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

pub async fn edit_collection_modal_form(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    session: actix_session::Session,
) -> impl Responder {
    let mut ctx = base_context(&session);
    let conn = &mut state.pool.get().unwrap();
    let c_id = path.into_inner();

    if let Ok(c) = collections::table
        .select(Collection::as_select())
        .find(c_id)
        .first(conn)
    {
        ctx.insert("collection", &c);
    };

    if let Ok(chs) = collection_headers::table
        .select(CollectionHeader::as_select())
        .filter(collection_headers::collection_id.eq(c_id))
        .get_results(conn)
    {
        println!("{:#?}", chs);
        ctx.insert("headers", &chs);
    }
    let rendered = state
        .tera
        .render("components/collections/collection_modal.html", &ctx)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}
