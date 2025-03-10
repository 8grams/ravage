use crate::{models::collection::Collection, schema::collections};
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
