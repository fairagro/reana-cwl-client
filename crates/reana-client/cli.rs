use crate::commands;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run(WorkflowArgs),
    Create(WorkflowArgs),
    Start(WorkflowIdArgs),
    Status(WorkflowIdArgs),
    Logs(WorkflowIdArgs),
    Upload(UploadArgs),
    Download(DownloadArgs),
    Workspace(WorkflowIdArgs),
    Specification(WorkflowIdArgs),
    List,
    Ping,
}

/// Handles the subcommands
/// # Errors
/// Returns error if a command errors
pub async fn handle_command_args(args: Cli) -> miette::Result<()> {
    match args.command {
        Commands::Run(args) => commands::workflows::create_and_run_workflow(args).await,
        Commands::Create(args) => commands::workflows::create(args).await,
        Commands::Start(args) => commands::workflows::start(args).await,
        Commands::Status(args) => commands::workflows::status(args).await,
        Commands::Logs(args) => commands::workflows::logs(args).await,
        Commands::Upload(args) => commands::workflows::upload(args).await,
        Commands::Download(args) => commands::workflows::download(args).await,
        Commands::Workspace(args) => commands::workflows::workspace(args).await,
        Commands::Specification(args) => commands::workflows::specification(args).await,
        Commands::List => commands::workflows::list().await,
        Commands::Ping => commands::ping().await,
    }
}

#[derive(Args, Debug)]
pub struct WorkflowArgs {
    #[arg(short = 'n', long = "name", help = "Name of the workflow")]
    pub name: Option<String>,
    pub cwlfile: PathBuf,
    pub jobfile: PathBuf,
}

#[derive(Args, Debug)]
pub struct WorkflowIdArgs {
    pub workflow_name_or_id: String,
}

#[derive(Args, Debug)]
pub struct DownloadArgs {
    pub workflow_name_or_id: String,
    #[arg(short = 'f', long = "filename", help = "Name of file to download")]
    pub filename: String,
}

#[derive(Args, Debug)]
pub struct UploadArgs {
    pub workflow_name_or_id: String,
    #[arg(short = 'f', long = "filename", help = "Name of file to download")]
    pub filename: PathBuf,
}
