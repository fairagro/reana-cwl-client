use crate::error::{APIError, APIResult};
use commonwl::{
    files::{Directory, File, FileOrDirectory},
    inputs::DefaultValue,
    storage::StoragePath,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use url::Url;

pub mod api;
pub mod client;
pub mod error;
pub mod models;

fn location_as_path(fod: &FileOrDirectory) -> APIResult<PathBuf> {
    fod.location()
        .map(PathBuf::from)
        .ok_or_else(|| APIError::CWL(commonwl::Error::Guard("Missing location")))
}

fn relative_location(location: &str, cwd: &Path) -> APIResult<PathBuf> {
    let url = Url::parse(location)?;
    let local_path = StoragePath::from_url(url).as_local_path()?;

    pathdiff::diff_paths(local_path, cwd)
        .ok_or_else(|| APIError::CWL(commonwl::Error::Guard("Failed to compute relative path")))
}

fn relativize_fod(fod: &mut FileOrDirectory, cwd: &Path) -> APIResult<()> {
    if let Some(location) = fod.location() {
        let rel = relative_location(location, cwd)?;
        match fod {
            FileOrDirectory::File(file) => {
                *file = File::builder()
                    .location(rel.to_string_lossy().into_owned())
                    .build()
            }
            FileOrDirectory::Directory(directory) => {
                *directory = Directory::builder()
                    .location(rel.to_string_lossy().into_owned())
                    .build()
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

fn relativize_default_value(dv: &mut DefaultValue, cwd: &Path) -> APIResult<()> {
    match dv {
        DefaultValue::FileOrDirectory(fod) => relativize_fod(fod, cwd),
        DefaultValue::Any(v) => relativize_json_value(v, cwd),
    }
}

fn relativize_json_value(value: &mut serde_json::Value, cwd: &Path) -> APIResult<()> {
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

fn relativize_json_element(v: &mut serde_json::Value, cwd: &Path) -> APIResult<()> {
    if let Ok(mut dv) = serde_json::from_value::<DefaultValue>(v.clone()) {
        relativize_default_value(&mut dv, cwd)?;
        *v = serde_json::to_value(dv)
            .map_err(|_| APIError::CWL(commonwl::Error::Guard("Failed to re-serialize input")))?;
    } else {
        relativize_json_value(v, cwd)?;
    }
    Ok(())
}

#[allow(clippy::implicit_hasher)]
fn relativize_inputs(inputs: &mut HashMap<String, DefaultValue>, cwd: &Path) -> APIResult<()> {
    for dv in inputs.values_mut() {
        relativize_default_value(dv, cwd)?;
    }
    Ok(())
}
