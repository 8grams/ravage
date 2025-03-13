use goose::{goose::GooseResponse, prelude::*};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::pin::Pin;

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

    let builder = reqwest::Client::builder();

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
        if user.weighted_users_index % 5 == 0 {
            let _ = sender.send(format!(
                "🔄 User {}: {} {}",
                user.weighted_users_index, req.method, req.path
            ));
        }
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
                if user.weighted_users_index % 5 == 0 {
                    let _ =
                        sender.send(format!("data: ✅ Success {}", r.response.unwrap().status()));
                }
            }
            Err(e) => {
                if user.weighted_users_index % 5 == 0 {
                    let _ = sender.send(format!("data: ❌ Failed, message: {}", e));
                }
            }
        }
    } else {
        let _ = user
            .set_client_builder(builder.default_headers(header_map))
            .await;
        if user.weighted_users_index % 5 == 0 {
            let _ = sender.send(format!(
                "🔄 User {}: GET {}",
                user.weighted_users_index, user.base_url
            ));
        }
        let result = user.get("").await;
        match result {
            Ok(r) => {
                if user.weighted_users_index % 5 == 0 {
                    let _ =
                        sender.send(format!("data: ✅ Success {}", r.response.unwrap().status()));
                }
            }
            Err(e) => {
                if user.weighted_users_index % 5 == 0 {
                    let _ = sender.send(format!("data: ❌ Error, message: {}", e));
                }
            }
        }
    }
    Ok(())
}
