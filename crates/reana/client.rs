use crate::{
    api::{
        self,
        client::ReanaClient,
        response::{WorkflowListResponse, WorkflowLogsResponse, WorkflowWorkspaceResponse},
    },
    error::ClientResult,
    io::{get_workflow_inputs, get_workflow_outputs},
    models::workflows::{WorkflowJson, WorkflowSpecification},
};
use commonwl::{
    engine::load_input_file_from_file,
    load_cwl_file,
    packed::{PackedCWL, pack_cwl},
};
use reqwest::StatusCode;
use std::{path::Path, sync::Arc};
use tracing::info;

/// Sends a ping request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails
pub async fn ping(client: Arc<ReanaClient>) -> ClientResult<StatusCode> {
    Ok(api::ping(client).await?)
}

/// Sends a create request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails, a given file does not exist or the CWL fails to pack
/// # Panics
/// Never
pub async fn create(
    client: Arc<ReanaClient>,
    name: &str,
    cwl_file: &Path,
    job_file: &Path,
    working_directory: &Path,
) -> ClientResult<(String, WorkflowJson)> {
    let doc = load_cwl_file(cwl_file, true)?;
    let workflow_id = "#main";
    let graph = pack_cwl(&doc, cwl_file, Some(workflow_id))?;
    let packed = PackedCWL {
        graph,
        cwl_version: doc.cwl_version().cloned(),
    };
    let specification = WorkflowSpecification {
        file: cwl_file.to_string_lossy().to_string(),
        specification: packed.clone(),
        r#type: "cwl".to_string(),
    };

    let base_path = cwl_file.parent().unwrap();
    let job_inputs = load_input_file_from_file(job_file, base_path)?;
    let inputs = get_workflow_inputs(&doc, &job_inputs, base_path, working_directory)?;
    let outputs = get_workflow_outputs(&packed, workflow_id)?;

    let workflow = WorkflowJson::new("0.9.4".to_string(), specification, inputs, outputs);

    let res = api::workflows::create(client.clone(), &workflow, Some(name)).await?;
    info!("[{}] {}", res.workflow_name, res.message);

    Ok((res.workflow_name, workflow))
}

/// Sends a start request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails
pub async fn start(client: Arc<ReanaClient>, workflow_id: &str) -> ClientResult<()> {
    let res = api::workflows::start(client, workflow_id).await?;
    info!("[{}] {}", res.workflow_name, res.message);
    Ok(())
}

/// Sends a upload request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails or the file does not exist
pub async fn upload_file(
    client: Arc<ReanaClient>,
    workflow_id: &str,
    file: &Path,
    working_directory: &Path,
) -> ClientResult<()> {
    let res = api::workflows::upload_file(client, workflow_id, file, working_directory).await?;
    info!("[{workflow_id}] {}", res.message);

    Ok(())
}

/// Sends a download request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails or the file does not exist or writing locally fails
pub async fn download_file(
    client: Arc<ReanaClient>,
    workflow_id: &str,
    filename: &str,
    working_directory: &Path,
) -> ClientResult<()> {
    let res =
        api::workflows::download_file(client, workflow_id, filename, working_directory).await?;
    info!("[{workflow_id}] download of {filename} sucessfully. {res:?}");
    Ok(())
}

/// Sends a status request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails
pub async fn status(client: Arc<ReanaClient>, workflow_id: &str) -> ClientResult<()> {
    let res = api::workflows::status(client.clone(), workflow_id).await?;
    info!("[{workflow_id}] {:?}", res.status);

    Ok(())
}

/// Sends a logs request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails
pub async fn logs(
    client: Arc<ReanaClient>,
    workflow_id: &str,
) -> ClientResult<WorkflowLogsResponse> {
    let res = api::workflows::logs(client.clone(), workflow_id).await?;
    
    Ok(res)
}

/// Sends a workspace request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails
pub async fn workspace(
    client: Arc<ReanaClient>,
    workflow_id: &str,
) -> ClientResult<WorkflowWorkspaceResponse> {
    let res = api::workflows::workspace(client.clone(), workflow_id).await?;
    Ok(res)
}

/// Sends a list request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails
pub async fn list(client: Arc<ReanaClient>) -> ClientResult<WorkflowListResponse> {
    let res = api::workflows::list(client.clone()).await?;
    Ok(res)
}
