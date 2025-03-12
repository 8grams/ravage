use crate::models::{collection::Collection, request::Request};
use goose::prelude::*;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
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
static REQUEST_DATA: OnceLock<Arc<RwLock<Option<Request>>>> = OnceLock::new();
static LOG_SENDER: OnceLock<Sender<String>> = OnceLock::new();

pub async fn goose_loadtest(
    collection: Collection,
    request: Option<Request>,
    config: LoadTestConfig,
    sender: Sender<String>,
) {
    let request_data = REQUEST_DATA.get_or_init(|| Arc::new(RwLock::new(None)));
    if let Some(req) = request {
        let mut stored_request = request_data.write().await;
        *stored_request = Some(req);
    }
    let _ = LOG_SENDER.set(sender);

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
    let mut goose = GooseAttack::initialize()?
        .register_scenario(
            scenario!("LoadtestTransactions")
                .register_transaction(transaction!(loadtest_transaction).set_on_start())
                .register_transaction(transaction!(loadtest_transaction_repeat)),
        )
        .set_default(GooseDefault::Host, collection.host.as_str())?
        .set_default(GooseDefault::ReportFile, config.report_path.as_str())?
        .set_default(GooseDefault::RequestLog, config.log_path.as_str())?;

    goose = goose.set_default(GooseDefault::StartupTime, config.starts_per_second)?;
    goose = goose.set_default(GooseDefault::Users, config.total_users)?;
    goose = goose.set_default(GooseDefault::Timeout, config.timeout.as_str())?;
    goose = goose.set_default(GooseDefault::RunTime, config.runtime)?;
    goose = goose.set_default(GooseDefault::StickyFollow, config.follow)?;

    if let Some(sender) = LOG_SENDER.get() {
        let _ = sender.send(format!(
            "🚀 Loadtest starting with {} users",
            config.total_users
        ));
    }

    let result = goose.execute().await;

    match result {
        Ok(metrics) => {
            if let Some(sender) = LOG_SENDER.get() {
                let _ = sender.send(format!(
                    "✅ Load test completed in {}s with {} users",
                    metrics.duration, metrics.total_users
                ));
            }
        }
        Err(error) => {
            if let Some(sender) = LOG_SENDER.get() {
                let _ = sender.send(format!("❌ Loadtest failed: {}", error));
            }
        }
    }

    Ok(())
}

async fn loadtest_transaction(user: &mut GooseUser) -> TransactionResult {
    perform_request(user).await
}

async fn loadtest_transaction_repeat(user: &mut GooseUser) -> TransactionResult {
    perform_request(user).await
}

async fn perform_request(user: &mut GooseUser) -> TransactionResult {
    let request_data = REQUEST_DATA.get().expect("FAILED REQUEST DATA");
    if let Some(request) = request_data.read().await.as_ref() {
        let path = &request.path;
        let method = request.method.to_uppercase();
        let body_content = request.body_content.clone().unwrap_or_default();

        if user.weighted_users_index % 5 == 0 {
            if let Some(sender) = LOG_SENDER.get() {
                let _ = sender.send(format!(
                    "🔄 User {}: {} {}",
                    user.weighted_users_index, method, path
                ));
            }
        }

        let result = match method.as_str() {
            "POST" => user.post(path, body_content).await,
            _ => user.get(path).await,
        };

        if let Some(sender) = LOG_SENDER.get() {
            match result {
                Ok(_) if user.weighted_users_index % 5 == 0 => {
                    let _ = sender.send(format!("✅ User {}: Success", user.weighted_users_index));
                }
                Err(e) => {
                    let _ = sender.send(format!(
                        "❌ User {}: Error - {}",
                        user.weighted_users_index, e
                    ));
                }
                _ => {}
            }
        }
    } else {
        let result = user.get("/").await;
        if user.weighted_users_index < 3 {
            if let Some(sender) = LOG_SENDER.get() {
                let _ = sender.send(format!(
                    "🔄 User {}: Default request",
                    user.weighted_users_index
                ));
            }
        }

        if let Err(e) = result {
            if let Some(sender) = LOG_SENDER.get() {
                let _ = sender.send(format!(
                    "❌ User {}: Error - {}",
                    user.weighted_users_index, e
                ));
            }
        }
    }

    Ok(())
}
