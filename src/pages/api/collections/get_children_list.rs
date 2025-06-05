use crate::{
    app_state::AppState, models::request::Request, schema::requests,
    utils::tera_context::base_context,
};
use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

pub async fn children_list(
    path: web::Path<i32>,
    state: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let mut ctx = base_context(&session);
    let collection_id = path.into_inner();

    // if let Ok(cs) = collections::table
    //     .select(Collection::as_select())
    //     .get_result(conn)
    // {
    //     ctx.insert("collections", &cs);
    // }
    if let Ok(reqs) = requests::table
        .select(Request::as_select())
        .filter(requests::collection_id.assume_not_null().eq(collection_id))
        .get_results(conn)
    {
        ctx.insert("requests", &reqs);
    }

    let rendered = state
        .tera
        .render("components/collections/children_list.html", &ctx)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}
