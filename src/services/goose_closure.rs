use goose::prelude::*;
use std::{sync::Arc, time::Duration};

use crate::{
    models::{collection::Collection, header::Header, request::Request},
    utils::custom_transaction::loadtest_transaction_wrapper,
};

#[derive(Clone)]
pub struct LoadConfig {
    pub follow: bool,
    pub load_test_id: i32,
    pub launch_all_users: usize,
    pub total_users: usize,
    pub timeout: String,
    pub runtime: usize,
    pub log_path: String,
    pub report_path: String,
}

#[derive(Clone)]
pub struct GooseLoadConfig {
    pub load_config: LoadConfig,
    pub sender: tokio::sync::broadcast::Sender<String>,
    pub collection: Collection,
    pub requests: Option<Vec<Request>>,
    pub headers: Option<Vec<Header>>,
}

pub async fn goose_closuer_load_test(config: GooseLoadConfig) {
    tokio::spawn(async move {
        if let Err(e) = run_loadtest(config).await {
            eprintln!("Goose load test failed: {}", e);
        }
    });
}

async fn run_loadtest(config: GooseLoadConfig) -> Result<(), GooseError> {
    let config_clone = config.clone();

    let mut scenario = scenario!("WebsiteUser")
        // After each transaction runs, sleep randomly from 5 to 15 seconds.
        .set_wait_time(Duration::from_secs(5), Duration::from_secs(15))?;
    if let Some(requests) = config.requests.clone() {
        for request in requests {
            let config = config_clone.clone();
            let trans = Transaction::new(Arc::new(move |user| {
                loadtest_transaction_wrapper(user, config.clone(), Some(request.clone()))
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
        .set_default(
            GooseDefault::StartupTime,
            config.load_config.launch_all_users,
        )?
        .set_default(GooseDefault::Users, config.load_config.total_users)?
        .set_default(GooseDefault::RunTime, config.load_config.runtime)?
        .set_default(GooseDefault::StickyFollow, config.load_config.follow)?;

    let _ = goose.execute().await?;
    Ok(())
}
