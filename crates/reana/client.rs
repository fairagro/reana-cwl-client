use crate::{
    api::{self, client::ReanaClient},
    error::{APIError, APIResult},
    location_as_path,
    models::workflows::{WorkflowInputs, WorkflowJson, WorkflowOutputs, WorkflowSpecification},
    relativize_inputs,
};
use commonwl::{
    documents::CWLDocument,
    engine::{collect_inputs, flatten_inputs, load_input_file_from_file},
    load_cwl_file,
    packed::{PackedCWL, pack_cwl},
};
use reqwest::StatusCode;
use std::{env, path::Path, sync::Arc};
use tracing::info;

pub async fn ping(client: Arc<ReanaClient>) -> APIResult<StatusCode> {
    api::ping(client).await
}

pub async fn create(
    client: Arc<ReanaClient>,
    name: &str,
    cwl_file: &Path,
    job_file: &Path,
) -> APIResult<()> {
    let doc = load_cwl_file(cwl_file, true)?;
    let graph = pack_cwl(&doc, cwl_file, None)?;
    let packed = PackedCWL {
        graph,
        cwl_version: doc.cwl_version().cloned(),
    };
    let specification = WorkflowSpecification {
        file: cwl_file.to_string_lossy().to_string(),
        specification: packed,
        r#type: "cwl".to_string(),
    };

    let base_path = cwl_file.parent().unwrap();
    let cwd = env::current_dir()?;
    let job_inputs = load_input_file_from_file(job_file, base_path)?;
    let mut cwl_inputs =
        collect_inputs(&doc, &job_inputs.inputs, base_path, base_path, None, None)?;
    relativize_inputs(&mut cwl_inputs, &cwd)?;

    let flattened_inputs = flatten_inputs(&cwl_inputs);

    let (files, directories): (Vec<_>, Vec<_>) =
        flattened_inputs.into_iter().partition(|fod| fod.is_file());

    let files = files
        .iter()
        .map(location_as_path)
        .collect::<APIResult<Vec<_>>>()?;
    let directories = directories
        .iter()
        .map(location_as_path)
        .collect::<APIResult<Vec<_>>>()?;

    let inputs = WorkflowInputs {
        directories,
        files: files.clone(),
        parameters: cwl_inputs,
    };

    let CWLDocument::Workflow(_workflow) = doc else {
        return Err(APIError::CWL(commonwl::Error::Guard("Not a Workflow")));
    };

    let outputs = WorkflowOutputs { files: vec![] };
    let workflow = WorkflowJson::new("0.9.4".to_string(), specification, inputs, outputs);

    let res = api::workflows::create(client.clone(), &workflow, Some(name)).await?;
    info!("[{}] {}", res.workflow_name, res.message);

    for item in files {
        api::workflows::upload_file(client.clone(), &res.workflow_id, &item, &cwd).await?;
    }

    let res = api::workflows::start(client, &res.workflow_id).await?;
    info!("[{}] {}", res.workflow_name, res.message);
    Ok(())
}
