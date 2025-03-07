use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    models::{collection::NewCollection, collection_header::NewCollectionHeader},
    schema::{collection_headers, collections},
};

#[derive(Deserialize, Debug, Serialize)]
pub struct Header {
    key: String,
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct JsonData {
    name: String,
    host: String,
    headers: Option<String>,
}

pub async fn new_collection(
    data: web::Data<AppState>,
    json_data: web::Json<JsonData>,
) -> impl Responder {
    let conn = &mut data.pool.get().unwrap();
    let new_data = json_data.into_inner();

    let coll = diesel::insert_into(collections::table)
        .values(NewCollection {
            name: &new_data.name,
            host: &new_data.host,
        })
        .returning(collections::id)
        .get_result(conn)
        .unwrap();

    if let Some(headers_str) = new_data.headers {
        let headers: Vec<Header> = serde_json::from_str(&headers_str).unwrap();
        let new_headers: Vec<NewCollectionHeader> = headers
            .into_iter()
            .map(|h| NewCollectionHeader {
                collection_id: coll,
                key: h.key,
                value: h.value,
            })
            .collect();
        println!("{:#?}", &new_headers);
        let _ = diesel::insert_into(collection_headers::table)
            .values(new_headers)
            .execute(conn);
    }

    HttpResponse::Ok().body("Ok")
}
