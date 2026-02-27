use crate::domain::features;
use crate::ports::dto::{InitProjectRequest, PackageManagerKind};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "fly-kit")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(InitArgs),
}

#[derive(Debug, Parser)]
pub struct InitArgs {
    #[arg(value_name = "PROJECT_NAME")]
    pub project_name: String,
    #[arg(long, default_value = ".")]
    pub dir: PathBuf,
    #[arg(long = "feature")]
    pub features: Vec<String>,
    #[arg(long, default_value = "npm")]
    pub package_manager: String,
}

impl TryFrom<InitArgs> for InitProjectRequest {
    type Error = anyhow::Error;

    fn try_from(v: InitArgs) -> anyhow::Result<Self, Self::Error> {
        if v.project_name.trim().is_empty() {
            anyhow::bail!("project name cannot be empty");
        }

        let package_manager: PackageManagerKind = v.package_manager.parse()?;

        let features = if v.features.is_empty() {
            features::default_futures_keys()
        } else {
            v.features
        };

        Ok(InitProjectRequest {
            project_name: v.project_name,
            dir: v.dir,
            features,
            package_manager,
        })
    }
}
