use futures_util::{
    StreamExt as _,
    future::{self, Either},
};
use std::time::{Duration, Instant};
use tokio::{sync::mpsc, time::interval};

use super::server::{LogServer, LogServerHandler};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

pub async fn log_ws(
    mut log_server: LogServerHandler,
    mut session: actix_ws::Session,
    msg_stream: actix_ws::MessageStream,
    log_id: i32,
) {
    let mut last_hearthbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();
    let conn_id = log_server.connect(conn_tx, log_id).await;
}
