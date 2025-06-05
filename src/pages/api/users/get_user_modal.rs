use crate::{AppState, models::user::User, schema::users, utils::tera_context::base_context};
use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
pub async fn new_user_modal(
    state: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let ctx = base_context(&session);
    let rendered = state
        .tera
        .render("components/users/user_modal.html", &ctx)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn edit_user_modal(
    path: web::Path<i32>,
    state: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let pool = &mut state.pool.get().unwrap();
    let mut ctx = base_context(&session);
    if let Ok(user_data) = users::table
        .select(User::as_select())
        .filter(users::id.eq(path.into_inner()))
        .get_result(pool)
    {
        ctx.insert("user", &user_data);
    }
    let rendered = state
        .tera
        .render("components/users/user_modal.html", &ctx)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}
