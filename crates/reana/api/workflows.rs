use crate::{
    api::{
        JSON_CONTENT_TYPE, OCTET_CONTENT_TYPE,
        client::ReanaClient,
        report,
        response::{
            MessageResponse, WorkflowListResponse, WorkflowLogsResponse, WorkflowMessageResponse,
            WorkflowSpecificationResponse, WorkflowStatusResponse, WorkflowSubmitResponse,
            WorkflowWorkspaceResponse,
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

/// Sends a list Request to the reana Enpoint
/// # Errors
/// Fails if building or sending the request fails
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

/// Sends a create Request to the reana Enpoint
/// # Errors
/// Fails if building or sending the request fails
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

/// Sends a start Request to the reana Enpoint
/// # Errors
/// Fails if building or sending the request fails
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

/// Sends a status Request to the reana Enpoint
/// # Errors
/// Fails if building or sending the request fails
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

/// Sends a logs Request to the reana Enpoint
/// # Errors
/// Fails if building or sending the request fails
pub async fn logs(
    reana: Arc<ReanaClient>,
    workflow_id_or_name: &str,
) -> APIResult<WorkflowLogsResponse> {
    let request = reana
        .build_request(
            reqwest::Method::GET,
            &format!("workflows/{workflow_id_or_name}/logs"),
        )
        .await?;

    debug!("Request: {request:?}");
    let response = request.send().await?;
    if let Err(err) = response.error_for_status_ref() {
        report(response).await;
        return Err(err.into());
    }
    let json = response.json::<WorkflowLogsResponse>().await?;

    Ok(json)
}

/// Sends a workspace Request to the reana Enpoint
/// # Errors
/// Fails if building or sending the request fails
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

/// Sends a upload Request to the reana Enpoint
/// # Errors
/// Fails if building or sending the request fails
pub async fn upload_file(
    reana: Arc<ReanaClient>,
    workflow_id_or_name: &str,
    location: &Path,
    desired_path: &str,
) -> APIResult<MessageResponse> {
    let content = fs::read(location).await?;

    let request = reana
        .build_request(
            reqwest::Method::POST,
            &format!("workflows/{workflow_id_or_name}/workspace"),
        )
        .await?
        .header(CONTENT_TYPE, OCTET_CONTENT_TYPE)
        .query(&[("file_name", desired_path)])
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

/// Sends a download Request to the reana Enpoint
/// # Errors
/// Fails if building or sending the request fails
/// # Panics
/// Parent path does not exist
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
    file.flush().await?;

    Ok(output_path)
}

/// Sends a specification Request to the reana Enpoint
/// # Errors
/// Fails if building or sending the request fails
pub async fn specification(
    reana: Arc<ReanaClient>,
    workflow_id_or_name: &str,
) -> APIResult<WorkflowSpecificationResponse> {
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
    let json = response.json::<WorkflowSpecificationResponse>().await?;

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::response::WorkflowStatus;
    use crate::models::workflows::{WorkflowInputs, WorkflowOutputs, WorkflowSpecification};
    use async_trait::async_trait;
    use commonwl::packed::PackedCWL;
    use mockito::{Matcher, Server};
    use reana_auth::TokenProvider;
    use serde_json::json;
    use std::{collections::HashMap, sync::Arc};
    use tempfile::tempdir;
    use url::Url;

    struct StaticTokenProvider(String);

    #[async_trait]
    impl TokenProvider for StaticTokenProvider {
        async fn get_token(&self) -> Result<String, reana_auth::AuthError> {
            Ok(self.0.clone())
        }
    }

    fn make_client(base_url: &str) -> Arc<ReanaClient> {
        let url = Url::parse(base_url).expect("valid test base URL");
        Arc::new(ReanaClient::new(
            url,
            Arc::new(StaticTokenProvider("test-token".to_string())),
        ))
    }

    fn test_workflow_json() -> WorkflowJson {
        WorkflowJson {
            inputs: WorkflowInputs {
                directories: vec![],
                files: vec![],
                parameters: HashMap::new(),
            },
            outputs: WorkflowOutputs { files: vec![] },
            version: "0.9.4".to_string(),
            workflow: WorkflowSpecification {
                file: "workflow.cwl".to_string(),
                specification: PackedCWL::default(),
                r#type: "cwl".to_string(),
            },
        }
    }

    #[tokio::test]
    async fn test_workflow_list() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = Server::new_async().await;
        let server_url = server.url();
        let client = make_client(&server_url);

        let server_mock = server
            .mock("GET", "/workflows")
            .match_query(Matcher::UrlEncoded(
                "access_token".into(),
                "test-token".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "has_next": false,
                    "has_prev": false,
                    "items": [],
                    "page": 1,
                    "total": 0,
                    "user_has_workflows": false
                })
                .to_string(),
            )
            .create_async()
            .await;

        let response = list(client.clone()).await?;

        server_mock.assert_async().await;
        assert!(!response.has_next);
        assert_eq!(response.items.len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_workflow() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = Server::new_async().await;
        let server_url = server.url();
        let client = make_client(&server_url);

        let server_mock = server
            .mock("POST", "/workflows")
            .match_query(Matcher::UrlEncoded(
                "access_token".into(),
                "test-token".into(),
            ))
            .match_query(Matcher::UrlEncoded(
                "workflow_name".into(),
                "test-workflow".into(),
            ))
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "workflow_id": "1",
                    "workflow_name": "test-workflow",
                    "message": "created"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let workflow = test_workflow_json();
        let response = create(client.clone(), &workflow, Some("test-workflow")).await?;

        server_mock.assert_async().await;
        assert_eq!(response.workflow_name, "test-workflow");
        assert_eq!(response.message, "created");
        Ok(())
    }

    #[tokio::test]
    async fn test_start_workflow() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = Server::new_async().await;
        let server_url = server.url();
        let client = make_client(&server_url);

        let server_mock = server
            .mock("POST", "/workflows/test-workflow/start")
            .match_query(Matcher::UrlEncoded(
                "access_token".into(),
                "test-token".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "workflow_id": "1",
                    "workflow_name": "test-workflow",
                    "message": "started",
                    "run_number": "42",
                    "user": "tester",
                    "status": "running"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let response = start(client.clone(), "test-workflow").await?;

        server_mock.assert_async().await;
        assert_eq!(response.workflow_name, "test-workflow");
        assert_eq!(response.status, WorkflowStatus::Running);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_status() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = Server::new_async().await;
        let server_url = server.url();
        let client = make_client(&server_url);

        let server_mock = server
            .mock("GET", "/workflows/test-workflow/status")
            .match_query(Matcher::UrlEncoded(
                "access_token".into(),
                "test-token".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "id": "1",
                    "created": "2024-01-01T00:00:00",
                    "name": "test-workflow",
                    "status": "running",
                    "logs": "gibberish",
                    "user": "tester",
                    "progress": {
                        "current_command": null,
                        "current_step_name": null,
                        "finished": { "job_ids": [], "total": 0 },
                        "failed": { "job_ids": [], "total": 0 },
                        "running": { "job_ids": [], "total": 0 },
                        "total": { "job_ids": [], "total": 0 }
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let response = status(client.clone(), "test-workflow").await?;

        server_mock.assert_async().await;
        assert_eq!(response.status, WorkflowStatus::Running);
        assert_eq!(response.name, "test-workflow");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_workspace() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = Server::new_async().await;
        let server_url = server.url();
        let client = make_client(&server_url);

        let server_mock = server
            .mock("GET", "/workflows/test-workflow/workspace")
            .match_query(Matcher::UrlEncoded(
                "access_token".into(),
                "test-token".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "has_next": false,
                    "has_prev": false,
                    "items": [
                        {
                            "name": "output.txt",
                            "size": { "human_readable": "1B", "raw": 1 },
                            "last-modified": "2024-01-01T00:00:00"
                        }
                    ],
                    "page": 1,
                    "total": 1
                })
                .to_string(),
            )
            .create_async()
            .await;

        let response = workspace(client.clone(), "test-workflow").await?;

        server_mock.assert_async().await;
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].name, "output.txt");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_specification() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = Server::new_async().await;
        let server_url = server.url();
        let client = make_client(&server_url);

        let response_body = json!({
            "specification": {
                "version": "0.9.4",
                "workflow": {
                    "file": "workflow.cwl",
                    "specification": { "$graph": [], "cwlVersion": null },
                    "type": "cwl"
                },
                "inputs": { "directories": [], "files": [], "parameters": {} },
                "outputs": { "files": [] }
            }
        });

        let server_mock = server
            .mock("GET", "/workflows/test-workflow/specification")
            .match_query(Matcher::UrlEncoded(
                "access_token".into(),
                "test-token".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response_body.to_string())
            .create_async()
            .await;

        let response = specification(client.clone(), "test-workflow").await?;
        assert_eq!(response.specification.workflow.file, "workflow.cwl");
        assert_eq!(response.specification.version, "0.9.4");
        server_mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_upload() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = Server::new_async().await;
        let server_url = server.url();
        let client = make_client(&server_url);

        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("hello.txt");
        tokio::fs::write(&file_path, b"hello world").await?;

        let server_mock = server
            .mock("POST", "/workflows/test-workflow/workspace")
            .match_query(Matcher::UrlEncoded(
                "access_token".into(),
                "test-token".into(),
            ))
            .match_query(Matcher::UrlEncoded("file_name".into(), "test-path".into()))
            .match_header("content-type", "application/octet-stream")
            .match_body("hello world")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message":"uploaded"}"#)
            .create_async()
            .await;

        let response =
            upload_file(client.clone(), "test-workflow", &file_path, "test-path").await?;

        server_mock.assert_async().await;
        assert_eq!(response.message, "uploaded");
        Ok(())
    }

    #[tokio::test]
    async fn test_download() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = Server::new_async().await;
        let server_url = server.url();
        let client = make_client(&server_url);

        let temp_dir = tempdir()?;
        let file_name = "output.txt";
        let expected_content = b"downloaded content";

        let server_mock = server
            .mock("GET", "/workflows/test-workflow/workspace/output.txt")
            .match_query(Matcher::UrlEncoded(
                "access_token".into(),
                "test-token".into(),
            ))
            .with_status(200)
            .with_body(expected_content.as_slice())
            .create_async()
            .await;

        let output_path =
            download_file(client.clone(), "test-workflow", file_name, temp_dir.path()).await?;

        server_mock.assert_async().await;
        assert_eq!(output_path, temp_dir.path().join(file_name));
        let actual = tokio::fs::read(&output_path).await?;
        assert_eq!(actual, expected_content);
        Ok(())
    }
}
