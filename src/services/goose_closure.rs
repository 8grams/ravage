//! Goose load testing service implementation.
//! This module provides functionality for configuring and running load tests using the Goose framework.

use goose::prelude::*;
use std::{sync::Arc, time::Duration};

use crate::{
    models::{
        collection::Collection, header::Header, request::Request, request_header::RequestHeader,
    },
    utils::custom_transaction::loadtest_transaction_wrapper,
};
use super::websocket::server_handler::LogServerHandler;

/// Configuration for load test parameters
/// 
/// This struct defines the basic parameters for a load test:
/// - `follow`: Whether to follow redirects
/// - `launch_all_users`: Number of users to launch at once
/// - `total_users`: Total number of users to simulate
/// - `timeout`: Request timeout duration
/// - `hatch_rate`: Rate at which to spawn new users
/// - `runtime`: Total duration of the load test
/// - `log_path`: Path to store request logs
/// - `report_path`: Path to store test reports
#[derive(Clone)]
pub struct LoadConfig {
    /// Whether to follow redirects
    pub follow: bool,
    /// Number of users to launch at once
    pub launch_all_users: usize,
    /// Total number of users to simulate
    pub total_users: usize,
    /// Request timeout duration
    pub timeout: String,
    /// Rate at which to spawn new users
    pub hatch_rate: String,
    /// Total duration of the load test
    pub runtime: usize,
    /// Path to store request logs
    pub log_path: String,
    /// Path to store test reports
    pub report_path: String,
}

/// Complete configuration for a Goose load test
/// 
/// This struct combines load test parameters with additional configuration:
/// - `load_config`: Basic load test parameters
/// - `sender`: Channel for sending test progress updates
/// - `collection`: Collection of requests to test
/// - `requests`: Optional specific requests to test
/// - `headers`: Optional global headers to apply
#[derive(Clone)]
pub struct GooseLoadConfig {
    /// Basic load test parameters
    pub load_config: LoadConfig,
    pub sender: LogServerHandler,
    pub collection: Collection,
    pub load_test_id: i32,
    pub requests: Option<Vec<(Request, Vec<RequestHeader>)>>,
    /// Optional global headers to apply
    pub headers: Option<Vec<Header>>,
}

/// Launches a Goose load test in a separate task
/// 
/// This function:
/// 1. Spawns a new task for the load test
/// 2. Handles any errors that occur during the test
/// 3. Sends error messages through the provided sender
/// 
/// # Arguments
/// * `config` - Complete configuration for the load test
pub async fn goose_closure_load_test(config: GooseLoadConfig) {
    tokio::spawn(async move {
        if let Err(e) = run_loadtest(config.clone()).await {
            let sender = config.sender;
            let _ = sender.send_message(
                config.load_test_id,
                format!("Goose load test failed: {}", e),
            );
            eprintln!("Goose load test failed: {}", e);
        }
    });
}

/// Executes the Goose load test with the given configuration
/// 
/// This function:
/// 1. Creates a Goose scenario with the configured requests
/// 2. Sets up the Goose attack with all parameters
/// 3. Executes the load test
/// 4. Reports results through the provided sender
/// 
/// # Arguments
/// * `config` - Complete configuration for the load test
/// 
/// # Returns
/// * `Result<(), GooseError>` - Success or failure of the load test
async fn run_loadtest(config: GooseLoadConfig) -> Result<(), GooseError> {
    let config_clone = config.clone();

    let mut scenario = scenario!("WebsiteUser")
        // After each transaction runs, sleep randomly from 0.01 to 0.1 seconds.
        .set_wait_time(Duration::from_millis(10), Duration::from_millis(100))?;

    // Register transactions based on configuration
    if let Some(requests) = config.requests.clone() {
        for req_n_h in requests {
            let config = config_clone.clone();
            let trans = Transaction::new(Arc::new(move |user| {
                loadtest_transaction_wrapper(user, config.clone(), Some(req_n_h.clone()))
            }));
            let new_scenario = scenario.clone().register_transaction(trans);
            scenario = new_scenario;
        }
    } else {
        let trans = Transaction::new(Arc::new(move |user| {
            loadtest_transaction_wrapper(user, config_clone.clone(), None)
        }));
        let new_scenario = scenario.clone().register_transaction(trans);
        scenario = new_scenario;
    }

    // Configure and start the Goose attack
    let goose = GooseAttack::initialize()?
        .register_scenario(scenario)
        .set_default(GooseDefault::Host, config.collection.host.as_str())?
        .set_default(
            GooseDefault::ReportFile,
            config.load_config.report_path.as_str(),
        )?
        .set_default(
            GooseDefault::RequestLog,
            config.load_config.log_path.as_str(),
        )?
        // .set_default(GooseDefault::Timeout, config.load_config.timeout.as_str())?
        .set_default(
            GooseDefault::HatchRate,
            config.load_config.hatch_rate.as_str(),
        )?
        .set_default(GooseDefault::Users, config.load_config.total_users)?
        .set_default(GooseDefault::RunTime, config.load_config.runtime)?
        .set_default(GooseDefault::NoResetMetrics, true)?;

    // Execute the load test and report results
    let result = goose.execute().await;

    let sender = config.sender;
    let _ = sender
        .send_message(
            config.load_test_id,
            format!(
                "<div id='logs' hx-swap-oob='beforeend'><pre><code>initializing {} users states...</code></pre></div>",
                config.load_config.total_users
            )
            .to_string(),
        )
        .await;
    match result {
        Ok(metrics) => {
            let _ = sender.send_message(
                config.load_test_id,
                format!(
                    "<div id='logs' hx-swap-oob='beforeend'><pre><code>✅ Load test completed in {}s with {} users</code></pre></div>",
                    metrics.duration, metrics.total_users
                ),
            ).await;
        }
        Err(error) => {
            let _ = sender.send_message(
                config.load_test_id,
                format!("<div id='logs' hx-swap-oob='beforeend'><pre><code>❌ Loadtest failed: {}</code></pre></div>", error),
            ).await;
        }
    }
    Ok(())
}
