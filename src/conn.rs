use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
use std::env;

pub fn sqlite_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or("db/ravage.db".to_string());
    let conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    return conn;
}

pub fn sqlite_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or("db/ravage.db".to_string());
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::new(manager).unwrap();
    return pool;
}
