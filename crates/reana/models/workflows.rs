use commonwl::{inputs::DefaultValue, packed::PackedCWL};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowJson {
    pub inputs: WorkflowInputs,
    pub outputs: WorkflowOutputs,
    pub version: String,
    pub workflow: WorkflowSpecification,
}

impl WorkflowJson {
    #[must_use]
    pub fn new(
        reana_version: String,
        workflow: WorkflowSpecification,
        inputs: WorkflowInputs,
        outputs: WorkflowOutputs,
    ) -> Self {
        WorkflowJson {
            inputs,
            outputs,
            version: reana_version,
            workflow,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowSpecification {
    pub file: String,
    pub specification: PackedCWL,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowInputs {
    pub directories: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
    pub parameters: HashMap<String, DefaultValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputs {
    pub files: Vec<String>,
}
