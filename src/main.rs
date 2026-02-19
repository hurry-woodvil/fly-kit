use anyhow::Result;
use clap::Parser;
use fly_kit::core::interface::cli::{Cli, SubCommand};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.sub_command {
        SubCommand::Init { project_name } => fly_kit::init(&project_name).await?,
    }

    Ok(())
}
