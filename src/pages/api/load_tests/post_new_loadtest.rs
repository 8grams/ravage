use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::services::get_collection::get_single_collection;
use crate::services::get_request::get_single_request;
use crate::services::gooses::{LoadTestConfig, goose_loadtest};

#[derive(Deserialize, Clone, PartialEq, PartialOrd)]
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

    let timeout: usize = json_data
        .timeout
        .unwrap_or("10".into())
        .parse::<usize>()
        .unwrap();
    let starts_per_second: usize = json_data
        .starts_per_second
        .unwrap_or("10".into())
        .parse::<usize>()
        .unwrap();
    let total_users: usize = json_data
        .total_users
        .unwrap_or("10".into())
        .parse::<usize>()
        .unwrap();
    let runtime: usize = json_data
        .runtime
        .unwrap_or("10".into())
        .parse::<usize>()
        .unwrap();
    let follow: bool = json_data.follow.unwrap_or("on".into()) == "on";

    let _ = goose_loadtest(
        coll,
        None,
        Some(LoadTestConfig {
            load_test_id: 0,
            starts_per_second,
            timeout,
            runtime,
            follow,
            total_users,
        }),
    )
    .await;

    HttpResponse::Ok().body("Ok")
}
