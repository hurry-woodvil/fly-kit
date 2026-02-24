use crate::core::project::packages;
use std::path::Path;

pub async fn install_dependencies(project_dir: &Path) -> anyhow::Result<()> {
    let plan = packages::InstallPlan {
        kind: packages::DependencyKind::Prod,
        packages: packages::PackageBuilder::package_build()?,
    };

    plan.npm_command(project_dir).await?;

    Ok(())
}

pub async fn install_dev_dependencies(project_dir: &Path) -> anyhow::Result<()> {
    let plan = packages::InstallPlan {
        kind: packages::DependencyKind::Dev,
        packages: packages::PackageBuilder::dev_package_build()?,
    };

    plan.npm_command(project_dir).await?;

    Ok(())
}
