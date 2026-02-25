use async_trait::async_trait;
use std::path::Path;

use crate::core::project::model::InstallPlan;

#[async_trait]
pub trait PackageManager: Send + Sync {
    fn id(&self) -> &'static str;

    async fn init_package_json(&self, dir: &Path) -> anyhow::Result<()>;
    async fn install(&self, dir: &Path, plan: &InstallPlan) -> anyhow::Result<()>;
}
