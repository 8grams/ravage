use crate::{app_state::AppState, schema::users, utils};
use actix_web::{HttpResponse, Responder, web};
use diesel::RunQueryDsl;
use diesel::prelude::*;

pub async fn delete_user(
    path: web::Path<i32>,
    state: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let session_json = utils::session::get_session_json(&session).await;
    if session_json.is_some_and(|t| t.role == "admin") {
        let _ = diesel::delete(users::table)
            .filter(users::id.eq(path.into_inner()))
            .execute(conn);
    }
    HttpResponse::Ok().body("Ok")
}
