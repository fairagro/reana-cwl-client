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

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowSpecification {
    pub file: String,
    pub specification: PackedCWL,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowInputs {
    directories: Vec<PathBuf>,
    files: Vec<PathBuf>,
    parameters: HashMap<String, DefaultValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputs {
    files: Vec<String>,
}
