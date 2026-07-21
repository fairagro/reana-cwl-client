use std::env;

use crate::{cli::WorkflowArgs, client};
use miette::IntoDiagnostic;
use reana::client;

pub async fn create_and_run_workflow(args: WorkflowArgs) -> miette::Result<()> {
    let client = client()?;
    let working_directory = env::current_dir().into_diagnostic()?;

    //create workspace
    let (workflow_id, spec) = client::create(
        client.clone(),
        args.name.as_deref().unwrap_or("default"),
        &args.cwlfile,
        &args.jobfile,
        &working_directory,
    )
    .await
    .into_diagnostic()?;

    //upload files
    for item in spec.inputs.files {
        client::upload_file(client.clone(), &workflow_id, &item, &working_directory).await?;
    }

    //start
    client::start(client.clone(), &workflow_id).await?;

    Ok(())
}
