use async_trait::async_trait;
use miette::Diagnostic;
use secrecy::SecretString;
use std::{error::Error, fmt};

#[async_trait]
pub trait TokenProvider: Send + Sync {
    async fn get_token(&self) -> Result<SecretString, AuthError>;
}

#[derive(Debug, Clone, Diagnostic)]
#[diagnostic(
    code(reana_auth::AuthError),
    help("Check if your credentials are valid")
)]
pub struct AuthError;

impl Error for AuthError {}
impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Authentication error")
    }
}

pub struct ReanaAccessToken {
    token: SecretString,
}

impl ReanaAccessToken {
    #[must_use]
    pub fn new(token: String) -> Self {
        Self {
            token: SecretString::new(token.into()),
        }
    }
}

#[async_trait]
impl TokenProvider for ReanaAccessToken {
    async fn get_token(&self) -> Result<SecretString, AuthError> {
        Ok(self.token.clone())
    }
}
