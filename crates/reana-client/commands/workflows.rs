use std::env;

use crate::{
    cli::{DownloadArgs, UploadArgs, WorkflowArgs, WorkflowIdArgs},
    client,
};
use miette::IntoDiagnostic;
use reana::client;
use tracing::info;

/// Creates and runs
/// # Errors
/// Returns Error if the request fails
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

/// Creates a Workfklow
/// # Errors
/// Returns Error if the request fails
pub async fn create(args: WorkflowArgs) -> miette::Result<()> {
    let client = client()?;
    let working_directory = env::current_dir().into_diagnostic()?;

    //create workspace
    client::create(
        client.clone(),
        args.name.as_deref().unwrap_or("default"),
        &args.cwlfile,
        &args.jobfile,
        &working_directory,
    )
    .await
    .into_diagnostic()?;

    Ok(())
}

/// Starts a Workfklow
/// # Errors
/// Returns Error if the request fails
pub async fn start(args: WorkflowIdArgs) -> miette::Result<()> {
    let client = client()?;

    client::start(client, &args.workflow_name_or_id).await?;

    Ok(())
}

/// Downlaods a file from a workspace
/// # Errors
/// Returns Error if the request fails
pub async fn download(args: DownloadArgs) -> miette::Result<()> {
    let client = client()?;
    let working_directory = env::current_dir().into_diagnostic()?;

    client::download_file(
        client,
        &args.workflow_name_or_id,
        &args.filename,
        &working_directory,
    )
    .await?;

    Ok(())
}

/// Uploads a file to a workspace
/// # Errors
/// Returns Error if the request fails
pub async fn upload(args: UploadArgs) -> miette::Result<()> {
    let client = client()?;
    let working_directory = env::current_dir().into_diagnostic()?;

    client::upload_file(
        client,
        &args.workflow_name_or_id,
        &args.filename,
        &working_directory,
    )
    .await?;

    Ok(())
}

/// Requests the status of a workflow
/// # Errors
/// Returns Error if the request fails
pub async fn status(args: WorkflowIdArgs) -> miette::Result<()> {
    let client = client()?;

    client::status(client, &args.workflow_name_or_id).await?;

    Ok(())
}

/// Requests the workspace of a workflow
/// # Errors
/// Returns Error if the request fails
pub async fn workspace(args: WorkflowIdArgs) -> miette::Result<()> {
    let client = client()?;

    let res = client::workspace(client, &args.workflow_name_or_id).await?;
    let list = res.items;
    let json = serde_json::to_string_pretty(&list).into_diagnostic()?;
    println!("{json}");

    Ok(())
}

/// Requests the workspace of a workflow
/// # Errors
/// Returns Error if the request fails
pub async fn list() -> miette::Result<()> {
    let client = client()?;

    let res = client::list(client).await?;
    for item in res.items {
        info!("{}:\t{:?}", item.name, item.status.unwrap())
    }
    Ok(())
}
