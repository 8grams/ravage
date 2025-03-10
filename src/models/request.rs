use crate::schema::requests;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=requests)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Request {
    pub id: i32,
    pub collection_id: Option<i32>,
    pub name: String,
    pub path: String,
    pub method: String,
    pub body_type: String,
    pub body_content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=requests)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewRequest<'a> {
    pub collection_id: &'a i32,
    pub method: &'a str,
    pub name: &'a str,
    pub path: &'a str,
    pub body_type: &'a str,
    pub body_content: &'a str,
}
