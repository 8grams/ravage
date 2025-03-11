use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    models::request_header::NewRequestHeader,
    schema::{request_headers, requests},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct JsonData {
    name: String,
    path: String,
    collection_id: String,
    method: String,
    body_type: String,
    body_content: String,
    headers: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Header {
    key: String,
    value: String,
}

pub async fn update_request(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    data: web::Json<JsonData>,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let json_data = data.into_inner();
    let coll_id = json_data.collection_id.parse::<i32>().unwrap();
    let req_id = id.into_inner();
    let _ = diesel::update(requests::table)
        .set((
            requests::name.eq(&json_data.name),
            requests::path.eq(&json_data.path),
            requests::collection_id.eq(&coll_id),
            requests::method.eq(&json_data.method),
            requests::body_type.eq(&json_data.body_type),
            requests::body_content.eq(&json_data.body_content),
        ))
        .filter(requests::id.eq(req_id))
        .execute(conn)
        .unwrap();
    if let Some(headers) = json_data.headers {
        let _ = diesel::delete(request_headers::table)
            .filter(request_headers::request_id.eq(req_id))
            .execute(conn)
            .unwrap();
        let vec_headers: Vec<Header> = serde_json::from_str(headers.as_str()).unwrap();
        let new_headers: Vec<NewRequestHeader> = vec_headers
            .into_iter()
            .map(|h| NewRequestHeader {
                request_id: req_id,
                value: h.value,
                key: h.key,
            })
            .collect();
        let _ = diesel::insert_into(request_headers::table)
            .values(&new_headers)
            .execute(conn)
            .unwrap();
    }
    HttpResponse::Ok().body("ok")
}
