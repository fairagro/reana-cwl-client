use crate::api::client::ReanaClient;
use glob::Pattern;
use std::sync::Arc;

/// Globbing function for REANA Server
/// Notice similar glob signature as <https://github.com/fairagro/commonwl/blob/main/crates/storage/s3_storage.rs>
/// # Errors
/// If `Pattern` is invalid
pub async fn glob(
    client: Arc<ReanaClient>,
    id: &str,
    pattern: &str,
) -> anyhow::Result<Box<dyn Iterator<Item = String> + Send>> {
    //reana stores all in "outputs"
    let pattern = format!("outputs/{pattern}");
    let pattern = Pattern::new(&pattern)?;

    let res = crate::client::workspace(client.clone(), id).await?;
    let listing = res.items;

    let files: Vec<_> = listing
        .iter()
        .filter(move |i| pattern.matches(&i.name))
        .map(|s| s.name.clone())
        .collect();

    Ok(Box::new(files.into_iter()))
}
