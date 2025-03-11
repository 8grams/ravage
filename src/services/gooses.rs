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
    pub timeout: usize,
    pub runtime: usize,
}

pub async fn goose_loadtest(
    collection: Collection,
    request: Option<Request>,
    config: Option<LoadTestConfig>,
) {
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let report_file_name = format!("report_{}.html", timestamp);
    let log_file_name = format!("log_{}.txt", timestamp);
    let log_dir = "./static/log";
    let report_dir = "./static/report";

    if let Err(e) = create_dir_all(log_dir) {
        eprintln!("Failed to create log directory: {}", e);
        return;
    }
    if let Err(e) = create_dir_all(report_dir) {
        eprintln!("Failed to create report directory: {}", e);
        return;
    }

    // Spawn the load test in the background
    spawn(async move {
        if let Err(e) = run_goose_loadtest(
            collection,
            request,
            config,
            report_file_name,
            log_file_name,
            report_dir,
            log_dir,
        )
        .await
        {
            eprintln!("Goose load test failed: {}", e);
        }
    });
}

async fn run_goose_loadtest(
    collection: Collection,
    request: Option<Request>,
    config: Option<LoadTestConfig>,
    report_file_name: String,
    log_file_name: String,
    report_dir: &str,
    log_dir: &str,
) -> Result<(), goose::GooseError> {
    // Start building the GooseAttack configuration
    let mut goose = GooseAttack::initialize()?
        .register_scenario(
            scenario!("LoadtestTransactions")
                .register_transaction(transaction!(loadtest_transaction)),
        )
        .set_default(GooseDefault::Host, collection.host.as_str())?
        .set_default(
            GooseDefault::ReportFile,
            format!("{}/{}", report_dir, report_file_name).as_str(),
        )?
        .set_default(
            GooseDefault::RequestLog,
            format!("{}/{}", log_dir, log_file_name).as_str(),
        )?;

    // Apply the config values if provided
    if let Some(config) = config {
        // Set the number of users to start per second
        goose = goose.set_default(GooseDefault::StartupTime, config.starts_per_second)?;

        // Set the total number of users
        goose = goose.set_default(GooseDefault::Users, config.total_users)?;

        // Set the timeout
        // goose = goose.set_default(GooseDefault::Timeout, config.timeout)?;

        // Set the runtime in seconds
        goose = goose.set_default(GooseDefault::RunTime, config.runtime)?;
        goose = goose.set_default(GooseDefault::StickyFollow, config.follow)?;
    } else {
        // Default runtime if no config provided
        goose = goose.set_default(GooseDefault::RunTime, 10)?;
    }

    // Execute the load test
    goose.execute().await?;
    Ok(())
}

// Sample transaction function
async fn loadtest_transaction(user: &mut GooseUser) -> TransactionResult {
    // Access load_test_id from user data if needed
    // if let Some(user_data) = user.get_session_data() {
    //     if let Ok(data) = serde_json::from_str::<serde_json::Value>(user_data) {
    //         if let Some(load_test_id) = data.get("load_test_id") {
    //             // You can use the load_test_id here if needed
    //             println!("Running test with ID: {}", load_test_id);
    //         }
    //     }
    // }

    // Use the request details from Collection/Request objects
    // This is just a placeholder - replace with your actual request logic
    let _response = user.get("").await?;
    Ok(())
}
