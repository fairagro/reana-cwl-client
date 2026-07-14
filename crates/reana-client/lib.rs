use std::{env, sync::Arc};

use miette::IntoDiagnostic;
use reana::api::client::ReanaClient;
use reana_auth::ReanaAccessToken;
use url::Url;
pub mod cli;
pub mod commands;

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
