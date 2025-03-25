use goose::{goose::GooseResponse, prelude::*};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::pin::Pin;
use std::time::Duration;

use crate::{
    models::{request::Request, request_header::RequestHeader},
    services::goose_closure::GooseLoadConfig,
};

pub fn loadtest_transaction_wrapper(
    user: &mut GooseUser,
    config: GooseLoadConfig,
    request: Option<(Request, Vec<RequestHeader>)>,
) -> Pin<Box<dyn Future<Output = TransactionResult> + Send + '_>> {
    Box::pin(perform_request(user, config, request))
}

async fn perform_request(
    user: &mut GooseUser,
    config: GooseLoadConfig,
    request: Option<(Request, Vec<RequestHeader>)>,
) -> TransactionResult {
    let sender = config.sender.clone();

    let mut header_map: HeaderMap = HeaderMap::new();

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

    let builder = reqwest::Client::builder()
        .pool_max_idle_per_host(0)  // Disable connection pooling
        .pool_idle_timeout(None)    // Keep connections alive
        .tcp_nodelay(true)          // Disable Nagle's algorithm
        .tcp_keepalive(Some(Duration::from_secs(60)))  // Enable TCP keepalive
        .http2_prior_knowledge()    // Enable HTTP/2
        .gzip(true)                // Enable gzip compression
        .timeout(Duration::from_secs(30))  // Set timeout
        .pool_max_idle_per_host(100)  // Increase connection pool size
        .http1_title_case_headers(true)  // Optimize HTTP/1 headers
        .http1_preserve_header_case(true)  // Preserve header case
        .http1_only(false)  // Allow HTTP/2
        .use_rustls_tls()  // Use rustls for better performance
        .http2_initial_connection_window_size(1024 * 1024)  // Increase initial connection window size
        .http2_initial_stream_window_size(1024 * 1024)  // Increase initial stream window size
        .http2_max_concurrent_streams(100)  // Increase max concurrent streams
        .http2_max_frame_size(16384)  // Set max frame size
        .http2_max_header_list_size(262144);  // Increase max header list size

    if let Some((req, headers)) = request {
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
        let _ = sender.send(format!(
            "data: <pre><code>🔄 User {}: {} {}</code></pre>\n\n",
            user.weighted_users_index, req.method, req.path
        ));
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
        match result {
            Ok(r) => {
                match r.response {
                    Ok(response) => {
                        let _ = sender.send(format!(
                            "data: <pre><code>✅ Success {}</code></pre>\n\n",
                            response.status()
                        ));
                    }
                    Err(e) => {
                        let _ = sender.send(format!(
                            "data: <pre><code>❌ Response error: {}</code></pre>\n\n",
                            e
                        ));
                    }
                }
            }
            Err(e) => {
                let _ = sender.send(format!(
                    "data: <pre><code>❌ Failed, message: {}</code></pre>\n\n",
                    e
                ));
            }
        }
    } else {
        let _ = user
            .set_client_builder(builder.default_headers(header_map))
            .await;
        let _ = sender.send(format!(
            "data: <code>🔄 User {}: GET {}</code>\n\n",
            user.weighted_users_index, user.base_url
        ));
        let result = user.get("").await;
        match result {
            Ok(r) => {
                match r.response {
                    Ok(response) => {
                        let _ = sender.send(format!(
                            "data: <pre><code>✅ Success {}</code></pre>\n\n",
                            response.status()
                        ));
                    }
                    Err(e) => {
                        let _ = sender.send(format!(
                            "data: <pre><code>❌ Response error: {}</code></pre>\n\n",
                            e
                        ));
                    }
                }
            }
            Err(e) => {
                let _ = sender.send(format!(
                    "data: <pre><code>❌ Error, message: {}</code></pre>\n\n",
                    e
                ));
            }
        }
    }
    Ok(())
}
