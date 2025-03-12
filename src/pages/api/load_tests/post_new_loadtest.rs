use std::env;

use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use serde::Deserialize;
use tokio::fs::create_dir_all;

use crate::app_state::AppState;
use crate::models::load_test::NewLoadTest;
use crate::models::request::Request;
use crate::services::get_collection::get_single_collection;
use crate::services::get_request::get_single_request;
use crate::services::gooses::{LoadTestConfig, goose_loadtest};
use crate::services::loadtest_services::insert_loadtest;

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
    let mut request: Option<Request> = None;
    if let Ok(req) = get_single_request(conn, request_id).await {
        request = Some(req);
    };

    let data_dir = env::var("DATA_DIR")
        .unwrap_or("/opt/data".into())
        .to_string();
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let report_file_name = format!("report_{}.html", timestamp);
    let log_file_name = format!("log_{}.txt", timestamp);

    create_dir_all(data_dir.clone()).await.unwrap();

    let _ = insert_loadtest(
        &mut state.pool.get().unwrap(),
        NewLoadTest {
            name: json_data.name,
            source_type: json_data.collection_id.clone(),
            source_id: collection_id,
            log_path: format!("/data/{}", log_file_name),
            report_path: format!("/data/{}", report_file_name),
        },
    )
    .await;
    let timeout = json_data.timeout.unwrap_or("100".into());
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
        request,
        LoadTestConfig {
            load_test_id: 0,
            starts_per_second,
            timeout,
            runtime,
            follow,
            total_users,
            log_path: format!("{}/{}", data_dir.clone(), log_file_name),
            report_path: format!("{}/{}", data_dir, report_file_name),
        },
    )
    .await;

    HttpResponse::Ok().body("Ok")
}
