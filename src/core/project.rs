use anyhow::{Context, Result, anyhow};
use std::path::Path;
use tokio::fs;

mod app;
mod package_json;
pub mod packages;

pub async fn create_project(project_dir: &Path, project_name: String) -> Result<()> {
    // create proeject directory
    create_project_dir(project_dir).await?;

    // create package.json
    let pkg_json = package_json::PackageJson::new(project_name);
    pkg_json.write_package_json(project_dir).await?;

    // create template files
    app::create_template_files(project_dir).await?;

    Ok(())
}

async fn create_project_dir(project_dir: &Path) -> Result<()> {
    // check folder exists
    if fs::try_exists(project_dir).await? {
        return Err(anyhow!(
            "Project directory already exists: {}",
            project_dir.display()
        ));
    }

    // create project directory
    fs::create_dir_all(project_dir)
        .await
        .with_context(|| format!("Failed to create directory: {}", project_dir.display()))?;

    Ok(())
}
