use crate::{
    api::response::{WorkflowLogsResponse, WorkflowStatus},
    error::ClientResult,
};
use commonwl::inputs::DefaultValue;
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
    pub finished_at: chrono::NaiveDateTime,
}

///Parses Workflow Logs to get the output object
/// # Errors
/// if parsing fails
pub fn get_log_outputs(
    logs: &WorkflowLogsResponse,
) -> ClientResult<Option<HashMap<String, DefaultValue>>> {
    let logs = &logs.logs;
    let full_log = serde_json::from_str::<ReanaLogMessage>(logs)?;
    let workflow_logs = full_log.workflow_logs;

    let logline = extract_json(&workflow_logs);

    let outputs: HashMap<String, DefaultValue> = match logline {
        Some(s) => serde_json::from_str(s)?,
        None => return Ok(None),
    };

    Ok(Some(outputs))
}

fn extract_json(s: &str) -> Option<&str> {
    const START: &str = "FinalOutput";
    const END: &str = "}FinalOutput";

    let start = s.find(START)? + START.len();
    let end = s[start..].find(END)? + start + 1; // include the closing '}'
    Some(&s[start..end])
}

#[cfg(test)]
mod tests {
    use crate::{api::response::WorkflowLogsResponse, logs::get_log_outputs};
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

        let outputs = get_log_outputs(&res).unwrap();
        assert!(outputs.is_some());
        assert!(outputs.unwrap().contains_key("out"))
    }
}
