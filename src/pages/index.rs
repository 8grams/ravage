use crate::{
    app_state::AppState,
    models::collection::Collection,
    schema::collections,
    utils::{self, tera_context::base_context},
};
use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

pub async fn main_pages(
    data: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let conn = &mut data.pool.get().unwrap();
    let mut ctx = base_context(&session);
    let current_tabs = utils::session::get_session_tabs(session.clone()).await;
    ctx.insert("tabs", &current_tabs);
    if let Ok(cs) = collections::table
        .select(Collection::as_select())
        .load::<Collection>(conn)
    {
        ctx.insert("collections", &cs);
    };
    let rendered = data.tera.render("pages/index.html", &ctx).unwrap();

    HttpResponse::Ok().body(rendered)
}
