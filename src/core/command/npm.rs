use std::path::Path;

mod install_cmd;

pub async fn cmd_run(project_dir: &Path) -> anyhow::Result<()> {
    install_cmd::install_dependencies(project_dir).await?;

    install_cmd::install_dev_dependencies(project_dir).await?;

    Ok(())
}
