use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use diesel::prelude::*;
use std::fs::{File, OpenOptions, create_dir_all};
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
    let text = reqw_req.text().await.unwrap();
    let log_dir = "./static/log";
    create_dir_all(log_dir).unwrap();

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let file_name = format!("log_{}.txt", timestamp);
    let log_path = format!("{}/{}", log_dir, file_name);
    let mut file = File::create(&log_path).unwrap();
    file.write(text.as_bytes()).unwrap();

    let new_loadtest = NewLoadTest {
        name: req.name.clone(),
        source_type: "Request".to_string(),
        source_id: req.id,
        log_path: format!("/log/{}", file_name),
    };

    let _ = diesel::insert_into(load_tests::table)
        .values(&new_loadtest)
        .execute(conn)
        .unwrap();

    println!("{}", text);
    HttpResponse::Ok().body("Ok")
}
