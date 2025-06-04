use crate::{AppState, models::user::User, schema::users, utils::tera_context::base_context};
use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

pub async fn users_page(
    state: web::Data<AppState>,
    session: actix_session::Session,
) -> impl Responder {
    let pool = &mut state.pool.get().unwrap();
    let mut ctx = base_context(&session);
    let users_data = users::table
        .select(User::as_select())
        .get_results(pool)
        .unwrap();
    ctx.insert("users", &users_data);
    let rendered = state.tera.render("pages/users/index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
