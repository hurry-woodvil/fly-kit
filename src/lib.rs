use anyhow::Result;

pub mod core;

pub async fn init(project_name: &str) -> Result<()> {
    // TODO: add initialization logic here (e.g. create project directory, generate files, etc.)
    println!("Initializing project: {}", project_name);

    Ok(())
}
