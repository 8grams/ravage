use crate::schema::load_tests;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=load_tests)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LoadTest {
    pub id: i32,
    pub source_type: Option<String>,
    pub source_id: Option<i32>,
    pub name: String,
    pub log_path: Option<String>,
    pub report_path: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=load_tests)]
pub struct NewLoadTest {
    pub source_type: String,
    pub source_id: i32,
    pub name: String,
}
