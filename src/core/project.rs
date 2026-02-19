use crate::core::package_json::PackageJson;
use anyhow::{Context, Result, anyhow};
use std::path::Path;
use tokio::fs;

pub async fn create_project_dir(project_dir: &Path) -> Result<()> {
    if fs::try_exists(project_dir).await? {
        return Err(anyhow!(
            "Project directory already exists: {}",
            project_dir.display()
        ));
    }

    fs::create_dir_all(project_dir)
        .await
        .with_context(|| format!("Failed to create directory: {}", project_dir.display()))?;

    Ok(())
}

pub async fn write_package_json(project_dir: &Path, name: &str) -> Result<()> {
    let pkg = PackageJson::new(name.to_string());
    let json = serde_json::to_string_pretty(&pkg).context("Failed to serialize package.")?;

    let path = project_dir.join("package.json");
    fs::write(&path, format!("{json}\n"))
        .await
        .with_context(|| format!("Failed to write file: {}", path.display()))?;

    Ok(())
}
