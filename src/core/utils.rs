use anyhow::Result;
use std::path::PathBuf;

pub async fn write(path: PathBuf, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(path, contents).await?;
    Ok(())
}
