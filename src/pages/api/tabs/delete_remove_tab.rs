use crate::utils::session::{self, Tab};
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    name: String,
    id: Option<i32>,
    r#type: String,
}

pub async fn remove_tab(
    session: actix_session::Session,
    params: web::Query<QueryParams>,
) -> impl Responder {
    let query_params = params.into_inner();
    let tab_from_params = Tab {
        name: query_params.name,
        id: query_params.id,
        r#type: query_params.r#type,
    };
    let mut current_session = session::get_session_tabs(session.clone()).await;
    current_session.retain(|f: &Tab| f != &tab_from_params);
    session.insert("tabs", current_session).unwrap();
    HttpResponse::Ok().body("Ok")
}
