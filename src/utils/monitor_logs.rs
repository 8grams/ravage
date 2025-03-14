use std::io::{Error, ErrorKind};
use std::time::Duration;

use crate::app_state::AppState;
use crate::app_state::LogChannel;

use actix_web::web;

use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::time::sleep;

pub async fn get_or_create_channel(state: &web::Data<AppState>, id: i32) -> LogChannel {
    let mut channels = state.log_channels.lock().await;

    channels
        .entry(id)
        .or_insert_with(|| {
            let (sender, _) = broadcast::channel::<String>(100);
            sender
        })
        .clone()
}

pub async fn stream_log_file(
    sender: Sender<String>,
    log_path: impl AsRef<std::path::Path>,
) -> Result<(), Error> {
    if !log_path.as_ref().exists() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Log file not found",
        ));
    }
    tokio::spawn(async move {
        let mut position: u64 = 0;

        loop {
            let file = match File::open(log_path).await {
                Ok(file) => file,
                Err(_) => {
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };
        }
    });
    Ok(())
}
