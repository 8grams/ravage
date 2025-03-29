use std::{collections::HashMap, sync::Arc};

use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, Pool},
};
use futures::lock::Mutex;
use tokio::sync::broadcast;
use crate::services::websocket::server_handler::LogServerHandler;

/// Type alias for a broadcast channel that sends log messages
pub type LogChannel = broadcast::Sender<String>;

/// Application state shared across all requests
/// 
/// This struct holds the global state that needs to be accessible throughout the application:
/// - `tera`: Template engine for rendering HTML pages
/// - `pool`: Database connection pool for SQLite
/// - `log_channels`: Thread-safe map of load test IDs to their log channels
pub struct AppState {
    /// Tera template engine for rendering HTML pages
    pub tera: tera::Tera,
    /// Database connection pool for SQLite operations
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
    /// Thread-safe map of load test IDs to their log channels
    pub log_channels: Arc<Mutex<HashMap<i32, LogChannel>>>,
    pub log_server: LogServerHandler,
}
