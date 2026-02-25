use async_trait::async_trait;
use std::path::{Path, PathBuf};

use super::writer::ProjectWriter;

pub struct FsWriter;

#[async_trait]
impl ProjectWriter for FsWriter {
    async fn create_dir_all(&self, path: &Path) -> anyhow::Result<()> {
        tokio::fs::create_dir_all(path).await?;
        Ok(())
    }

    async fn write_file(&self, path: PathBuf, content: &str) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            self.create_dir_all(parent).await?;
        }
        tokio::fs::write(path, content).await?;
        Ok(())
    }
}
