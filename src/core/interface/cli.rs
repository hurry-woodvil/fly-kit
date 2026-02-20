use clap;
use std::path::PathBuf;

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
        project_name: String,
        #[arg(long)]
        dir: Option<PathBuf>,
    },
}
