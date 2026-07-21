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
    Start(WorkflowArgs),
    Status(WorkflowIdArgs),
    Ping,
}

pub async fn handle_command_args(args: Cli) -> miette::Result<()> {
    match args.command {
        Commands::Start(args) => commands::workflows::create_and_run_workflow(args).await,
        Commands::Status(args) => commands::workflows::status(args).await,
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
