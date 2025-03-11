use crate::schema::request_headers;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=request_headers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RequestHeader {
    pub id: i32,
    pub request_id: i32,
    pub key: String,
    pub value: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=request_headers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRequestHeader {
    pub request_id: i32,
    pub key: String,
    pub value: String,
}
