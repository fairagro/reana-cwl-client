use crate::{api::client::ReanaClient, error::APIResult};
use reqwest::{Method, StatusCode};
use std::sync::Arc;

pub mod client;
pub mod response;
pub mod workflows;

pub const JSON_CONTENT_TYPE: &str = "application/json";
pub const OCTET_CONTENT_TYPE: &str = "application/octet-stream";

pub async fn ping(reana: Arc<ReanaClient>) -> APIResult<StatusCode> {
    let request = reana.build_request(Method::GET, "ping").await?;
    let response = request.send().await?.error_for_status()?;

    Ok(response.status())
}
