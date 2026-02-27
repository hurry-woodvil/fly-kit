use clap::Parser;
use fly_kit::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            fly_kit::init(args.try_into()?).await?;
        }
    }

    Ok(())
}
