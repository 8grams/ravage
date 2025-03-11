use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::io::Write;

use crate::app_state::AppState;
use crate::services::get_collection::{get_collection_headers, get_single_collection};
use crate::services::get_request::{get_request_headers, get_single_request};
use crate::services::reqwest::send_request;

pub async fn run(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();

    let req_id = id.into_inner();
    let req = get_single_request(conn, req_id).await.unwrap();
    let req_headers = get_request_headers(conn, req_id).await.unwrap();
    let coll = get_single_collection(conn, req.collection_id)
        .await
        .unwrap();
    let coll_headers = get_collection_headers(conn, req.collection_id)
        .await
        .unwrap();

    let reqhdrs: Vec<(String, String)> =
        req_headers.into_iter().map(|h| (h.key, h.value)).collect();
    let headers: Vec<(String, String)> =
        coll_headers.into_iter().map(|h| (h.key, h.value)).collect();
    let headers_map: HashMap<String, String> = headers.into_iter().chain(reqhdrs).collect();

    let reqw_req = send_request(&req, &coll, Some(headers_map)).await.unwrap();
    let bytes = reqw_req.bytes().await.unwrap();

    let text = String::from_utf8(bytes.clone().to_vec()).unwrap();
    let escaped = html_escape::encode_text(&text.clone()).into_owned();

    let log_dir = "./static/log";
    create_dir_all(log_dir).unwrap();

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let file_name = format!("log_{}.txt", timestamp);
    let log_path = format!("{}/{}", log_dir, file_name);
    let mut file = File::create(&log_path).unwrap();
    file.write_all(&bytes.clone()).unwrap();

    HttpResponse::Ok().body(escaped)
}
