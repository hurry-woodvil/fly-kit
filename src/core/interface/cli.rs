use clap::{self, Parser};
use std::path::PathBuf;

use crate::core::{
    command::npm::Npm,
    io::fs_writer::FsWriter,
    project::{self, features},
};

#[derive(clap::Parser, Debug)]
#[command(name = "fly-kit", version, about = "Frontend project generator")]
pub struct Cli {
    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    /// Generate a new project
    Init {
        /// Project name (e.g. my-app)
        name: String,

        /// Output directory (default: current directory)
        #[arg(long)]
        out_dir: Option<PathBuf>,

        /// Comma-separated feature list (e.g. "typescript,jest")
        #[arg(long)]
        features: Option<String>,

        /// Skip npm install
        #[arg(long)]
        skip_install: bool,

        /// Skip creating files (dry-ish)
        #[arg(long)]
        skip_create: bool,
    },
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.sub_command {
        SubCommand::Init {
            name,
            out_dir,
            features: feature_csv,
            skip_install,
            skip_create,
        } => {
            let base = out_dir.unwrap_or(std::env::current_dir()?);
            let project_dir = base.join(&name);

            // Feature selection
            let feats = if let Some(csv) = feature_csv {
                let names: Vec<&str> = csv
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();
                features::resolve_features(&names)?
            } else {
                features::default_features()
            };

            // Wire dependencies
            let writer = FsWriter;
            let pm = Npm;

            project::create_project(&project_dir, feats, &writer, &pm, skip_install, skip_create)
                .await?;

            Ok(())
        }
    }
}
