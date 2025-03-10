use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use diesel::prelude::*;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::Write;

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

    let req = send_request(&req, &coll, None).await.unwrap();
    let text = req.text().await.unwrap();
    let log_dir = "./static/log";
    create_dir_all(log_dir).unwrap();

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let log_path = format!("{}/log_{}.txt", log_dir, timestamp);
    let mut file = File::create(&log_path).unwrap();
    file.write(text.as_bytes()).unwrap();

    println!("{}", text);
    HttpResponse::Ok().body("Ok")
}
