use fly_kit::core;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    core::interface::cli::run().await
}
