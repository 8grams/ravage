use crate::schema::collection_headers;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Selectable, Queryable)]
#[diesel(table_name = collection_headers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CollectionHeader {
    pub id: i32,
    pub collection_id: i32,
    pub key: String,
    pub value: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name=collection_headers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCollectionHeader {
    pub collection_id: i32,
    pub key: String,
    pub value: String,
}
