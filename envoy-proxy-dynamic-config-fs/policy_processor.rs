use pdk::hl::*;
use serde_json::Value;

pub fn get_request_headers(header_state: &RequestHeadersState) -> Vec<(String, String)> {
    return header_state.handler().headers();
}
pub fn get_response_headers(header_state: &ResponseHeadersState) -> Vec<(String, String)> {
    return header_state.handler().headers();
}


pub async fn get_request_body(body_state: &RequestBodyState) -> String {
    let body = body_state.handler().body();

    match serde_json::from_slice::<Value>(&body) {
        Ok(json_value) => {
            return json_value.to_string();
        }
        Err(err) => {
            return "Body parse error ".to_string() + &err.to_string();
        }
    }
}

pub async fn get_response_body(body_state: &ResponseBodyState) -> String {
    let body = body_state.handler().body();

    match serde_json::from_slice::<Value>(&body) {
        Ok(json_value) => {
            return json_value.to_string();
        }
        Err(err) => {
            return "Body parse error ".to_string() + &err.to_string();
        }
    }
}

pub fn get_header_value(headers: &Vec<(String, String)>, key: &str) -> String {
    for (header_key, header_value) in headers {
        if header_key.eq_ignore_ascii_case(key) {
            return header_value.to_string();
        }
    }
    return String::new();
}