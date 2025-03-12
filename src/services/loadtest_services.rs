
use diesel::prelude::*;
use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, PooledConnection},
};

use crate::{
    models::load_test::{LoadTest, NewLoadTest},
    schema::load_tests,
};

pub async fn get_loadtests(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<LoadTest>, diesel::result::Error> {
    load_tests::table
        .order(load_tests::created_at.desc())
        .select(LoadTest::as_select())
        .get_results(conn)
}

pub async fn insert_loadtest(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    new_loadtest: NewLoadTest,
) -> Result<LoadTest, diesel::result::Error> {
    diesel::insert_into(load_tests::table)
        .values(&new_loadtest)
        .returning(LoadTest::as_select())
        .get_result(conn)
}

pub async fn update_loadtest(
    conn: &mut SqliteConnection,
    loadtest_id: i32,
    log_path: String,
    report_path: String,
) {
    let _ = diesel::update(load_tests::table)
        .set((
            load_tests::log_path.eq(log_path),
            load_tests::report_path.eq(report_path),
        ))
        .filter(load_tests::id.eq(loadtest_id))
        .execute(conn)
        .unwrap();
}
