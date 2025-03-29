//! Custom transaction handling for load testing.
//! This module provides functionality for executing HTTP requests during load tests with proper error handling and logging.

use goose::{goose::GooseResponse, prelude::*};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::pin::Pin;
use std::time::Duration;

use crate::{
    models::{request::Request, request_header::RequestHeader},
    services::goose_closure::GooseLoadConfig,
};

/// Wraps a load test transaction with proper error handling and logging
/// 
/// This function creates a future that will execute the HTTP request with the given configuration.
/// It handles both specific requests and default GET requests to the base URL.
/// 
/// # Arguments
/// * `user` - The Goose user executing the transaction
/// * `config` - Configuration for the load test including headers and logging
/// * `request` - Optional specific request to execute, or None for default GET
/// 
/// # Returns
/// * `Pin<Box<dyn Future<Output = TransactionResult> + Send + '_>>` - A future that will execute the request
pub fn loadtest_transaction_wrapper(
    user: &mut GooseUser,
    config: GooseLoadConfig,
    request: Option<(Request, Vec<RequestHeader>)>,
) -> Pin<Box<dyn Future<Output = TransactionResult> + Send + '_>> {
    Box::pin(perform_request(user, config, request))
}

/// Performs the actual HTTP request with proper error handling and logging
/// 
/// This function:
/// 1. Sets up HTTP headers from both global and request-specific configurations
/// 2. Configures the HTTP client with optimized settings for load testing
/// 3. Executes the request with proper error handling
/// 4. Logs the results through the provided sender
/// 
/// # Arguments
/// * `user` - The Goose user executing the transaction
/// * `config` - Configuration for the load test including headers and logging
/// * `request` - Optional specific request to execute, or None for default GET
/// 
/// # Returns
/// * `TransactionResult` - The result of the transaction execution
async fn perform_request(
    user: &mut GooseUser,
    config: GooseLoadConfig,
    request: Option<(Request, Vec<RequestHeader>)>,
) -> TransactionResult {
    let sender = config.sender.clone();

    // Initialize header map for the request
    let mut header_map: HeaderMap = HeaderMap::new();

    // Add global headers if present
    if let Some(headers) = config.headers {
        for h in headers {
            if let (Ok(header_name), Ok(header_value)) = (
                HeaderName::from_bytes(h.key.as_bytes()),
                HeaderValue::from_str(&h.value),
            ) {
                header_map.insert(header_name, header_value);
            }
        }
    }

    // Configure HTTP client with optimized settings
    let builder = reqwest::Client::builder()
        .pool_max_idle_per_host(0) // Disable connection pooling
        .pool_idle_timeout(None) // Keep connections alive
        .tcp_nodelay(true) // Disable Nagle's algorithm
        .tcp_keepalive(Some(Duration::from_secs(60))) // Enable TCP keepalive
        .http2_prior_knowledge() // Enable HTTP/2
        .gzip(true) // Enable gzip compression
        .timeout(Duration::from_secs(30)) // Set timeout
        .pool_max_idle_per_host(100) // Increase connection pool size
        .http1_title_case_headers() // Optimize HTTP/1 headers
        .use_rustls_tls() // Use rustls for better performance
        .http2_initial_connection_window_size(1024 * 1024) // Increase initial connection window size
        .http2_initial_stream_window_size(1024 * 1024) // Increase initial stream window size
        .http2_max_frame_size(16384) // Set max frame size
        .http2_max_header_list_size(262144); // Increase max header list size

    // Handle specific request if provided
    if let Some((req, headers)) = request {
        // Add request-specific headers
        for h in headers {
            if let (Ok(header_name), Ok(header_value)) = (
                HeaderName::from_bytes(h.key.as_bytes()),
                HeaderValue::from_str(&h.value),
            ) {
                header_map.insert(header_name, header_value);
            }
        }
        let _ = user
            .set_client_builder(builder.default_headers(header_map))
            .await;
        let _ = sender.send_message(
            config.load_test_id,
            format!(
                "<div id='logs' hx-swap-oob='beforeend'><pre><code>🔄 User {}: {} {}</code></pre></div>",
                user.weighted_users_index, req.method, req.path
            ),
        ).await;
        let result: Result<GooseResponse, _> = match req.method.to_uppercase().as_str() {
            "POST" => {
                if let Some(body_type) = req.body_type {
                    match body_type.to_lowercase().as_str() {
                        "application/json" => {
                            user.post_json(
                                &req.path,
                                &serde_json::from_str::<serde_json::Value>(
                                    &req.body_content.clone().unwrap_or_default(),
                                )
                                .unwrap_or_default(),
                            )
                            .await
                        }
                        "application/x-www-form-urlencoded" => {
                            user.post_form(&req.path, &req.body_content.unwrap_or_default())
                                .await
                        }
                        _ => {
                            user.post(&req.path, req.body_content.clone().unwrap_or_default())
                                .await
                        }
                    }
                } else {
                    user.post(&req.path, req.body_content.unwrap_or_default())
                        .await
                }
            }
            "DELETE" => user.delete(&req.path).await,
            _ => user.get(&req.path).await,
        };
        // Handle response and log results
        match result {
            Ok(r) => match r.response {
                Ok(response) => {
                    let _ = sender.send_message(
                        config.load_test_id,
                        format!(
                            "<div id='logs' hx-swap-oob='beforeend'><pre><code>✅ Success {}</code></pre></div>",
                            response.status()
                        ),
                    ).await;
                }
                Err(e) => {
                    let _ = sender.send_message(
                        config.load_test_id,
                        format!("<div id='logs' hx-swap-oob='beforeend'><pre><code>❌ Response error: {}</code></pre></div>", e),
                    ).await;
                }
            },
            Err(e) => {
                let _ = sender.send_message(
                    config.load_test_id,
                    format!("<div id='logs' hx-swap-oob='beforeend'><pre><code>❌ Failed, message: {}</code></pre></div>", e),
                ).await;
            }
        }
    } else {
        // Handle default GET request to base URL
        let _ = user
            .set_client_builder(builder.default_headers(header_map))
            .await;
        let _ = sender.send_message(
            config.load_test_id,
            format!(
                "<div id='logs' hx-swap-oob='beforeend'><pre><code>🔄 User {}: GET {}</code></pre></div>",
                user.weighted_users_index, user.base_url
            ),
        ).await;
        let result = user.get("").await;
        match result {
            Ok(r) => match r.response {
                Ok(response) => {
                    let _ = sender.send_message(
                        config.load_test_id,
                        format!(
                            "<div id='logs' hx-swap-oob='beforeend'><pre><code>✅ Success {}</code></pre></div>",
                            response.status()
                        ),
                    ).await;
                }
                Err(e) => {
                    let _ = sender.send_message(
                        config.load_test_id,
                        format!("<div id='logs' hx-swap-oob='beforeend'><pre><code>❌ Response error: {}</code></pre></div>", e),
                    );
                }
            },
            Err(e) => {
                let _ = sender.send_message(
                    config.load_test_id,
                    format!("<div id='logs' hx-swap-oob='beforeend'><pre><code>❌ Error, message: {}</code></pre></div>", e),
                ).await;
            }
        }
    }
    Ok(())
}
