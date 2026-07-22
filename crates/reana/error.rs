use miette::Diagnostic;
use reana_auth::AuthError;
use std::io;
use thiserror::Error;

pub type APIResult<T> = Result<T, APIError>;
pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Error, Diagnostic, Debug)]
pub enum APIError {
    #[diagnostic(code = "reana_auth::AuthError")]
    #[error("Authentication Failed")]
    Auth(#[from] AuthError),

    #[diagnostic(code = "io::Error")]
    #[error("I/O Error")]
    IO(#[from] io::Error),

    #[diagnostic(code = "serde_json::Error")]
    #[error("Failed to parse JSON")]
    JSON(#[from] serde_json::Error),

    #[diagnostic(code = "reqwest::Error")]
    #[error("Failed to send request")]
    SendError(#[from] reqwest::Error),

    #[diagnostic(code = "url::ParseError")]
    #[error("Could not parse URL")]
    URL(#[from] url::ParseError),

    #[diagnostic(code = "commonwl::Error")]
    #[error("Could not handle CWL")]
    CWL(#[from] commonwl::Error),

    #[diagnostic(code = "anyhow::Error")]
    #[error("Unknown Error")]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Diagnostic, Debug)]
pub enum ClientError {
    #[diagnostic(code = "reana::APIError")]
    #[error("Failed to use REANA API")]
    API(#[from] APIError),

    #[diagnostic(code = "url::ParseError")]
    #[error("Could not parse URL")]
    URL(#[from] url::ParseError),

    #[diagnostic(code = "commonwl::Error")]
    #[error("Could not handle CWL")]
    CWL(#[from] commonwl::Error),

    #[error("Guard clause failed")]
    #[diagnostic(code(reana::ClientError::Guard))]
    Guard(&'static str),

    #[diagnostic(code = "io::Error")]
    #[error("I/O Error")]
    IO(#[from] io::Error),

    #[diagnostic(code = "anyhow::Error")]
    #[error("Unknown Error")]
    Unknown(#[from] anyhow::Error),
}
