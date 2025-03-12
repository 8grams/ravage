use crate::models::{collection::Collection, request::Request};
use goose::prelude::*;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tokio::sync::broadcast::Sender;
use tokio::task::spawn;

#[derive(Clone, Debug)]
pub struct LoadTestConfig {
    pub follow: bool,
    pub load_test_id: i32,
    pub launch_all_users: usize,
    pub total_users: usize,
    pub timeout: String,
    pub runtime: usize,
    pub log_path: String,
    pub report_path: String,
    pub headers: Option<HashMap<String, String>>,
}

// Global static OnceLock to store the request and log sender safely
static REQUEST_DATA: OnceLock<Arc<RwLock<Option<Request>>>> = OnceLock::new();
static LOG_SENDER: OnceLock<Arc<RwLock<Option<Sender<String>>>>> = OnceLock::new();
static HEADERS_DATA: OnceLock<Arc<RwLock<Option<HashMap<String, String>>>>> = OnceLock::new();

pub async fn goose_loadtest(
    collection: Collection,
    request: Option<Request>,
    config: LoadTestConfig,
    sender: Sender<String>,
) {
    let request_data = REQUEST_DATA.get_or_init(|| Arc::new(RwLock::new(None)));
    let log_sender = LOG_SENDER.get_or_init(|| Arc::new(RwLock::new(None)));
    let headers_data = HEADERS_DATA.get_or_init(|| Arc::new(RwLock::new(None)));

    // Store the new request if provided
    if let Some(req) = request {
        let mut stored_request = request_data.write().await;
        *stored_request = Some(req);
    }

    if let Some(headers) = config.clone().headers {
        let mut stored_headers = headers_data.write().await;
        *stored_headers = Some(headers);
    }

    // Store the new sender
    let mut log_sender_lock = log_sender.write().await;
    *log_sender_lock = Some(sender);

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
                .register_transaction(transaction!(loadtest_transaction_repeat)),
        )
        .set_default(GooseDefault::Host, collection.host.as_str())?
        .set_default(GooseDefault::ReportFile, config.report_path.as_str())?
        .set_default(GooseDefault::RequestLog, config.log_path.as_str())?;

    goose = goose.set_default(GooseDefault::StartupTime, config.launch_all_users)?;
    goose = goose.set_default(GooseDefault::Users, config.total_users)?;
    goose = goose.set_default(GooseDefault::Timeout, config.timeout.as_str())?;
    goose = goose.set_default(GooseDefault::RunTime, config.runtime)?;
    goose = goose.set_default(GooseDefault::StickyFollow, config.follow)?;

    let log_sender = LOG_SENDER.get().expect("FAILED LOG SENDER");

    let result = goose.execute().await;

    match result {
        Ok(metrics) => {
            if let Some(sender) = log_sender.read().await.clone() {
                let _ = sender.send(format!(
                    "✅ Load test completed in {}s with {} users",
                    metrics.duration, metrics.total_users
                ));
            }
        }
        Err(error) => {
            if let Some(sender) = log_sender.read().await.clone() {
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
    let log_sender = LOG_SENDER.get().expect("FAILED LOG SENDER");
    let header_data = HEADERS_DATA.get().expect("FAILED HEADERS DATA");

    let mut header_map = HeaderMap::new();

    if let Some(headers) = header_data.read().await.clone() {
        for (key, value) in headers {
            if let (Ok(header_name), Ok(header_value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(&value),
            ) {
                header_map.insert(header_name, header_value);
            }
        }
    }

    let builder = reqwest::Client::builder().default_headers(header_map);
    let _ = user.set_client_builder(builder).await;

    if let Some(request) = request_data.read().await.as_ref() {
        let path = &request.path;
        let method = request.method.to_uppercase();
        let body_content = request.body_content.clone().unwrap_or_default();
        let body_type = request.body_type.clone().unwrap_or_default();

        let sender = log_sender.read().await.clone();

        if user.weighted_users_index % 5 == 0 {
            if let Some(sender) = sender.as_ref() {
                let _ = sender.send(format!(
                    "🔄 User {}: {} {}",
                    user.weighted_users_index, method, path
                ));
            }
        }

        let result = match method.as_str() {
            "POST" => match body_type.as_str() {
                "application/json" => {
                    println!("{}", body_content);
                    user.post_json(
                        path,
                        &serde_json::from_str::<serde_json::Value>(&body_content)
                            .unwrap_or_default(),
                    )
                    .await
                }
                _ => user.post(path, body_content).await,
            },
            _ => user.get(path).await,
        };

        if let Some(sender) = sender.as_ref() {
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
        let sender = log_sender.read().await.clone();

        if user.weighted_users_index < 3 {
            if let Some(sender) = sender.as_ref() {
                let _ = sender.send(format!(
                    "🔄 User {}: Default request",
                    user.weighted_users_index
                ));
            }
        }

        if let Err(e) = result {
            if let Some(sender) = sender.as_ref() {
                let _ = sender.send(format!(
                    "❌ User {}: Error - {}",
                    user.weighted_users_index, e
                ));
            }
        }
    }

    Ok(())
}
