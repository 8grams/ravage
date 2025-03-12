use std::{collections::HashMap, sync::Arc};

use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, Pool},
};
use futures::lock::Mutex;
use tokio::sync::broadcast;

pub type LogChannel = broadcast::Sender<String>;

pub struct AppState {
    pub tera: tera::Tera,
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
    pub log_channels: Arc<Mutex<HashMap<i32, LogChannel>>>,
}
