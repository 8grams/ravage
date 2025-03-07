use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

use crate::{app_state::AppState, models::collection::Collection, schema::collections};

pub async fn collections(data: web::Data<AppState>) -> impl Responder {
    let conn = &mut data.pool.get().unwrap();
    let cs = collections::table
        .select(Collection::as_select())
        .get_results(conn)
        .unwrap();
    HttpResponse::Ok().json(cs)
}
