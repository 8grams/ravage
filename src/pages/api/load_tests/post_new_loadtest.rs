use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::services::get_collection::get_single_collection;
use crate::services::get_request::get_single_request;
use crate::services::gooses::goose_loadtest;

#[derive(Deserialize)]
pub struct JsonData {
    name: String,
    collection_id: String,
    request_id: Option<String>,
    timeout: Option<String>,
    starts_per_second: Option<String>,
    total_users: Option<String>,
    follow: Option<String>,
    runtime: Option<String>,
}

pub async fn new_loadtest(data: web::Json<JsonData>, state: web::Data<AppState>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let json_data = data.into_inner();

    let collection_id = json_data.collection_id.parse::<i32>().unwrap();
    let coll = get_single_collection(conn, collection_id).await.unwrap();
    let request_id = json_data
        .request_id
        .unwrap_or("0".into())
        .parse::<i32>()
        .unwrap();
    let req = get_single_request(conn, request_id);

    let _ = goose_loadtest(coll, None).await;

    HttpResponse::Ok().body("Ok")
}
