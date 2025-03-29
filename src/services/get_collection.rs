//! Collection retrieval service functions.
//! This module provides functions for retrieving collections and their headers from the database.

use crate::{
    models::{collection::Collection, collection_header::CollectionHeader},
    schema::{collection_headers, collections},
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error,
};

/// Retrieves all collections from the database
/// 
/// This function:
/// 1. Queries the collections table
/// 2. Returns all collections as a vector
/// 
/// # Arguments
/// * `conn` - Database connection from the connection pool
/// 
/// # Returns
/// * `Result<Vec<Collection>, Error>` - List of collections or database error
pub async fn get_main_collections(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<Collection>, Error> {
    collections::table
        .select(Collection::as_select())
        .get_results(conn)
}

/// Retrieves a single collection by its ID
/// 
/// This function:
/// 1. Queries the collections table
/// 2. Finds the collection by ID
/// 3. Returns the matching collection
/// 
/// # Arguments
/// * `conn` - Database connection from the connection pool
/// * `collection_id` - ID of the collection to retrieve
/// 
/// # Returns
/// * `Result<Collection, Error>` - The requested collection or database error
pub async fn get_single_collection(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    collection_id: i32,
) -> Result<Collection, Error> {
    collections::table
        .select(Collection::as_select())
        .find(collection_id)
        .get_result(conn)
}

/// Retrieves all headers for a specific collection
/// 
/// This function:
/// 1. Queries the collection_headers table
/// 2. Filters by collection ID
/// 3. Returns all matching headers as a vector
/// 
/// # Arguments
/// * `conn` - Database connection from the connection pool
/// * `collection_id` - ID of the collection to get headers for
/// 
/// # Returns
/// * `Result<Vec<CollectionHeader>, Error>` - List of headers or database error
pub async fn get_collection_headers(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    collection_id: i32,
) -> Result<Vec<CollectionHeader>, Error> {
    collection_headers::table
        .select(CollectionHeader::as_select())
        .filter(collection_headers::collection_id.eq(collection_id))
        .get_results(conn)
}
