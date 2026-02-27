use std::path::PathBuf;

pub trait Writer: Send + Sync {
    fn write<'a>(
        &'a self,
        path: PathBuf,
        content: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>>;
}
