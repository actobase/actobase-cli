use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde_json::Value;

pub fn post_json_with_api_key(
    base_url: &str,
    path: &str,
    payload: &Value,
    api_key: Option<&str>,
) -> Result<Value, String> {
    let client = Client::new();
    let mut request = client.post(join_url(base_url, path)).json(payload);
    if let Some(api_key) = api_key {
        request = request.header(AUTHORIZATION, format!("Bearer {}", api_key));
    }
    let response = request
        .send()
        .map_err(|error| format!("post {}{}: {}", base_url, path, error))?;

    read_json_response(response)
}

pub fn get_json_with_api_key(base_url: &str, path: &str, api_key: &str) -> Result<Value, String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {}", api_key);
    let auth_value =
        HeaderValue::from_str(&bearer).map_err(|error| format!("invalid api key: {}", error))?;
    headers.insert(AUTHORIZATION, auth_value);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let response = client
        .get(join_url(base_url, path))
        .headers(headers)
        .send()
        .map_err(|error| format!("get {}{}: {}", base_url, path, error))?;

    read_json_response(response)
}

fn join_url(base_url: &str, path: &str) -> String {
    format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

fn read_json_response(response: reqwest::blocking::Response) -> Result<Value, String> {
    let status = response.status();
    let body = response
        .text()
        .map_err(|error| format!("read response body: {}", error))?;
    let payload: Value = serde_json::from_str(&body).map_err(|error| {
        format!(
            "parse json response (status {}): {} body={}",
            status.as_u16(),
            error,
            body
        )
    })?;

    if status.is_success() {
        Ok(payload)
    } else {
        Err(format!(
            "request failed with status {}: {}",
            status.as_u16(),
            serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string())
        ))
    }
}
