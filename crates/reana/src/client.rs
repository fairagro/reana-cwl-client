use reana_auth::TokenProvider;
use reqwest::{Client, Method, RequestBuilder};
use std::sync::Arc;
use url::Url;

use crate::error::APIResult;

pub struct ReanaClient {
    http_client: Client,
    base_url: Url,
    token_provider: Arc<dyn TokenProvider>,
}

impl ReanaClient {
    pub fn new(mut base_url: Url, token_provider: Arc<dyn TokenProvider>) -> Self {
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }

        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("failed to build reqwest client");
        Self {
            http_client,
            base_url,
            token_provider,
        }
    }

    pub(crate) async fn build_request(
        &self,
        method: Method,
        endpoint: &str,
    ) -> APIResult<RequestBuilder> {
        let token = self.token_provider.get_token().await?;

        let url = self.base_url.join(endpoint)?;
        Ok(self
            .http_client
            .request(method, url)
            .query(&[("access_token", &token)]))
    }
}
