/**
 * Flex Default Policy
 *
 * <p> This policy is used to log the request and response details.
 * It logs the request headers, body, and response headers, body.
 * It also logs the correlation ID and request ID from the headers.
 * It is used to debug the requests and responses in the Flex platform.</p>
 *
 * Author: Shashank
 */

mod generated;
mod policy_processor;

use anyhow::{anyhow, Result};
use pdk::hl::*;
use pdk::logger;
use serde::{Deserialize, Serialize};

use crate::generated::config::Config;

#[derive(Serialize, Deserialize)]
pub struct ApiRequestData {
    corelation_id: String,
    request_id: String,
}

async fn log_request(request_state: RequestState, _config: &Config) -> ApiRequestData {
    let header_state = request_state.into_headers_state().await;
    let headers = policy_processor::get_request_headers(&header_state);

    let body_state = header_state.into_body_state().await;
    let body = policy_processor::get_request_body(&body_state).await;

    let corelation_id = policy_processor::get_header_value(&headers, "x-correlation-id");
    let request_id = policy_processor::get_header_value(&headers, "x-request-id");

    let log_message = format!(
        "Request Details: Request ID: {}, Correlation ID: {:?}, Headers: {:?}, Body: {}",
        request_id, corelation_id, headers, body
    );

    let request_data = ApiRequestData {
        corelation_id: corelation_id.clone(),
        request_id: request_id.clone(),
    };

    if _config.log_debug {
        logger::debug!("{}", log_message);
    }

    return request_data;
}
async fn log_response(
    response_state: ResponseState,
    request_data: ApiRequestData,
    _config: &Config,
) {
    let header_state = response_state.into_headers_state().await;
    let headers = policy_processor::get_response_headers(&header_state);

    let body_state = header_state.into_body_state().await;
    let body = policy_processor::get_response_body(&body_state).await;

    let corelation_id = request_data.corelation_id;
    let request_id = request_data.request_id;

    let log_message = format!(
        "Response Details: Request ID: {}, Correlation ID: {:?}, Headers: {:?}, Body: {}",
        request_id, corelation_id, headers, body
    );

    if _config.log_debug {
        logger::debug!("{}", log_message);
    }
}

/// Function that will handle the request part of the requests.
async fn request_filter(state: RequestState, config: &Config) -> Flow<ApiRequestData> {
    let request_data = log_request(state, config).await;
    Flow::Continue(request_data)
}

/// Function that will handle the response part of the requests.
async fn response_filter(
    state: ResponseState,
    request_data: RequestData<ApiRequestData>,
    config: &Config,
) {
    let RequestData::Continue(req_data) = request_data else {
        logger::debug!("No request data found!");
        return;
    };

    log_response(state, req_data, config).await;
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config: Config = serde_json::from_slice(&bytes).map_err(|err| {
        anyhow!(
            "Failed to parse configuration '{}'. Cause: {}",
            String::from_utf8_lossy(&bytes),
            err
        )
    })?;

    logger::info!("Configuring Flex Default Policy");

    let filter = on_request(|request_state| request_filter(request_state, &config))
        .on_response(|rs, rd| response_filter(rs, rd, &config));
    launcher.launch(filter).await?;

    Ok(())
}
