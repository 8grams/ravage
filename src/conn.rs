use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::env;

pub fn sqlite_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or("db/ravage.db".to_string());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn sqlite_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = env::var("DATABASE_URL").unwrap_or("db/ravage.db".to_string());
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(manager).unwrap()
}
