use crate::schema::collections;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Selectable, Clone)]
#[diesel(table_name=collections)]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub host: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = collections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCollection<'a> {
    pub name: &'a str,
    pub host: &'a str,
}
