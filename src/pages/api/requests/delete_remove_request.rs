use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

use crate::{app_state::AppState, schema::requests};

pub async fn remove_request(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let _ = diesel::delete(requests::table)
        .filter(requests::id.eq(id.into_inner()))
        .execute(conn)
        .unwrap();
    HttpResponse::Ok().body("Ok")
}
