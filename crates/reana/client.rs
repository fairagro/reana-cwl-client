use crate::{
    api::{self, client::ReanaClient},
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

pub async fn ping(client: Arc<ReanaClient>) -> ClientResult<StatusCode> {
    Ok(api::ping(client).await?)
}

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
    let inputs = get_workflow_inputs(doc, job_inputs, base_path, working_directory)?;
    let outputs = get_workflow_outputs(&packed, workflow_id)?;

    let workflow = WorkflowJson::new("0.9.4".to_string(), specification, inputs, outputs);

    let res = api::workflows::create(client.clone(), &workflow, Some(name)).await?;
    info!("[{}] {}", res.workflow_name, res.message);

    Ok((res.workflow_name, workflow))
}

pub async fn upload_file(
    client: Arc<ReanaClient>,
    workflow_id: &str,
    file: &Path,
    working_directory: &Path,
) -> ClientResult<()> {
    let res =
        api::workflows::upload_file(client.clone(), workflow_id, file, working_directory).await?;
    info!("[{}] {}", workflow_id, res.message);

    Ok(())
}

pub async fn start(client: Arc<ReanaClient>, workflow_id: &str) -> ClientResult<()> {
    let res = api::workflows::start(client, workflow_id).await?;
    info!("[{}] {}", res.workflow_name, res.message);
    Ok(())
}
