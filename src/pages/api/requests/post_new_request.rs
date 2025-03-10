use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    models::request::{NewRequest, Request},
    schema::requests,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct JsonData {
    name: String,
    path: String,
    collection_id: String,
    method: String,
    body_type: String,
    body_content: String,
}

pub async fn new_request(state: web::Data<AppState>, data: web::Json<JsonData>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let json_data = data.into_inner();

    let c_id: i32 = match json_data.collection_id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid collection_id"),
    };

    let new_request = NewRequest {
        name: &json_data.name,
        path: &json_data.path,
        method: &json_data.method,
        body_type: &json_data.body_type,
        body_content: &json_data.body_content,
        collection_id: &c_id,
    };

    // Insert new request and get the result
    match diesel::insert_into(requests::table)
        .values(&new_request)
        .returning(Request::as_select())
        .get_result::<Request>(conn)
    {
        Ok(succ_req) => HttpResponse::Ok()
            .append_header(("hx-location", format!("/requests/{}", &succ_req.id)))
            .json(succ_req),
        Err(err) => {
            eprintln!("Failed to insert request: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to insert request")
        }
    }
}
