use crate::{
    api::{
        JSON_CONTENT_TYPE, OCTET_CONTENT_TYPE,
        client::ReanaClient,
        report,
        response::{
            MessageResponse, WorkflowListResponse, WorkflowMessageResponse, WorkflowStatusResponse,
            WorkflowSubmitResponse, WorkflowWorkspaceResponse,
        },
    },
    error::APIResult,
    models::workflows::WorkflowJson,
};
use reqwest::header::CONTENT_TYPE;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use tracing::debug;

pub async fn list(reana: Arc<ReanaClient>) -> APIResult<WorkflowListResponse> {
    let request = reana
        .build_request(reqwest::Method::GET, "workflows")
        .await?;

    debug!("Request: {request:?}");
    let response = request.send().await?;
    if let Err(err) = response.error_for_status_ref() {
        report(response).await;
        return Err(err.into());
    }

    let workflows = response.json::<WorkflowListResponse>().await?;

    Ok(workflows)
}

pub async fn create(
    reana: Arc<ReanaClient>,
    workflow: &WorkflowJson,
    name: Option<&str>,
) -> APIResult<WorkflowMessageResponse> {
    let value = serde_json::to_value(workflow)?;

    let request = reana
        .build_request(reqwest::Method::POST, "workflows")
        .await?
        .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        .query(&name.map(|n| [("workflow_name", n)]).unwrap_or_default())
        .json(&value);

    debug!("Request: {request:?}");
    let response = request.send().await?;
    if let Err(err) = response.error_for_status_ref() {
        report(response).await;
        return Err(err.into());
    }

    let json = response.json::<WorkflowMessageResponse>().await?;

    Ok(json)
}

pub async fn start(
    reana: Arc<ReanaClient>,
    workflow_id_or_name: &str,
) -> APIResult<WorkflowSubmitResponse> {
    let request = reana
        .build_request(
            reqwest::Method::POST,
            &format!("workflows/{workflow_id_or_name}/start"),
        )
        .await?;

    debug!("Request: {request:?}");
    let response = request.send().await?;
    if let Err(err) = response.error_for_status_ref() {
        report(response).await;
        return Err(err.into());
    }

    let json = response.json::<WorkflowSubmitResponse>().await?;

    Ok(json)
}

pub async fn status(
    reana: Arc<ReanaClient>,
    workflow_id_or_name: &str,
) -> APIResult<WorkflowStatusResponse> {
    let request = reana
        .build_request(
            reqwest::Method::GET,
            &format!("workflows/{workflow_id_or_name}/status"),
        )
        .await?;

    debug!("Request: {request:?}");
    let response = request.send().await?;
    if let Err(err) = response.error_for_status_ref() {
        report(response).await;
        return Err(err.into());
    }
    let json = response.json::<WorkflowStatusResponse>().await?;

    Ok(json)
}

pub async fn workspace(
    reana: Arc<ReanaClient>,
    workflow_id_or_name: &str,
) -> APIResult<WorkflowWorkspaceResponse> {
    let request = reana
        .build_request(
            reqwest::Method::GET,
            &format!("workflows/{workflow_id_or_name}/workspace"),
        )
        .await?;

    debug!("Request: {request:?}");
    let response = request.send().await?;
    if let Err(err) = response.error_for_status_ref() {
        report(response).await;
        return Err(err.into());
    }
    let json = response.json::<WorkflowWorkspaceResponse>().await?;

    Ok(json)
}

pub async fn upload_file(
    reana: Arc<ReanaClient>,
    workflow_id_or_name: &str,
    file: &Path,
    working_dir: &Path,
) -> APIResult<MessageResponse> {
    let file_name = if file.is_absolute() {
        pathdiff::diff_paths(file, working_dir).unwrap_or(file.to_path_buf())
    } else {
        file.to_path_buf()
    }
    .to_string_lossy()
    .into_owned();

    let content = fs::read(file).await?;

    let request = reana
        .build_request(
            reqwest::Method::POST,
            &format!("workflows/{workflow_id_or_name}/workspace"),
        )
        .await?
        .header(CONTENT_TYPE, OCTET_CONTENT_TYPE)
        .query(&[("file_name", &file_name)])
        .body(content);

    debug!("Request: {request:?}");
    let response = request.send().await?;
    if let Err(err) = response.error_for_status_ref() {
        report(response).await;
        return Err(err.into());
    }
    let json = response.json::<MessageResponse>().await?;

    Ok(json)
}

pub async fn download_file(
    reana: Arc<ReanaClient>,
    workflow_id_or_name: &str,
    file_name: &str,
    output_folder: &Path,
) -> APIResult<PathBuf> {
    let request = reana
        .build_request(
            reqwest::Method::GET,
            &format!("workflows/{workflow_id_or_name}/workspace/{file_name}"),
        )
        .await?;

    debug!("Request: {request:?}");
    let response = request.send().await?;
    if let Err(err) = response.error_for_status_ref() {
        report(response).await;
        return Err(err.into());
    }
    let content = response.bytes().await?;

    let output_path = output_folder.join(file_name);
    fs::create_dir_all(&output_path.parent().unwrap()).await?;
    let mut file = File::create(&output_path).await?;
    file.write_all(&content).await?;

    Ok(output_path)
}

pub async fn specification(
    reana: Arc<ReanaClient>,
    workflow_id_or_name: &str,
) -> APIResult<WorkflowJson> {
    let request = reana
        .build_request(
            reqwest::Method::GET,
            &format!("workflows/{workflow_id_or_name}/specification"),
        )
        .await?;

    debug!("Request: {request:?}");
    let response = request.send().await?;
    if let Err(err) = response.error_for_status_ref() {
        report(response).await;
        return Err(err.into());
    }
    let json = response.json::<WorkflowJson>().await?;

    Ok(json)
}
