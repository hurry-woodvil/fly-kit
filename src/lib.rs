use anyhow::Result;
use std::path::PathBuf;

pub mod core;

pub async fn init(project_name: String, dir: Option<PathBuf>) -> Result<()> {
    let base = dir.unwrap_or(std::env::current_dir()?);
    let project_dir = base.join(&project_name);

    // TODO: add initialization logic here (e.g. create project directory, generate files, etc.)
    core::modules::create_modules(&project_dir, &project_name).await?;

    Ok(())
}
