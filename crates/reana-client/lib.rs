use std::{env, sync::Arc};

use miette::IntoDiagnostic;
use reana::api::client::ReanaClient;
use reana::auth::ReanaAccessToken;
use url::Url;
pub mod cli;
pub mod commands;

/// Returns an REANA API Client based on environment variables
/// # Errors
/// Returns Error if the given URL is invalid
pub fn client() -> miette::Result<Arc<ReanaClient>> {
    let token = env::var("REANA_TOKEN").into_diagnostic()?;
    let url = env::var("REANA_URL").into_diagnostic()?;

    let client = ReanaClient::new(
        Url::parse(&url)
            .into_diagnostic()?
            .join("api")
            .into_diagnostic()?,
        Arc::new(ReanaAccessToken::new(token)),
    );

    Ok(Arc::new(client))
}
