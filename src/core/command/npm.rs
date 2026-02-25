use anyhow::Context;
use async_trait::async_trait;
use std::path::Path;

use crate::core::command::package_manager::PackageManager;
use crate::core::project::model::{DependencyKind, InstallPlan, Version};

pub struct Npm;

#[async_trait]
impl PackageManager for Npm {
    fn id(&self) -> &'static str {
        "npm"
    }

    async fn init_package_json(&self, dir: &Path) -> anyhow::Result<()> {
        // npm init -y
        // crate::core::command::run_cmd(dir, "npm", &["init", "-y"]).await
        let project_name = dir
            .file_name()
            .and_then(|name| name.to_str())
            .context("Failed to determine project directory name")?;
        let package_json =
            crate::core::project::package_json::PackageJson::new(project_name.to_string());
        package_json.write_package_json(dir).await
    }

    async fn install(&self, dir: &Path, plan: &InstallPlan) -> anyhow::Result<()> {
        let mut args: Vec<String> = vec!["install".to_string()];
        if matches!(plan.kind, DependencyKind::Dev) {
            args.push("-D".to_string());
        }

        for p in &plan.packages {
            match p.version.0 {
                Version::Latest => args.push(p.name.0.to_string()),
                Version::Pin(v) => args.push(format!("{}@{}", p.name.0, v)),
            }
        }

        crate::core::command::run_cmd(dir, "npm", &args).await
    }
}
