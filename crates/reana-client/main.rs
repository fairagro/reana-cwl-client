use clap::Parser;
use miette::IntoDiagnostic;
use reana_client::cli::{Cli, handle_command_args};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> miette::Result<()> {
    dotenvy::dotenv().into_diagnostic()?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,reana=debug,reqwest=info", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Cli::parse();
    handle_command_args(args).await
}
