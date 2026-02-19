use anyhow::{Result, anyhow};
use std::path::PathBuf;

pub mod command;
pub mod interface;
pub mod package_json;
pub mod project;

pub async fn init_project(name: String, dir: Option<PathBuf>) -> Result<()> {
    validate_name(&name)?;

    let base_dir = match dir {
        Some(d) => d,
        None => std::env::current_dir()?,
    };

    let project_dir = base_dir.join(&name);

    project::create_project_dir(&project_dir).await?;
    project::write_package_json(&project_dir, &name).await?;

    Ok(())
}

fn validate_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(anyhow!("Project name must not be empty"));
    }
    Ok(())
}
