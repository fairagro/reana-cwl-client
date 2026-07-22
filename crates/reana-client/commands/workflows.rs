use std::env;

use crate::{
    cli::{DownloadArgs, UploadArgs, WorkflowArgs, WorkflowIdArgs},
    client,
    commands::{JobLog, WorkflowLogs},
};
use miette::IntoDiagnostic;
use owo_colors::OwoColorize;
use reana::client;
use tracing::info;

/// Creates and runs
/// # Errors
/// Returns Error if the request fails
///
/// # Panics
/// if workfing dir has no parent
pub async fn create_and_run_workflow(args: WorkflowArgs) -> miette::Result<()> {
    let client = client()?;

    //create workspace
    let (workflow_id, spec) = client::create(
        client.clone(),
        args.name.as_deref().unwrap_or("default"),
        &args.cwlfile,
        &args.jobfile,
    )
    .await
    .into_diagnostic()?;

    let working_directory = args.jobfile.canonicalize().into_diagnostic()?;
    let working_directory = working_directory.parent().unwrap();
    //upload files
    for item in spec.inputs.files {
        let location = working_directory.join(&item);
        client::upload_file(
            client.clone(),
            &workflow_id,
            &location,
            &item.to_string_lossy(),
        )
        .await?;
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

    //create workspace
    client::create(
        client.clone(),
        args.name.as_deref().unwrap_or("default"),
        &args.cwlfile,
        &args.jobfile,
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

    client::upload_file(
        client,
        &args.workflow_name_or_id,
        &args.location,
        &args.filename,
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

/// Requests the logs of a workflow
/// # Errors
/// Returns Error if the request fails
pub async fn logs(args: WorkflowIdArgs) -> miette::Result<()> {
    let client = client()?;

    let res = client::logs(client, &args.workflow_name_or_id).await?;

    let parsed: WorkflowLogs = serde_json::from_str(&res.logs).into_diagnostic()?;

    if parsed.job_logs.is_empty() {
        println!("No job logs available yet.");
    }

    for (job_id, job) in &parsed.job_logs {
        print_job_header(job_id, job);
        println!("{}", job.logs.trim_end());
        println!();
    }

    if let Some(wf_logs) = &parsed.workflow_logs
        && !wf_logs.trim().is_empty()
    {
        println!("{}", "── engine logs ──".dimmed());
        println!("{}", wf_logs.trim_end());
    }

    Ok(())
}

fn print_job_header(job_id: &str, job: &JobLog) {
    let status_colored = match job.status.as_str() {
        "finished" => job.status.green().to_string(),
        "failed" => job.status.red().to_string(),
        "running" => job.status.yellow().to_string(),
        _ => job.status.clone(),
    };

    println!(
        "{} {} [{}]",
        "▶".bold(),
        job.job_name.bold(),
        status_colored,
    );
    println!(
        "  {} {}   {} {}",
        "image:".dimmed(),
        job.docker_img,
        "job id:".dimmed(),
        job_id,
    );
    if let (Some(start), Some(end)) = (&job.started_at, &job.finished_at) {
        println!("  {} {} → {}", "time:".dimmed(), start, end);
    }
    println!();
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
        info!("{}:\t{:?}", item.name, item.status.unwrap_or_default());
    }
    Ok(())
}

/// Requests the workspace of a workflow
/// # Errors
/// Returns Error if the request fails
pub async fn specification(args: WorkflowIdArgs) -> miette::Result<()> {
    let client = client()?;

    let res = client::specification(client, &args.workflow_name_or_id).await?;
    let json = serde_json::to_string_pretty(&res).into_diagnostic()?;
    println!("{json}");

    Ok(())
}
