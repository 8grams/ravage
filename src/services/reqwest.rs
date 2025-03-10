use reqwest::{Client, Method, Response};
use std::{collections::HashMap, error::Error};

use crate::models::{collection::Collection, request::Request};

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

    if let Some(body) = &request.body_content {
        req_builder = req_builder.body(body.clone());
    }

    let response = req_builder.send().await?;
    Ok(response)
}
