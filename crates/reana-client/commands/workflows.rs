use crate::{cli::WorkflowArgs, client};
use miette::IntoDiagnostic;
use reana::client::create;

pub async fn create_workflow(args: WorkflowArgs) -> miette::Result<()> {
    let client = client()?;
    create(
        client,
        args.name.as_deref().unwrap_or("default"),
        &args.cwlfile,
        &args.jobfile,
    )
    .await
    .into_diagnostic()
}
