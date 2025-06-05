use crate::{app_state::AppState, models::user::NewUser, schema::users, utils};
use actix_web::{HttpResponse, Responder, web};
use diesel::RunQueryDsl;
use diesel::prelude::*;

pub async fn update_user(
    path: web::Path<i32>,
    form: web::Form<NewUser>,
    state: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let session_json = utils::session::get_session_json(&session).await;
    let form_data = form.into_inner();
    if session_json.is_some_and(|t| t.role == "admin") {
        let _ = diesel::update(users::table)
            .set((
                users::name.eq(form_data.name),
                users::email.eq(form_data.email),
                users::role.eq(form_data.role),
            ))
            .filter(users::id.eq(path.into_inner()))
            .execute(conn);
    }
    HttpResponse::Ok().body("Ok")
}
