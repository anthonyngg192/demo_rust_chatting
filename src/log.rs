use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::error::ClientError;
use crate::{Error, Result};

pub async fn log_request(
    uuid: Uuid,
    request_method: Method,
    uri: Uri,
    service_error: Option<&Error>,
    client_error: Option<&ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = service_error.map(|f| f.as_ref().to_string());

    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut f| f.get_mut("data").map(|f| f.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),
        req_path: uri.to_string(),
        req_method: request_method.to_string(),
        error_type,
        error_data,
        client_error_type: client_error.map(|f| f.as_ref().to_string()),
    };
    println!("   ->> log_request: \n{}", json!(log_line));

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      // uuid string formatted
    timestamp: String, // (should be iso8601)

    // -- http request attributes.
    req_path: String,
    req_method: String,

    // -- Errors attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
