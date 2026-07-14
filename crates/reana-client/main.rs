use clap::Parser;
use miette::IntoDiagnostic;
use reana_client::cli::{Cli, handle_command_args};

#[tokio::main]
async fn main() -> miette::Result<()> {
    dotenvy::dotenv().into_diagnostic()?;

    let args = Cli::parse();
    handle_command_args(args).await
}
