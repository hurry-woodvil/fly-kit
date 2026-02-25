use async_trait::async_trait;
use std::path::{Path, PathBuf};

#[async_trait]
pub trait ProjectWriter: Send + Sync {
    async fn create_dir_all(&self, path: &Path) -> anyhow::Result<()>;
    async fn write_file(&self, path: PathBuf, content: &str) -> anyhow::Result<()>;
}
