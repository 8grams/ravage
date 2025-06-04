use crate::{app_state::AppState, schema::users, utils};
use actix_web::{HttpResponse, Responder, web};
use diesel::RunQueryDsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RoleFormData {
    pub role: String,
}

pub async fn update_role(
    form_data: web::Form<RoleFormData>,
    path: web::Path<i32>,
    state: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let session_json = utils::session::get_session_json(&session).await;
    if session_json.is_some_and(|t| t.role == "admin") {
        let _ = diesel::update(users::table)
            .set(users::role.eq(&form_data.role))
            .filter(users::id.eq(path.into_inner()))
            .execute(conn);
    }
    HttpResponse::Ok().body("Ok")
}
