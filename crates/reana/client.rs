use crate::{
    api::{
        self,
        client::ReanaClient,
        response::{
            WorkflowListResponse, WorkflowLogsResponse, WorkflowSpecificationResponse,
            WorkflowStatus, WorkflowWorkspaceResponse,
        },
    },
    error::ClientResult,
    io::{get_workflow_inputs, get_workflow_outputs},
    models::workflows::{WorkflowJson, WorkflowSpecification},
    wrap_tools,
};
use commonwl::{
    documents::CWLDocument,
    engine::{InputObject, load_input_file_from_file},
    load_cwl_file,
    packed::{PackedCWL, pack_cwl},
};
use reqwest::StatusCode;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tracing::{debug, info};

/// Sends a ping request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails
pub async fn ping(client: Arc<ReanaClient>) -> ClientResult<StatusCode> {
    Ok(api::ping(client).await?)
}

/// Sends a create request to the REANA Endpoint using filepaths
/// # Errors
/// Returns Error if the request fails, a given file does not exist or the CWL fails to pack
/// # Panics
/// Never
pub async fn create(
    client: Arc<ReanaClient>,
    name: &str,
    cwl_file: &Path,
    job_file: &Path,
) -> ClientResult<CreatedWorkspace> {
    let cwl_file = dunce::canonicalize(cwl_file)?;
    let job_file = dunce::canonicalize(job_file)?;
    debug!("Resolved executiom pair to {cwl_file:?} and {job_file:?}");

    let base_path = cwl_file.parent().unwrap();
    let job_inputs = load_input_file_from_file(&job_file, base_path)?;
    create2(client, name, &cwl_file, &job_inputs).await
}

pub struct CreatedWorkspace {
    pub workflow_id: String,
    pub specification: WorkflowJson,
    pub local_workspace: PathBuf,
}

impl CreatedWorkspace {
    pub fn new(workflow_id: &str, specification: WorkflowJson, local_workspace: &Path) -> Self {
        Self {
            workflow_id: workflow_id.to_string(),
            specification,
            local_workspace: local_workspace.to_path_buf(),
        }
    }
}

/// Sends a create request to the REANA Endpoint using already loaded inputs
/// # Errors
/// Returns Error if the request fails, a given file does not exist or the CWL fails to pack
/// # Panics
/// Never
pub async fn create2(
    client: Arc<ReanaClient>,
    name: &str,
    cwl_file: &Path,
    inputs: &InputObject,
) -> ClientResult<CreatedWorkspace> {
    let cwl_file = dunce::canonicalize(cwl_file)?;
    let cwl_file = cwl_file.as_path();

    let workflow_id = "#main";

    let doc = load_cwl_file(cwl_file, true)?;
    let doc = match doc {
        CWLDocument::CommandLineTool(_) | CWLDocument::ExpressionTool(_) => wrap_tools(doc),
        _ => doc,
    };

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

    let specification_dir = cwl_file.parent().unwrap_or(Path::new("."));
    let (inputs, local_workspace) = get_workflow_inputs(&doc, inputs, specification_dir)?;
    let outputs = get_workflow_outputs(&packed, workflow_id)?;

    let workflow = WorkflowJson::new("0.9.4".to_string(), specification, inputs, outputs);

    let res = api::workflows::create(client.clone(), &workflow, Some(name)).await?;
    info!("[{}] {}", res.workflow_name, res.message);

    Ok(CreatedWorkspace::new(
        &res.workflow_name,
        workflow,
        &local_workspace,
    ))
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
    location: &Path,
    desired_path: &str,
) -> ClientResult<()> {
    let res = api::workflows::upload_file(client, workflow_id, location, desired_path).await?;
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
pub async fn status(client: Arc<ReanaClient>, workflow_id: &str) -> ClientResult<WorkflowStatus> {
    let res = api::workflows::status(client.clone(), workflow_id).await?;
    info!("[{workflow_id}] {:?}", res.status);

    Ok(res.status)
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

/// Sends a specification request to the REANA Endpoint
/// # Errors
/// Returns Error if the request fails
pub async fn specification(
    client: Arc<ReanaClient>,
    workflow_id: &str,
) -> ClientResult<WorkflowSpecificationResponse> {
    let res = api::workflows::specification(client.clone(), workflow_id).await?;
    Ok(res)
}
