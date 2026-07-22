use commonwl::{
    OneOrMany,
    documents::{CWLDocument, StringOrDocument, Workflow, WorkflowStep},
    inputs::{WorkflowInputParameter, WorkflowStepInput},
    outputs::{StringOrWorkflowStepOutput, WorkflowOutputParameter},
};

pub mod api;
pub mod client;
pub mod error;
mod io;
pub mod models;

pub mod auth {
    pub use reana_auth::{AuthError, ReanaAccessToken, TokenProvider};
}

/// As REANA does not support Tools we wrap Tools into single step workflows
/// # Panics
/// IDs should be set, so should never panic on unwrap
#[must_use]
pub fn wrap_tools(doc: CWLDocument) -> CWLDocument {
    let inputs: Vec<WorkflowInputParameter> = doc
        .get_inputs()
        .into_iter()
        .map(|i| {
            WorkflowInputParameter::builder()
                .maybe_id(i.id)
                .maybe_doc(i.doc)
                .maybe_load_listing(i.load_listing)
                .maybe_load_contents(i.load_contents)
                .maybe_secondary_files(i.secondary_files)
                .maybe_format(i.format)
                .maybe_label(i.label)
                .maybe_default(i.default)
                .r#type(i.r#type)
                .build()
        })
        .collect::<Vec<_>>();
    let output_ids = doc.get_output_ids();

    let step_run = StringOrDocument::Document(Box::new(doc.clone()));
    let step = WorkflowStep::builder()
        .id("step1".to_owned())
        .run(step_run)
        .r#in(
            inputs
                .iter()
                .map(|i| {
                    WorkflowStepInput::builder()
                        .id(i.id.clone().unwrap())
                        .source(OneOrMany::One(i.id.clone().unwrap()))
                        .build()
                })
                .collect::<Vec<_>>(),
        )
        .out(
            output_ids
                .iter()
                .map(|o| StringOrWorkflowStepOutput::String(o.clone()))
                .collect(),
        )
        .build();

    let outputs = match doc {
        CWLDocument::CommandLineTool(clt) => clt
            .outputs
            .into_iter()
            .map(|o| {
                WorkflowOutputParameter::builder()
                    .maybe_id(o.id.clone())
                    .r#type(o.r#type)
                    .output_source(OneOrMany::One(format!("step1/{}", o.id.clone().unwrap())))
                    .maybe_format(o.format)
                    .maybe_secondary_files(o.secondary_files)
                    .maybe_doc(o.doc)
                    .maybe_label(o.label)
                    .build()
            })
            .collect::<Vec<_>>(),
        CWLDocument::ExpressionTool(et) => et
            .outputs
            .into_iter()
            .map(|o| {
                WorkflowOutputParameter::builder()
                    .maybe_id(o.id.clone())
                    .r#type(o.r#type)
                    .output_source(OneOrMany::One(format!("step1/{}", o.id.clone().unwrap())))
                    .maybe_format(o.format)
                    .maybe_secondary_files(o.secondary_files)
                    .maybe_doc(o.doc)
                    .maybe_label(o.label)
                    .build()
            })
            .collect::<Vec<_>>(),
        _ => unimplemented!(),
    };

    CWLDocument::Workflow(
        Workflow::builder()
            .cwl_version("v1.2")
            .inputs(inputs)
            .steps(vec![step])
            .outputs(outputs)
            .build(),
    )
}
