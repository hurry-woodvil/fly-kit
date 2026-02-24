use crate::core::command::npm;
use crate::core::project;
use anyhow::Result;
use std::path::PathBuf;

pub mod core;

pub async fn init(project_name: String, dir: Option<PathBuf>) -> Result<()> {
    let base = dir.unwrap_or(std::env::current_dir()?);
    let project_dir = base.join(&project_name);

    // create project
    project::create_project(&project_dir, project_name).await?;

    // npm command run
    npm::cmd_run(&project_dir).await?;

    Ok(())
}
