use actix_session::Session;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Tab {
    pub name: String,
    pub id: Option<i32>,
    pub r#type: String,
}

pub async fn get_session_tabs(session: Session) -> Vec<Tab> {
    if let Some(current) = session.get::<Vec<Tab>>("tabs").unwrap() {
        return current;
    } else {
        return vec![];
    };
}
