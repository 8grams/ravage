use crate::schema::collection_headers;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Selectable)]
#[diesel(table_name = collection_headers)]
pub struct CollectionHeader {
    pub id: i32,
    pub collection_id: i32,
    pub key: String,
    pub value: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=collection_headers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCollectionHeader<'a> {
    pub collection_id: &'a i32,
    pub key: &'a str,
    pub value: &'a str,
}
