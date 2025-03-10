use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    models::request::{NewRequest, Request},
    schema::requests,
    utils::{self, session::Tab},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct JsonData {
    name: String,
    path: String,
    collection_id: String,
    method: String,
    body_type: String,
    body_content: String,
    tab_index: Option<usize>,
}

pub async fn new_request(
    state: web::Data<AppState>,
    session: actix_session::Session,
    data: web::Json<JsonData>,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let json_data = data.into_inner();
    let mut tabs = utils::session::get_session_tabs(session.clone()).await;

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
        Ok(succ_req) => {
            // Create a new tab from the inserted request
            let new_tab = Tab {
                name: succ_req.name.clone(),
                r#type: "request".to_string(),
                collection_id: succ_req.collection_id.unwrap_or(1),
                id: Some(succ_req.id),
            };

            // Add or replace tab at specified index
            if let Some(index) = json_data.tab_index {
                if index < tabs.len() {
                    tabs[index] = new_tab; // Replace existing tab
                } else {
                    tabs.push(new_tab); // Add new tab
                }
            } else {
                tabs.push(new_tab); // Append if no index provided
            }

            // Update session with new tabs
            if let Err(err) = session.insert("tabs", &tabs) {
                eprintln!("Failed to update session: {:?}", err);
            }

            HttpResponse::Ok()
                .append_header(("hx-location", format!("/requests/{}", &succ_req.id)))
                .json(succ_req)
        }
        Err(err) => {
            eprintln!("Failed to insert request: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to insert request")
        }
    }
}
