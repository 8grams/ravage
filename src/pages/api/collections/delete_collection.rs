use crate::schema::collections;
use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

use crate::app_state::AppState;

pub async fn remove_single_collection(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let _ = diesel::delete(collections::table)
        .filter(collections::id.eq(path.into_inner()))
        .execute(conn)
        .unwrap();
    HttpResponse::Ok().body("Ok")
}
