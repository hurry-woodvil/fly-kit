use self::builder::ProjectBuilder;
use self::feature::Feature;
use crate::core::{command::package_manager::PackageManager, io::writer::ProjectWriter};
use std::path::Path;

pub mod app;
mod builder;
mod feature;
pub mod features;
pub mod model;
pub mod package_json;

pub async fn create_project(
    project_dir: &Path,
    features: Vec<Box<dyn Feature>>,
    writer: &dyn ProjectWriter,
    pm: &dyn PackageManager,
    skip_install: bool,
    skip_create: bool,
) -> anyhow::Result<()> {
    let builder = features
        .into_iter()
        .fold(ProjectBuilder::new(), |b, f| b.add_feature_boxed(f));

    builder
        .build(project_dir, writer, pm, skip_install, skip_create)
        .await?;

    Ok(())
}
