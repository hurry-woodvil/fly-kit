use async_trait::async_trait;
use std::path::Path;

use crate::core::command::package_manager::PackageManager;
use crate::core::io::writer::ProjectWriter;

use super::model::{InstallPlan, ProjectFile};

pub struct FeatureContext<'a> {
    pub project_dir: &'a Path,
    pub writer: &'a dyn ProjectWriter,
    pub pm: &'a dyn PackageManager,
}

#[async_trait]
pub trait Feature: Send + Sync {
    fn id(&self) -> &'static str;

    fn install_plans(&self) -> Vec<InstallPlan> {
        vec![]
    }

    fn files(&self) -> Vec<ProjectFile> {
        vec![]
    }

    async fn apply(&self, ctx: FeatureContext<'_>) -> anyhow::Result<()> {
        for f in self.files() {
            ctx.writer
                .write_file(ctx.project_dir.join(f.path), f.content)
                .await?;
        }
        Ok(())
    }
}
