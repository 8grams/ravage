use diesel::{
    SqliteConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

use crate::{
    models::{request::Request, request_header::RequestHeader},
    schema::{request_headers, requests},
};

pub async fn get_single_request(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    request_id: i32,
) -> Result<Request, diesel::result::Error> {
    requests::table
        .select(Request::as_select())
        .find(request_id)
        .get_result(conn)
}

pub async fn get_request_headers(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    request_id: i32,
) -> Result<Vec<RequestHeader>, diesel::result::Error> {
    request_headers::table
        .select(RequestHeader::as_select())
        .filter(request_headers::request_id.eq(request_id))
        .get_results(conn)
}
