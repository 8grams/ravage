use crate::{models::collection::Collection, schema::collections};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

pub async fn get_main_collections(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Vec<Collection> {
    collections::table
        .select(Collection::as_select())
        .get_results(conn)
        .unwrap()
}
