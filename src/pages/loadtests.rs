use actix_web::{HttpResponse, Responder, Scope, web};

use crate::{
    app_state::AppState, services::loadtest_services::get_loadtests,
    utils::tera_context::base_context,
};

async fn index(state: web::Data<AppState>, session: actix_session::Session) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let loadtests = get_loadtests(conn).await.unwrap();
    let mut ctx = base_context(&session);
    ctx.insert("histories", &loadtests);
    let rendered = state
        .tera
        .render("pages/histories/index.html", &ctx)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}

pub fn loadtest_scope() -> Scope {
    web::scope("/histories").route("", web::get().to(index))
}
