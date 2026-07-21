use crate::client;
use miette::IntoDiagnostic;
use reana::api;
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
