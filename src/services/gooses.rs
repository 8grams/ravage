use chrono::Utc;
use goose::prelude::*;
use std::fs::{create_dir_all, write};
use tokio::task::spawn;

use crate::models::{collection::Collection, request::Request};

pub struct LoadTestConfig {
    user_count: i32,
}

pub async fn goose_loadtest(collection: Collection, request: Option<Request>) {
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
    report_file_name: String,
    log_file_name: String,
    report_dir: &str,
    log_dir: &str,
) -> Result<(), goose::GooseError> {
    let mut goose = GooseAttack::initialize()?
        .register_scenario(
            scenario!("LoadtestTransactions")
                .register_transaction(transaction!(loadtest_transaction)),
        )
        .set_default(GooseDefault::Host, collection.host.as_str())?
        .set_default(GooseDefault::RunTime, 10)?
        .set_default(
            GooseDefault::ReportFile,
            format!("{}/{}", report_dir, report_file_name).as_str(),
        )?
        .set_default(
            GooseDefault::RequestLog,
            format!("{}/{}", log_dir, log_file_name).as_str(),
        )?;

    goose.execute().await?;
    Ok(())
}

// Sample transaction function
async fn loadtest_transaction(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/").await?;
    Ok(())
}
