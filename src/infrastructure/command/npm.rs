use anyhow::Context;
use serde::Serialize;
use std::process::Stdio;
use std::{collections::BTreeMap, path::Path};
use tokio::process::Command;

use super::package_manager::{DependencyKind, PackageManager};
use crate::infrastructure::command::package_manager::InstallRequest;

#[derive(Debug, Default)]
pub struct Npm;

impl Npm {
    async fn run(project_dir: &Path, args: &[&str]) -> anyhow::Result<()> {
        let status = Command::new("npm")
            .args(args)
            .current_dir(project_dir)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await?;

        if !status.success() {
            anyhow::bail!("npm command failed: {:?}", args);
        }

        Ok(())
    }
}

impl PackageManager for Npm {
    fn name(&self) -> &'static str {
        "npm"
    }

    fn init<'a>(
        &'a self,
        project_dir: &'a Path,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
        let name = project_dir.file_name();
        Box::pin(async move {
            let name = name
                .and_then(|s| s.to_str().map(|s| s.to_string()))
                .unwrap();
            let package_json = PackageJson::new(name);
            package_json.write_package_json(project_dir).await?;
            Ok(())
        })
    }

    fn install<'a>(
        &'a self,
        project_dir: &'a Path,
        req: InstallRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            if req.packages.is_empty() {
                return Ok(());
            }

            let mut args: Vec<&str> = vec!["install"];

            if matches!(req.kind, DependencyKind::Dev) {
                args.push("-D");
            }

            let pkgs = req.packages.clone();
            let pkg_refs: Vec<&str> = pkgs.iter().map(|s| s.as_str()).collect();

            let mut all: Vec<&str> = Vec::with_capacity(args.len() + pkg_refs.len());
            all.extend(args);
            all.extend(pkg_refs);

            Self::run(project_dir, &all).await
        })
    }
}

#[derive(Debug, Serialize)]
pub struct PackageJson {
    name: String,
    version: String,
    private: bool,
    #[serde(rename = "type")]
    package_type: String,
    scripts: BTreeMap<String, String>,
}

impl PackageJson {
    pub fn new(name: String) -> Self {
        let mut scripts: BTreeMap<String, String> = BTreeMap::new();
        scripts.insert("dev".to_string(), "next dev".to_string());
        scripts.insert("build".to_string(), "next build".to_string());
        scripts.insert("start".to_string(), "next start".to_string());
        scripts.insert("lint".to_string(), "next lint".to_string());
        scripts.insert("format".to_string(), "prettier . --check".to_string());
        scripts.insert("format:write".to_string(), "prettier . --write".to_string());
        scripts.insert("test".to_string(), "jest".to_string());
        scripts.insert("test:watch".to_string(), "jest  --watch".to_string());
        Self {
            name,
            version: "0.1.0".to_string(),
            private: true,
            package_type: "module".to_string(),
            scripts,
        }
    }

    pub async fn write_package_json(&self, project_dir: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self).context("Failed to serialize package.")?;

        let path = project_dir.join("package.json");
        tokio::fs::write(&path, format!("{json}\n"))
            .await
            .with_context(|| format!("Failed to write file: {}", path.display()))?;

        Ok(())
    }
}
