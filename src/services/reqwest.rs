//! HTTP request service functions.
//! This module provides functions for sending HTTP requests using the reqwest client.

use reqwest::{Client, Method, Response};
use std::{collections::HashMap, error::Error};

use crate::models::{collection::Collection, request::Request};

/// Sends an HTTP request using the reqwest client
/// 
/// This function:
/// 1. Creates a new reqwest client
/// 2. Constructs the full URL from collection host and request path
/// 3. Sets up the request with method, headers, and body
/// 4. Sends the request and returns the response
/// 
/// # Arguments
/// * `request` - The request configuration containing method, path, body type and content
/// * `collection` - The collection containing the host URL
/// * `headers` - Optional headers to include in the request
/// 
/// # Returns
/// * `Result<Response, Box<dyn Error>>` - The HTTP response or error if request fails
/// 
/// # Supported Methods
/// * GET
/// * POST
/// * PUT
/// * DELETE
/// * PATCH
/// * HEAD
/// * OPTIONS
pub async fn send_request(
    request: &Request,
    collection: &Collection,
    headers: Option<HashMap<String, String>>,
) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}{}", collection.host, request.path);

    let method = match request.method.to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "PATCH" => Method::PATCH,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        _ => return Err("unsupported http method".into()),
    };

    let mut req_builder = client.request(method, &url);

    if let Some(hdrs) = headers {
        for (key, value) in hdrs {
            req_builder = req_builder.header(&key, &value);
        }
    }
    if let Some(body_type) = &request.body_type {
        req_builder = req_builder.header("content-type", body_type);
    }

    if let Some(body) = &request.body_content {
        req_builder = req_builder.body(body.clone());
    }

    let response = req_builder.send().await?;
    Ok(response)
}
