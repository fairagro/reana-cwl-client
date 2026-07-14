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
    Ping,
}

pub async fn handle_command_args(args: Cli) -> miette::Result<()> {
    match args.command {
        Commands::Start(_workflow_args) => todo!(),
        Commands::Ping => commands::ping().await?,
    }
    Ok(())
}

#[derive(Args, Debug)]
pub struct WorkflowArgs {
    #[arg(short = 'n', long = "name", help = "Name of the workflow")]
    pub name: Option<String>,
    pub cwlfile: PathBuf,
    pub jobfile: PathBuf,
}
