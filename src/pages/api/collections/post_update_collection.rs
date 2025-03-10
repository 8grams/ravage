use crate::{
    app_state::AppState,
    models::collection_header::NewCollectionHeader,
    schema::{collection_headers, collections},
};
use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Header {
    key: String,
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdatedData {
    name: String,
    host: String,
    headers: Option<String>,
}

pub async fn update_collection(
    path: web::Path<i32>,
    state: web::Data<AppState>,
    data: web::Json<UpdatedData>,
) -> impl Responder {
    let c_id = path.into_inner();

    let json_data = data.into_inner();
    let conn = &mut state.pool.get().unwrap();
    let _ = diesel::update(collections::table)
        .filter(collections::id.eq(c_id))
        .set((
            collections::name.eq(&json_data.name),
            collections::host.eq(&json_data.host),
        ))
        .execute(conn)
        .unwrap();
    let _ = diesel::delete(collection_headers::table)
        .filter(collection_headers::collection_id.eq(c_id))
        .execute(conn)
        .unwrap();
    if let Some(headers_str) = json_data.headers {
        let headers: Vec<Header> = serde_json::from_str(&headers_str).unwrap();
        let new_headers: Vec<NewCollectionHeader> = headers
            .into_iter()
            .map(|h| NewCollectionHeader {
                collection_id: c_id,
                key: h.key,
                value: h.value,
            })
            .collect();
        let _ = diesel::insert_into(collection_headers::table)
            .values(new_headers)
            .execute(conn)
            .unwrap();
    }
    HttpResponse::Ok().body("Ok")
}
