use diesel::{
    SqliteConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

use crate::{models::request::Request, schema::requests};

pub async fn get_single_request(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    request_id: i32,
) -> Result<Request, diesel::result::Error> {
    requests::table
        .select(Request::as_select())
        .find(request_id)
        .get_result(conn)
}
