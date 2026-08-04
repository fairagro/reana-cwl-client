use crate::{
    api::response::{WorkflowLogsResponse, WorkflowStatus},
    error::ClientResult,
};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ReanaLogMessage {
    pub workflow_logs: String,
    pub job_logs: HashMap<String, JobLog>,
    pub engine_specific: Option<Value>,
}

#[derive(Deserialize)]
pub struct JobLog {
    pub workflow_uuid: String,
    pub job_name: String,
    pub compute_backend: String,
    pub backend_job_id: String,
    pub docker_img: String,
    pub cmd: String,
    pub status: WorkflowStatus,
    pub logs: String,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub finished_at: Option<chrono::NaiveDateTime>,
}

///Parses Workflow Logs to get the output object
/// # Errors
/// if parsing fails
pub fn get_log_outputs(message: &ReanaLogMessage) -> ClientResult<Option<Value>> {
    let logline = extract_json(&message.workflow_logs);

    let outputs = match logline {
        Some(s) => serde_json::from_str(s)?,
        None => return Ok(None),
    };

    Ok(Some(outputs))
}

///Parses Workflow Logs to get the output object
/// # Errors
/// if JSON parsing fails
pub fn get_log_message(logs: &WorkflowLogsResponse) -> ClientResult<ReanaLogMessage> {
    let logs = &logs.logs;
    Ok(serde_json::from_str::<ReanaLogMessage>(logs)?)
}

fn extract_json(s: &str) -> Option<&str> {
    const START: &str = "FinalOutput";
    const END: &str = "}FinalOutput";

    let start = s.find(START)? + START.len();
    let end = s[start..].find(END)? + start + 1; // include the closing '}'
    Some(&s[start..end])
}

#[must_use]
pub fn engine_version(message: &ReanaLogMessage) -> Option<(String, String)> {
    const START: &str = "run-cwl-workflow ";
    const END: &str = "\n";

    let message = &message.workflow_logs;
    let start = message.find(START)? + START.len();
    let end = message[start..].find(END)? + start;

    Some(("reana".to_string(), message[start..end].to_owned()))
}

#[cfg(test)]
mod tests {
    use crate::{
        api::response::WorkflowLogsResponse,
        logs::{engine_version, get_log_message, get_log_outputs},
    };
    use std::{fs, path::Path};

    #[test]
    fn test_analyze_logs() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let log = root.join("../../testdata/example_logs.json");
        let data = fs::read_to_string(log).unwrap();

        let res = WorkflowLogsResponse {
            logs: data,
            workflow_id: "test".to_string(),
            workflow_name: "test".to_string(),
            user: "test".to_string(),
        };

        let message = get_log_message(&res).unwrap();
        let outputs = get_log_outputs(&message).unwrap();
        assert!(outputs.is_some());
    }

    #[test]
    fn test_engine_version() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let log = root.join("../../testdata/example_logs.json");
        let data = fs::read_to_string(log).unwrap();

        let res = WorkflowLogsResponse {
            logs: data,
            workflow_id: "test".to_string(),
            workflow_name: "test".to_string(),
            user: "test".to_string(),
        };
        let message = get_log_message(&res).unwrap();
        let engine_version = engine_version(&message);
        assert!(engine_version.is_some());

        assert_eq!(
            engine_version,
            Some((
                "reana".to_string(),
                "0.9.4 with cwltool 3.1.20210628163208".to_string()
            ))
        )
    }
}
