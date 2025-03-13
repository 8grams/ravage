use std::collections::HashMap;
use std::env;

use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use serde::Deserialize;
use tokio::fs::create_dir_all;

use crate::app_state::AppState;
use crate::models::load_test::NewLoadTest;
use crate::models::request::Request;
use crate::services::get_collection::{get_collection_headers, get_single_collection};
use crate::services::get_request::{get_request_headers, get_single_request};
use crate::services::gooses::{LoadTestConfig, goose_loadtest};
use crate::services::loadtest_services::insert_loadtest;
use crate::utils::monitor_logs::get_or_create_channel;

#[derive(Deserialize, Clone, PartialEq, PartialOrd)]
pub struct JsonData {
    name: String,
    collection_id: String,
    request_id: Option<String>,
    timeout: Option<String>,
    launch_all_users: Option<String>,
    total_users: Option<String>,
    follow: Option<String>,
    runtime: Option<String>,
    body_type: Option<String>,
    body_content: Option<String>,
    headers: Option<String>,
}

pub async fn new_loadtest(data: web::Json<JsonData>, state: web::Data<AppState>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let json_data = data.into_inner();
    let mut headers: HashMap<String, String> = HashMap::new();

    let collection_id = json_data.collection_id.parse::<i32>().unwrap();
    let coll = get_single_collection(conn, collection_id).await.unwrap();
    let request_id = json_data
        .request_id
        .unwrap_or("0".into())
        .parse::<i32>()
        .unwrap();

    if let Ok(hdrs) = get_collection_headers(conn, collection_id).await {
        headers = hdrs.into_iter().map(|h| (h.key, h.value)).collect();
    }
    let mut request: Option<Request> = None;
    if let Ok(req) = get_single_request(conn, request_id).await {
        request = Some(req.clone());
        if let Ok(hdrs) = get_request_headers(conn, req.id).await {
            headers = hdrs.into_iter().map(|h| (h.key, h.value)).collect()
        }
    };
    if let Some(req) = request.clone() {
        if let Some(body_type) = json_data.body_type {
            request = Some(Request {
                body_type: Some(body_type),
                ..req.clone()
            });
        }
        if let Some(body_content) = json_data.body_content {
            request = Some(Request {
                body_content: Some(body_content),
                ..req
            });
        }
    }

    if let Some(hdr_string) = json_data.headers {
        let hdrs: Vec<crate::models::header::Header> = serde_json::from_str(&hdr_string).unwrap();
        for h in hdrs.into_iter() {
            headers.insert(h.key, h.value);
        }
    }

    let data_dir = env::var("DATA_DIR")
        .unwrap_or("/opt/data".into())
        .to_string();
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let report_file_name = format!("report_{}.html", timestamp);
    let log_file_name = format!("log_{}.txt", timestamp);

    create_dir_all(data_dir.clone()).await.unwrap();

    let lt = insert_loadtest(
        &mut state.pool.get().unwrap(),
        NewLoadTest {
            name: json_data.name,
            source_type: json_data.collection_id.clone(),
            source_id: collection_id,
            log_path: format!("/data/{}", log_file_name),
            report_path: format!("/data/{}", report_file_name),
        },
    )
    .await
    .unwrap();
    let sender = get_or_create_channel(&state, lt.id).await;
    let timeout = json_data.timeout.unwrap_or("100".into());
    let launch_all_users: usize = json_data
        .launch_all_users
        .unwrap_or("30".into())
        .parse::<usize>()
        .unwrap();
    let total_users: usize = json_data
        .total_users
        .unwrap_or("500".into())
        .parse::<usize>()
        .unwrap();
    let runtime: usize = json_data
        .runtime
        .unwrap_or("30".into())
        .parse::<usize>()
        .unwrap();
    let follow: bool = json_data.follow.unwrap_or("on".into()) == "on";

    println!("{:?}", headers);
    println!("{:?}", request);
    let _ = goose_loadtest(
        coll,
        request,
        LoadTestConfig {
            load_test_id: lt.id,
            launch_all_users,
            timeout,
            runtime,
            follow,
            total_users,
            log_path: format!("{}/{}", data_dir.clone(), log_file_name),
            report_path: format!("{}/{}", data_dir, report_file_name),
            headers: Some(headers),
        },
        sender,
    )
    .await;

    let response = format!(
        r"<pre id='log' data-id='{}'><code>Load test running!!</code></pre>",
        lt.id
    );
    HttpResponse::Ok().body(response)
}
