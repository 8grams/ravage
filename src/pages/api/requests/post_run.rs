use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use diesel::prelude::*;
use std::fs::{File, create_dir_all};
use std::io::Write;

use crate::models::load_test::NewLoadTest;
use crate::schema::load_tests;
use crate::services::reqwest::send_request;
use crate::{
    app_state::AppState,
    models::{collection::Collection, request::Request},
    schema::{collections, requests},
};

pub async fn run(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();

    let req = requests::table
        .select(Request::as_select())
        .find(id.into_inner())
        .get_result(conn)
        .unwrap();
    let coll = collections::table
        .select(Collection::as_select())
        .filter(collections::id.eq(req.collection_id))
        .get_result(conn)
        .unwrap();

    let reqw_req = send_request(&req, &coll, None).await.unwrap();
    let bytes = reqw_req.bytes().await.unwrap();

    let mut text_response = String::new();

    let text = String::from_utf8(bytes.clone().to_vec()).unwrap();
    if text.to_lowercase().contains("<html>") {
        let escaped = html_escape::encode_text(&text);
        text_response = escaped.to_string();
    } else {
        text_response = text.to_string();
    }

    let log_dir = "./static/log";
    create_dir_all(log_dir).unwrap();

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let file_name = format!("log_{}.txt", timestamp);
    let log_path = format!("{}/{}", log_dir, file_name);
    let mut file = File::create(&log_path).unwrap();
    file.write_all(&bytes.clone()).unwrap();

    HttpResponse::Ok().body(text_response)
}
