use crate::{app_state::AppState, models::user::NewUser, schema::users, utils};
use actix_web::{HttpResponse, Responder, web};
use diesel::RunQueryDsl;

pub async fn new_user(
    form: web::Form<NewUser>,
    state: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let session_json = utils::session::get_session_json(&session).await;
    println!("{:?}", &session_json);
    if session_json.is_some_and(|t| t.role == "admin") {
        let _ = diesel::insert_into(users::table)
            .values(form.into_inner())
            .execute(conn);
    }
    HttpResponse::Ok().body("Ok")
}
