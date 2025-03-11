use crate::{
    models::{collection::Collection, collection_header::CollectionHeader},
    schema::{collection_headers, collections},
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error,
};

pub async fn get_main_collections(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Vec<Collection> {
    collections::table
        .select(Collection::as_select())
        .get_results(conn)
        .unwrap()
}

pub async fn get_single_collection(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    collection_id: i32,
) -> Result<Collection, Error> {
    collections::table
        .select(Collection::as_select())
        .find(collection_id)
        .get_result(conn)
}

pub async fn get_collection_headers(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    collection_id: i32,
) -> Result<Vec<CollectionHeader>, Error> {
    collection_headers::table
        .select(CollectionHeader::as_select())
        .filter(collection_headers::collection_id.eq(collection_id))
        .get_results(conn)
}
