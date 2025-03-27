use goose::prelude::*;
use std::{sync::Arc, time::Duration};

use crate::{
    models::{
        collection::Collection, header::Header, request::Request, request_header::RequestHeader,
    },
    utils::custom_transaction::loadtest_transaction_wrapper,
};

use super::websocket::server_handler::LogServerHandler;

#[derive(Clone)]
pub struct LoadConfig {
    pub follow: bool,
    pub launch_all_users: usize,
    pub total_users: usize,
    pub timeout: String,
    pub hatch_rate: String,
    pub runtime: usize,
    pub log_path: String,
    pub report_path: String,
}

#[derive(Clone)]
pub struct GooseLoadConfig {
    pub load_config: LoadConfig,
    pub sender: LogServerHandler,
    pub collection: Collection,
    pub requests: Option<Vec<(Request, Vec<RequestHeader>)>>,
    pub headers: Option<Vec<Header>>,
}

pub async fn goose_closure_load_test(config: GooseLoadConfig) {
    tokio::spawn(async move {
        if let Err(e) = run_loadtest(config.clone()).await {
            let sender = config.sender;
            let _ = sender.send_message(
                config.collection.id,
                format!("Goose load test failed: {}", e),
            );
            eprintln!("Goose load test failed: {}", e);
        }
    });
}

async fn run_loadtest(config: GooseLoadConfig) -> Result<(), GooseError> {
    let config_clone = config.clone();

    let mut scenario = scenario!("WebsiteUser")
        // After each transaction runs, sleep randomly from 0.01 to 0.1 seconds.
        .set_wait_time(Duration::from_millis(10), Duration::from_millis(100))?;

    // if request
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
        // .set_default(GooseDefault::StickyFollow, config.load_config.follow)?
        // Performance optimizations
        // .set_default(GooseDefault::ThrottleRequests, 0)?
        // .set_default(GooseDefault::NoStatusCodes, true)?
        // .set_default(GooseDefault::RunningMetrics, 1)?
        .set_default(GooseDefault::NoResetMetrics, true)?;
    // .set_default(GooseDefault::NoMetrics, false)?
    // .set_default(GooseDefault::NoErrorSummary, false)?
    // .set_default(GooseDefault::NoAutoStart, true)?  // Prevent auto-start

    let result = goose.execute().await;

    let sender = config.sender;
    match result {
        Ok(metrics) => {
            let _ = sender.send_message(
                config.collection.id,
                format!(
                    "<pre><code>✅ Load test completed in {}s with {} users</code></pre>",
                    metrics.duration, metrics.total_users
                ),
            );
        }
        Err(error) => {
            let _ = sender.send_message(
                config.collection.id,
                format!("<pre><code>❌ Loadtest failed: {}</code></pre>", error),
            );
        }
    }
    Ok(())
}
