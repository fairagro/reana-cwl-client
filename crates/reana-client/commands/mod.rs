use miette::IntoDiagnostic;
use reana::api;
use tracing::info;

pub mod workflows;

use crate::client;

pub async fn ping() -> miette::Result<()> {
    let client = client()?;
    let code = api::ping(client).await.into_diagnostic()?;
    info!("Server returned {code}");
    Ok(())
}
