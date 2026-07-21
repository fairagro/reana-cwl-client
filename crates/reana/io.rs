use crate::{
    error::{ClientError, ClientResult},
    models::workflows::{WorkflowInputs, WorkflowOutputs},
};
use commonwl::{
    Identifiable, OneOrMany,
    documents::{CWLDocument, StringOrDocument},
    engine::{InputObject, collect_inputs, flatten_inputs},
    files::{Directory, File, FileOrDirectory},
    inputs::DefaultValue,
    outputs::{CommandOutputParameterType, CommandOutputSchema, CommandOutputType},
    packed::PackedCWL,
    storage::StoragePath,
    types::CWLType,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tracing::error;
use url::Url;

pub(crate) fn get_workflow_inputs(
    doc: CWLDocument,
    job_inputs: InputObject,
    base_path: &Path,
    working_directory: &Path,
) -> ClientResult<WorkflowInputs> {
    let mut cwl_inputs =
        collect_inputs(&doc, &job_inputs.inputs, base_path, base_path, None, None)?;

    relativize_inputs(&mut cwl_inputs, working_directory)?;

    let flattened_inputs = flatten_inputs(&cwl_inputs);

    let (files, directories): (Vec<_>, Vec<_>) =
        flattened_inputs.into_iter().partition(commonwl::files::FileOrDirectory::is_file);

    let files = files
        .iter()
        .map(location_as_path)
        .collect::<ClientResult<Vec<_>>>()?;
    let directories = directories
        .iter()
        .map(location_as_path)
        .collect::<ClientResult<Vec<_>>>()?;

    Ok(WorkflowInputs {
        directories,
        files: files.clone(),
        parameters: cwl_inputs,
    })
}

pub(crate) fn get_workflow_outputs(
    packed: &PackedCWL,
    workflow_id: &str,
) -> ClientResult<WorkflowOutputs> {
    let Some(CWLDocument::Workflow(workflow)) = packed
        .graph
        .iter()
        .find(|i| i.get_id().is_some_and(|id| id == workflow_id))
    else {
        return Err(ClientError::Guard("Could not find main entity workflow"));
    };

    let mut output_files = vec![];
    for out in &workflow.outputs {
        if output_produces_file(&out.r#type)
            && let Some(source) = &out.output_source
        {
            for item in source.as_many() {
                let Some((step_id, output_id)) = item.rsplit_once('/') else {
                    continue;
                };

                let Some(step) = workflow
                    .steps
                    .iter()
                    .find(|i| i.get_id().is_some_and(|id| id == step_id))
                else {
                    error!("Could not retrieve step '{step_id}'");
                    continue;
                };

                let StringOrDocument::String(tool_id) = &step.run else {
                    error!("Could not retrieve step");
                    continue;
                };

                let Some(tool_doc) = packed
                    .graph
                    .iter()
                    .find(|d| d.get_id().is_some_and(|id| id == tool_id))
                else {
                    error!("Could not find tool '{tool_id}'");
                    continue;
                };

                let CWLDocument::CommandLineTool(tool) = tool_doc else {
                    continue;
                };

                let output_id = format!("{tool_id}/{output_id}");
                let Some(param) = tool
                    .outputs
                    .iter()
                    .find(|p| p.id.as_deref() == Some(&output_id))
                else {
                    continue;
                };

                if let Some(binding) = &param.output_binding
                    && let Some(glob) = &binding.glob
                {
                    output_files.extend(glob.as_many());
                }
            }
        }
    }
    Ok(WorkflowOutputs {
        files: output_files,
    })
}

fn location_as_path(fod: &FileOrDirectory) -> ClientResult<PathBuf> {
    fod.location()
        .map(PathBuf::from)
        .ok_or_else(|| ClientError::CWL(commonwl::Error::Guard("Missing location")))
}

#[allow(clippy::implicit_hasher)]
fn relativize_inputs(inputs: &mut HashMap<String, DefaultValue>, cwd: &Path) -> ClientResult<()> {
    for dv in inputs.values_mut() {
        relativize_default_value(dv, cwd)?;
    }
    Ok(())
}

fn relative_location(location: &str, cwd: &Path) -> ClientResult<PathBuf> {
    let url = Url::parse(location)?;
    let local_path = StoragePath::from_url(url).as_local_path()?;

    pathdiff::diff_paths(local_path, cwd)
        .ok_or_else(|| ClientError::CWL(commonwl::Error::Guard("Failed to compute relative path")))
}

fn relativize_fod(fod: &mut FileOrDirectory, cwd: &Path) -> ClientResult<()> {
    if let Some(location) = fod.location() {
        let rel = relative_location(location, cwd)?;
        match fod {
            FileOrDirectory::File(file) => {
                *file = File::builder()
                    .location(rel.to_string_lossy().into_owned())
                    .build();
            }
            FileOrDirectory::Directory(directory) => {
                *directory = Directory::builder()
                    .location(rel.to_string_lossy().into_owned())
                    .build();
            }
        }
    }

    if let FileOrDirectory::File(f) = fod
        && let Some(secondary_files) = f.secondary_files.as_mut()
    {
        for sf in secondary_files.iter_mut() {
            relativize_fod(sf, cwd)?;
        }
    }

    Ok(())
}

fn relativize_default_value(dv: &mut DefaultValue, cwd: &Path) -> ClientResult<()> {
    match dv {
        DefaultValue::FileOrDirectory(fod) => relativize_fod(fod, cwd),
        DefaultValue::Any(v) => relativize_json_value(v, cwd),
    }
}

fn relativize_json_value(value: &mut serde_json::Value, cwd: &Path) -> ClientResult<()> {
    match value {
        serde_json::Value::Array(values) => {
            for v in values.iter_mut() {
                relativize_json_element(v, cwd)?;
            }
            Ok(())
        }
        serde_json::Value::Object(mapping) => {
            for v in mapping.values_mut() {
                relativize_json_element(v, cwd)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn relativize_json_element(v: &mut serde_json::Value, cwd: &Path) -> ClientResult<()> {
    if let Ok(mut dv) = serde_json::from_value::<DefaultValue>(v.clone()) {
        relativize_default_value(&mut dv, cwd)?;
        *v = serde_json::to_value(dv).map_err(|_| {
            ClientError::CWL(commonwl::Error::Guard("Failed to re-serialize input"))
        })?;
    } else {
        relativize_json_value(v, cwd)?;
    }
    Ok(())
}

fn is_file_like(t: &CommandOutputType) -> bool {
    match t {
        CommandOutputType::CWLType(CWLType::File | CWLType::Directory) => true,
        CommandOutputType::CommandOutputSchema(schema) => match schema.as_ref() {
            CommandOutputSchema::Array(arr) => match &arr.items {
                OneOrMany::One(item) => is_file_like(item),
                OneOrMany::Many(items) => items.iter().any(is_file_like),
            },
            _ => false,
        },
        CommandOutputType::String(_) => false,
        _ => false,
    }
}

pub(crate) fn output_produces_file(t: &CommandOutputParameterType) -> bool {
    match t {
        CommandOutputParameterType::Stdout | CommandOutputParameterType::Stderr => true,
        CommandOutputParameterType::CommandOutputType(one_or_many) => match one_or_many {
            OneOrMany::One(t) => is_file_like(t),
            OneOrMany::Many(ts) => ts.iter().any(is_file_like),
        },
    }
}
