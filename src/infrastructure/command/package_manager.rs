use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum DependencyKind {
    Prod,
    Dev,
}

#[derive(Debug, Clone)]
pub struct InstallRequest {
    pub kind: DependencyKind,
    pub packages: Vec<String>,
}

pub trait PackageManager: Send + Sync {
    fn name(&self) -> &'static str;

    fn init<'a>(
        &'a self,
        project_dir: &'a Path,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>>;

    /// e.g. `npm install` or `yarn install`
    fn install<'a>(
        &'a self,
        project_dir: &'a Path,
        req: InstallRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>>;
}
