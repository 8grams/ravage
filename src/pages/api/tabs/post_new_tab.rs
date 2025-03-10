use crate::utils::session;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JsonData {
    name: String,
    id: Option<i32>,
    r#type: String,
    collection_id: String,
}

pub async fn new_tab(session: actix_session::Session, data: web::Json<JsonData>) -> impl Responder {
    let json_data = data.into_inner();
    let mut current_session = session::get_session_tabs(session.clone()).await;
    let new_session = session::Tab {
        name: json_data.name,
        id: json_data.id,
        r#type: json_data.r#type,
        collection_id: json_data.collection_id.parse::<i32>().unwrap(),
    };
    current_session.push(new_session);

    session.insert("tabs", current_session).unwrap();
    HttpResponse::Ok().body("Ok")
}
