use crate::models::{collection::Collection, request::Request};
use chrono::Utc;
use goose::prelude::*;
use std::fs::create_dir_all;
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

pub async fn goose_loadtest(
    collection: Collection,
    request: Option<Request>,
    config: LoadTestConfig,
) {
    // Spawn the load test in the background
    spawn(async move {
        if let Err(e) = run_goose_loadtest(collection, request, config).await {
            eprintln!("Goose load test failed: {}", e);
        }
    });
}

async fn run_goose_loadtest(
    collection: Collection,
    request: Option<Request>,
    config: LoadTestConfig,
) -> Result<(), goose::GooseError> {
    // Start building the GooseAttack configuration
    let mut goose = GooseAttack::initialize()?
        .register_scenario(
            scenario!("LoadtestTransactions")
                .register_transaction(transaction!(loadtest_transaction)),
        )
        .set_default(GooseDefault::Host, collection.host.as_str())?
        .set_default(GooseDefault::ReportFile, config.report_path.as_str())?
        .set_default(GooseDefault::RequestLog, config.log_path.as_str())?;

    // Apply the config values if provided
    // Set the number of users to start per second
    goose = goose.set_default(GooseDefault::StartupTime, config.starts_per_second)?;

    // Set the total number of users
    goose = goose.set_default(GooseDefault::Users, config.total_users)?;

    // Set the timeout
    goose = goose.set_default(GooseDefault::Timeout, config.timeout.as_str())?;

    // Set the runtime in seconds
    goose = goose.set_default(GooseDefault::RunTime, config.runtime)?;
    goose = goose.set_default(GooseDefault::StickyFollow, config.follow)?;

    // Execute the load test
    goose.execute().await?;
    Ok(())
}

// Sample transaction function
async fn loadtest_transaction(user: &mut GooseUser) -> TransactionResult {
    // Use the request details from Collection/Request objects
    // This is just a placeholder - replace with your actual request logic
    let _response = user.get("").await?;
    Ok(())
}
