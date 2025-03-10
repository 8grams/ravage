use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, schema::requests};

#[derive(Deserialize, Serialize, Debug)]
pub struct JsonData {
    name: String,
    path: String,
    collection_id: String,
    method: String,
    body_type: String,
    body_content: String,
}

pub async fn update_request(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    data: web::Json<JsonData>,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let json_data = data.into_inner();
    let coll_id = json_data.collection_id.parse::<i32>().unwrap();
    let _ = diesel::update(requests::table)
        .set((
            requests::name.eq(&json_data.name),
            requests::path.eq(&json_data.path),
            requests::collection_id.eq(&coll_id),
            requests::method.eq(&json_data.method),
            requests::body_type.eq(&json_data.body_type),
            requests::body_content.eq(&json_data.body_content),
        ))
        .filter(requests::id.eq(id.into_inner()))
        .execute(conn)
        .unwrap();
    HttpResponse::Ok().body("ok")
}
