use miette::IntoDiagnostic;
use reana::api;

use crate::client;

pub async fn ping() -> miette::Result<()> {
    let client = client()?;
    api::ping(client).await.into_diagnostic()
}
