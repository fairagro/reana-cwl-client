use crate::{api::client::ReanaClient, error::APIResult};
use reqwest::{Method, Response, StatusCode};
use std::sync::Arc;
use tracing::{debug, error};

pub mod client;
pub mod response;
pub mod workflows;

pub const JSON_CONTENT_TYPE: &str = "application/json";
pub const OCTET_CONTENT_TYPE: &str = "application/octet-stream";

pub async fn ping(reana: Arc<ReanaClient>) -> APIResult<StatusCode> {
    let request = reana.build_request(Method::GET, "ping").await?;
    debug!("Request: {request:?}");

    let response = request.send().await?;
    if let Err(err) = response.error_for_status_ref() {
        report(response).await;
        return Err(err.into());
    };

    Ok(response.status())
}

pub async fn report(response: Response) {
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    error!("REANA request failed ({status}): {body}");
}
