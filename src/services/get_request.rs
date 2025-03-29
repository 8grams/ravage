//! Request retrieval service functions.
//! This module provides functions for retrieving HTTP requests and their headers from the database.

use diesel::{
    SqliteConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

use crate::{
    models::{request::Request, request_header::RequestHeader},
    schema::{request_headers, requests},
};

/// Retrieves all requests belonging to a specific collection
/// 
/// This function:
/// 1. Queries the requests table
/// 2. Filters by collection ID
/// 3. Returns all matching requests as a vector
/// 
/// # Arguments
/// * `conn` - Database connection from the connection pool
/// * `collection_id` - ID of the collection to get requests for
/// 
/// # Returns
/// * `Result<Vec<Request>, diesel::result::Error>` - List of requests or database error
pub async fn get_collection_requests(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    collection_id: i32,
) -> Result<Vec<Request>, diesel::result::Error> {
    requests::table
        .select(Request::as_select())
        .filter(requests::collection_id.eq(collection_id))
        .get_results(conn)
}

/// Retrieves a single request by its ID
/// 
/// This function:
/// 1. Queries the requests table
/// 2. Finds the request by ID
/// 3. Returns the matching request
/// 
/// # Arguments
/// * `conn` - Database connection from the connection pool
/// * `request_id` - ID of the request to retrieve
/// 
/// # Returns
/// * `Result<Request, diesel::result::Error>` - The requested request or database error
pub async fn get_single_request(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    request_id: i32,
) -> Result<Request, diesel::result::Error> {
    requests::table
        .select(Request::as_select())
        .find(request_id)
        .get_result(conn)
}

/// Retrieves all headers for a specific request
/// 
/// This function:
/// 1. Queries the request_headers table
/// 2. Filters by request ID
/// 3. Returns all matching headers as a vector
/// 
/// # Arguments
/// * `conn` - Database connection from the connection pool
/// * `request_id` - ID of the request to get headers for
/// 
/// # Returns
/// * `Result<Vec<RequestHeader>, diesel::result::Error>` - List of headers or database error
pub async fn get_request_headers(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    request_id: i32,
) -> Result<Vec<RequestHeader>, diesel::result::Error> {
    request_headers::table
        .select(RequestHeader::as_select())
        .filter(request_headers::request_id.eq(request_id))
        .get_results(conn)
}
