use crate::client;
use miette::IntoDiagnostic;
use reana::api::{self};
use serde::Deserialize;
use tracing::info;

pub mod workflows;

/// Pings the REANA API using client
/// # Errors
/// Returns Error if the request fails
pub async fn ping() -> miette::Result<()> {
    let client = client()?;
    let code = api::ping(client).await.into_diagnostic()?;
    info!("Server returned {code}");
    Ok(())
}

#[derive(Deserialize)]
struct WorkflowLogs {
    job_logs: std::collections::BTreeMap<String, JobLog>,
    workflow_logs: Option<String>,
}

#[derive(Deserialize)]
struct JobLog {
    job_name: String,
    status: String,
    docker_img: String,
    started_at: Option<String>,
    finished_at: Option<String>,
    logs: String,
}
