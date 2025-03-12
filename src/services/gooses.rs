use crate::models::{collection::Collection, request::Request};
use goose::prelude::*;
use std::sync::OnceLock;
use tokio::sync::broadcast::Sender;
use tokio::task::spawn;

pub struct LoadTestConfig {
    pub follow: bool,
    pub load_test_id: i32,
    pub starts_per_second: usize,
    pub total_users: usize,
    pub timeout: String,
    pub runtime: usize,
    pub log_path: String,
    pub report_path: String,
}

// Global static OnceLock to store the request safely
static REQUEST_DATA: OnceLock<Request> = OnceLock::new();
static LOG_SENDER: OnceLock<Sender<String>> = OnceLock::new();

pub async fn goose_loadtest(
    collection: Collection,
    request: Option<Request>,
    config: LoadTestConfig,
    sender: Sender<String>,
) {
    // Store request in OnceLock for use in transactions
    if let Some(req) = request {
        let _ = REQUEST_DATA.set(req); // Ignores error if already set
    }
    let _ = LOG_SENDER.set(sender);

    // Spawn the load test in the background
    spawn(async move {
        if let Err(e) = run_goose_loadtest(collection, config).await {
            eprintln!("Goose load test failed: {}", e);
        }
    });
}

async fn run_goose_loadtest(
    collection: Collection,
    config: LoadTestConfig,
) -> Result<(), goose::GooseError> {
    // Build the GooseAttack configuration
    let mut goose = GooseAttack::initialize()?
        .register_scenario(
            scenario!("LoadtestTransactions")
                .register_transaction(transaction!(loadtest_transaction).set_on_start())
                .register_transaction(transaction!(loadtest_transaction_repeat)),
        )
        .set_default(GooseDefault::Host, collection.host.as_str())?
        .set_default(GooseDefault::ReportFile, config.report_path.as_str())?
        .set_default(GooseDefault::RequestLog, config.log_path.as_str())?;

    // Apply the config values
    goose = goose.set_default(GooseDefault::StartupTime, config.starts_per_second)?;
    goose = goose.set_default(GooseDefault::Users, config.total_users)?;
    goose = goose.set_default(GooseDefault::Timeout, config.timeout.as_str())?;
    goose = goose.set_default(GooseDefault::RunTime, config.runtime)?;
    goose = goose.set_default(GooseDefault::StickyFollow, config.follow)?;

    if let Some(sender) = LOG_SENDER.get() {
        let _ = sender.send(format!(
            "Loadtest starting with {} users",
            config.total_users
        ));
    }
    // Execute the load test
    let result = goose.execute().await;

    match result {
        Ok(metrics) => {
            if let Some(sender) = LOG_SENDER.get() {
                let _ = sender.send(format!(
                    "Loadtest success with {} users",
                    metrics.total_users
                ));
                let _ = sender.send(format!("Durations {}", metrics.duration));
            }
        }
        Err(error) => {
            if let Some(sender) = LOG_SENDER.get() {
                let _ = sender.send(format!("Loadtest failed: {}", error));
            }
        }
    }

    Ok(())
}

// Initial transaction that runs once per user on start
async fn loadtest_transaction(user: &mut GooseUser) -> TransactionResult {
    perform_request(user).await
}

// Repeated transaction that runs continuously during the load test
async fn loadtest_transaction_repeat(user: &mut GooseUser) -> TransactionResult {
    perform_request(user).await
}

// Shared function to perform the actual request based on the method
async fn perform_request(user: &mut GooseUser) -> TransactionResult {
    // Retrieve the request from OnceLock
    if let Some(request) = REQUEST_DATA.get() {
        let path = &request.path;
        let body_content = request.body_content.clone().unwrap_or_default();

        // Determine which HTTP method to use
        match request.method.to_uppercase().as_str() {
            "POST" => {
                if let Some(body_type) = &request.body_type {
                    match body_type.as_str() {
                        "application/json" => {
                            user.post(path, body_content).await?;
                        }
                        _ => {
                            user.post(path, body_content).await?;
                        }
                    }
                } else {
                    {
                        user.post(path, "").await?;
                    }
                }
            }
            _ => {
                user.get(path).await?;
            }
        };
    } else {
        // If no specific request is provided, use default collection endpoint
        user.get("").await?;
    }

    Ok(())
}
