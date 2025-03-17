use crate::app_state::AppState;
use crate::app_state::LogChannel;

use actix_web::web;

use tokio::sync::broadcast;

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
