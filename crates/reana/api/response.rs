use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowListResponse {
    pub has_next: bool,
    pub has_prev: bool,
    pub items: Vec<WorkflowResponse>,
    pub page: i32,
    pub total: i32,
    pub user_has_workflows: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowResponse {
    pub id: String,
    pub name: String,
    pub created: chrono::NaiveDateTime,
    pub launcher_url: Option<String>,
    pub progress: Option<WorkflowProgressDates>,
    pub status: Option<WorkflowStatus>,
    pub size: Option<ItemSize>,
    pub user: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowProgressDates {
    pub run_finished_at: Option<chrono::NaiveDateTime>,
    pub run_started_at: Option<chrono::NaiveDateTime>,
    pub run_stopped_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowStatus {
    Created,
    Running,
    Finished,
    Failed,
    Stopped,
    #[default]
    Queued,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ItemSize {
    pub human_readable: String,
    pub raw: i64, //supports -1
}

#[derive(Deserialize, Debug, Default)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowMessageResponse {
    pub workflow_id: String,
    pub workflow_name: String,
    pub message: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowSubmitResponse {
    pub workflow_id: String,
    pub workflow_name: String,
    pub message: String,
    pub run_number: String,
    pub user: String,
    pub status: WorkflowStatus,
}

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowStatusResponse {
    pub id: String,
    pub created: chrono::NaiveDateTime,
    pub logs: String,
    pub name: String,
    pub status: WorkflowStatus,
    pub user: String,
    pub progress: WorkflowProgress,
}

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowLogsResponse {
    pub logs: String,
    pub workflow_id: String,
    pub workflow_name: String,
    pub user: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowProgress {
    pub current_command: Option<String>,
    pub current_step_name: Option<String>,
    pub finished: WorkflowEnumeration,
    pub failed: WorkflowEnumeration,
    pub running: WorkflowEnumeration,
    pub total: WorkflowEnumeration,
    #[serde(flatten)]
    pub dates: WorkflowProgressDates,
}

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowEnumeration {
    pub job_ids: Vec<String>,
    pub total: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowWorkspaceResponse {
    pub has_next: bool,
    pub has_prev: bool,
    pub items: Vec<WorkflowWorkspaceItem>,
    pub page: u64,
    pub total: u64,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct WorkflowWorkspaceItem {
    pub name: String,
    pub size: ItemSize,
    #[serde(rename = "last-modified")]
    pub last_modified: chrono::NaiveDateTime,
}
