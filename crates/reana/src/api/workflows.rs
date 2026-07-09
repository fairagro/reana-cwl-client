use crate::{
    api::{
        JSON_CONTENT_TYPE, OCTET_CONTENT_TYPE,
        response::{
            WorkflowListResponse, WorkflowMessageResponse, WorkflowStatusResponse,
            WorkflowSubmitResponse, WorkflowWorkspaceResponse,
        },
    },
    client::ReanaClient,
    error::APIResult,
    models::workflows::WorkflowJson,
};
use reqwest::header::CONTENT_TYPE;
use std::{path::Path, sync::Arc};
use tokio::fs;

pub async fn list(reana: Arc<ReanaClient>) -> APIResult<WorkflowListResponse> {
    let request = reana
        .build_request(reqwest::Method::GET, "workflows")
        .await?;
    let response = request.send().await?.error_for_status()?;

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

    let response = request.send().await?.error_for_status()?;
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

    let response = request.send().await?.error_for_status()?;
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

    let response = request.send().await?.error_for_status()?;
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

    let response = request.send().await?.error_for_status()?;
    let json = response.json::<WorkflowWorkspaceResponse>().await?;

    Ok(json)
}

pub async fn upload_file(
    reana: Arc<ReanaClient>,
    workflow_id_or_name: &str,
    file: &Path,
    working_dir: &Path,
) -> APIResult<WorkflowWorkspaceResponse> {
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

    let response = request.send().await?.error_for_status()?;
    let json = response.json::<WorkflowWorkspaceResponse>().await?;

    Ok(json)
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

    let response = request.send().await?.error_for_status()?;
    let json = response.json::<WorkflowJson>().await?;

    Ok(json)
}
