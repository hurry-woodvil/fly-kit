use anyhow::{Context, Result, anyhow};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::process::Command;

pub async fn run_cmd<I, S>(cwd: &Path, program: &str, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .await
        .with_context(|| format!("Failed to run: {}", program))?;

    if !status.success() {
        return Err(anyhow!("Command failed: {}", status));
    }

    Ok(())
}

pub async fn write(path: PathBuf, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(path, contents).await?;
    Ok(())
}
