// Copyright 2023 Salesforce, Inc. All rights reserved.
mod generated;
mod polict_constants;
use anyhow::{anyhow, Result};

use pdk::classy::proxy_wasm::types::Status;
use pdk::hl::*;
use pdk::logger;
use serde::{Deserialize, Serialize};

use crate::generated::config::Config;

pub enum FilterError {
    StatusError(u32),
    ClientError(HttpClientError),
    NonParsableResponseBody(serde_json::Error),
    PropogationError(String),
}

#[derive(Deserialize)]
pub struct PolicyApiResponse {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientErrorResponse {
    pub message: String,
}

async fn invoke_policy_api(
    config: &Config,
    client: HttpClient,
) -> Result<PolicyApiResponse, FilterError> {
    let response = client
        .request(&config.external_service)
        .get()
        .await
        .map_err(FilterError::ClientError)?;

    if response.status_code() == 200 {
        serde_json::from_slice(response.body()).map_err(FilterError::NonParsableResponseBody)
    } else {
        Err(FilterError::StatusError(response.status_code()))
    }
}

async fn process_response(response: PolicyApiResponse) -> Result<(), FilterError> {
    logger::info!(
        "Policy API response received: id={}, name={}",
        response.id,
        response.name
    );
    Ok(())
}

async fn do_filter(config: &Config, client: HttpClient) -> Result<(), FilterError> {
    let response = invoke_policy_api(config, client).await?;
    process_response(response).await?;
    Ok(())
}
fn generate_error_response(status_code: u32) -> Flow<()> {
    let client_error_response = ClientErrorResponse {
        message: format!("An error occurred while processing the request. Status code: {}", status_code),
    };
    let body = serde_json::to_vec(&client_error_response).unwrap_or_else(|_| b"{\"message\":\"Internal Server Error\"}".to_vec());
    Flow::Break(Response::new(status_code).with_body(body))
}
/// Defines a filter function that works as a wrapper for the real filter function that enables simplified error handling
async fn request_filter(state: RequestState, client: HttpClient, config: &Config) -> Flow<()> {
    let state = state.into_headers_state().await;

    match do_filter(config, client).await {
        Ok(_) => Flow::Continue(()),
        Err(err) => match err {
            FilterError::StatusError(status_code) => generate_error_response(status_code),
            FilterError::ClientError(http_client_error) => generate_error_response(500),
            FilterError::NonParsableResponseBody(error) => generate_error_response(500),
            FilterError::PropogationError(_) => generate_error_response(500),
        },
    }
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
    let filter = on_request(|request, client| request_filter(request, client, &config));

    launcher.launch(filter).await?;
    Ok(())
}
