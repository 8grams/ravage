use std::env;

use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use serde::Deserialize;
use tokio::fs::create_dir_all;

use crate::app_state::AppState;
use crate::models::header::Header;
use crate::models::load_test::NewLoadTest;
use crate::models::request::Request;
use crate::models::request_header::RequestHeader;
use crate::services::get_collection::{get_collection_headers, get_single_collection};
use crate::services::get_request::{
    get_collection_requests, get_request_headers, get_single_request,
};
use crate::services::goose_closure::{GooseLoadConfig, LoadConfig, goose_closure_load_test};
use crate::services::loadtest_services::insert_loadtest;
use crate::services::websocket::server_handler::LogServerHandler;

#[derive(Deserialize, Clone, PartialEq, PartialOrd)]
pub struct JsonData {
    name: String,
    collection_id: String,
    request_id: Option<String>,
    timeout: Option<String>,
    hatch_rate: Option<String>,
    launch_all_users: Option<String>,
    total_users: Option<String>,
    follow: Option<String>,
    runtime: Option<String>,
    body_type: Option<String>,
    body_content: Option<String>,
    headers: Option<String>,
}

pub async fn new_loadtest(data: web::Json<JsonData>, state: web::Data<AppState>) -> impl Responder {
    let mut ctx = tera::Context::new();
    let conn = &mut state.pool.get().unwrap();
    let json_data = data.into_inner();
    let mut headers: Vec<Header> = Vec::new();

    let collection_id = json_data.collection_id.parse::<i32>().unwrap();
    let coll = get_single_collection(conn, collection_id).await.unwrap();
    let request_id = json_data
        .request_id
        .unwrap_or("0".into())
        .parse::<i32>()
        .unwrap();

    if let Ok(hdrs) = get_collection_headers(conn, collection_id).await {
        headers = hdrs
            .into_iter()
            .map(|h| Header {
                key: h.key,
                value: h.value,
            })
            .collect();
    }
    let mut requests: Vec<(Request, Vec<RequestHeader>)> = Vec::new();
    if request_id != 0 {
        if let Ok(req) = get_single_request(conn, request_id).await {
            let hdrs = get_request_headers(conn, req.id).await.unwrap();
            requests.push((req.clone(), hdrs));
        };
    } else {
        for req in get_collection_requests(conn, collection_id).await.unwrap() {
            let hdrs = get_request_headers(conn, req.id).await.unwrap();
            requests.push((req.clone(), hdrs));
        }
    }
    if !requests.is_empty() && request_id != 0 {
        let mut request = requests.first().unwrap().clone();
        if let Some(body_type) = json_data.body_type {
            request = (
                Request {
                    body_type: Some(body_type),
                    ..request.clone().0
                },
                request.1,
            );
        }
        if let Some(body_content) = json_data.body_content {
            request = (
                Request {
                    body_content: Some(body_content),
                    ..request.0
                },
                request.1,
            );
        }
        requests = vec![request];
    }

    if let Some(hdr_string) = json_data.headers {
        if let Ok(hdrs) = serde_json::from_str::<Vec<Header>>(&hdr_string) {
            let keys: Vec<String> = hdrs.iter().map(|h| h.key.clone()).collect();
            headers = headers
                .iter()
                .filter(|h| !keys.contains(&h.key))
                .cloned()
                .collect();
            for h in hdrs {
                headers.push(h)
            }
        };
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

    ctx.insert("LOADTEST_ID", &lt.id);
    let sender = state.log_server.clone();
    let timeout = json_data.timeout.unwrap_or("100".into());
    let hatch_rate = json_data.hatch_rate.unwrap_or("100".into());
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

    let config = GooseLoadConfig {
        load_config: LoadConfig {
            log_path: format!("{}/{}", data_dir.clone(), log_file_name),
            report_path: format!("{}/{}", data_dir, report_file_name),
            launch_all_users,
            timeout,
            hatch_rate,
            runtime,
            follow,
            total_users,
        },
        sender,
        collection: coll,
        requests: Some(requests),
        headers: Some(vec![]),
    };
    let _ = goose_closure_load_test(config).await;

    let rendered = state
        .tera
        .render("components/load_tests/log_modal.html", &ctx)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}
