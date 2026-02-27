// module defined
pub mod core;
pub mod infrastructure;
pub mod interface;

// re-export
pub use core::application;
pub use core::domain;
pub use core::ports;
pub use interface::cli::{Cli, Commands};

// module import
use application::init_project::InitProject;
use infrastructure::command::npm::Npm;
use infrastructure::io::fs_writer::FsWriter;
use ports::dto::InitProjectRequest;
use ports::dto::PackageManagerKind;

pub async fn init(req: InitProjectRequest) -> anyhow::Result<()> {
    let writer = FsWriter::default();
    let pm = match req.package_manager {
        PackageManagerKind::Npm => Npm::default(),
    };
    let init_project = InitProject {
        writer: &writer,
        pm: &pm,
    };
    init_project.execute(req).await?;
    Ok(())
}
