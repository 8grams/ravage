//! Log monitoring functionality for load tests.
//! This module provides utilities for managing log channels that broadcast load test results.

use crate::app_state::AppState;
use crate::app_state::LogChannel;
use actix_web::web;
use tokio::sync::broadcast;

/// Gets an existing log channel or creates a new one for a specific load test
/// 
/// This function manages log channels for load tests, ensuring that each test has its own
/// dedicated channel for broadcasting results. The channel is created with a capacity of 100
/// messages to prevent memory issues during high-load scenarios.
/// 
/// # Arguments
/// * `state` - The application state containing the log channels map
/// * `id` - The ID of the load test
/// 
/// # Returns
/// * `LogChannel` - A sender that can be used to broadcast log messages
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
