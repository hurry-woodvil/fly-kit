pub mod npm;
pub mod package_manager;

pub async fn run_cmd<S>(dir: &std::path::Path, bin: &str, args: &[S]) -> anyhow::Result<()>
where
    S: AsRef<std::ffi::OsStr>,
{
    use std::process::Stdio;
    use tokio::process::Command;

    let status = Command::new(bin)
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        anyhow::bail!("command failed: {}", bin);
    }

    Ok(())
}
