use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::env;

/// Creates a single SQLite database connection
/// 
/// # Arguments
/// * None - Uses environment variable `DATABASE_URL` or defaults to "db/ravage.db"
/// 
/// # Returns
/// * `SqliteConnection` - A connection to the SQLite database
/// 
/// # Panics
/// * If the database connection cannot be established
pub fn sqlite_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or("db/ravage.db".to_string());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Creates a connection pool for SQLite database connections
/// 
/// # Arguments
/// * None - Uses environment variable `DATABASE_URL` or defaults to "db/ravage.db"
/// 
/// # Returns
/// * `Pool<ConnectionManager<SqliteConnection>>` - A pool of database connections
/// 
/// # Panics
/// * If the connection pool cannot be created
pub fn sqlite_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = env::var("DATABASE_URL").unwrap_or("db/ravage.db".to_string());
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(manager).unwrap()
}
