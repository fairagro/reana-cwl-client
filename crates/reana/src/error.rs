use std::io;

use miette::Diagnostic;
use reana_auth::AuthError;
use thiserror::Error;

pub type APIResult<T> = Result<T, APIError>;

#[derive(Error, Diagnostic, Debug)]
pub enum APIError {
    #[diagnostic(code = "reana_auth::AuthError")]
    #[error("Authentication Failed")]
    Auth(#[from] AuthError),

    #[diagnostic(code = "io::Error")]
    #[error("I/O Error")]
    IO(#[from]io::Error),

    #[diagnostic(code = "serde_json::Error")]
    #[error("Failed to parse JSON")]
    JSON(#[from] serde_json::Error),

    #[diagnostic(code = "reqwest::Error")]
    #[error("Failed to send request")]
    SendError(#[from] reqwest::Error),

    #[diagnostic(code = "url::ParseError")]
    #[error("Could not parse URL")]
    URL(#[from] url::ParseError),
}
