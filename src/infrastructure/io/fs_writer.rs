use super::writer::Writer;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct FsWriter;

impl Writer for FsWriter {
    fn write<'a>(
        &'a self,
        path: PathBuf,
        content: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            tokio::fs::write(path, content).await?;
            Ok(())
        })
    }
}
