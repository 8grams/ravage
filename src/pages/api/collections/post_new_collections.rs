use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use serde::Deserialize;

use crate::{
    app_state::AppState, models::collection::NewCollection, schema::collections::dsl::collections,
};

#[derive(Deserialize, Debug)]
pub struct JsonData {
    name: String,
    host: String,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub refresh: Option<bool>,
}

pub async fn new_collection(
    data: web::Data<AppState>,
    json_data: web::Json<JsonData>,
    query_params: web::Query<QueryParams>,
) -> impl Responder {
    let conn = &mut data.pool.get().unwrap();
    let QueryParams { refresh } = query_params.into_inner();
    let new_data = json_data.into_inner();

    let _ = diesel::insert_into(collections)
        .values(NewCollection {
            name: &new_data.name,
            host: &new_data.host,
        })
        .execute(conn)
        .unwrap();
    if refresh.is_some_and(|t| t) {
        return HttpResponse::Ok()
            .append_header(("Hx-Refresh", "true"))
            .body("Ok");
    }
    HttpResponse::Ok().body("Ok")
}
