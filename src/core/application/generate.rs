// module import
use super::app::create_template_files;
use crate::domain::plan::Plan;
use crate::infrastructure::command::package_manager::PackageManager;
use crate::infrastructure::io::writer::Writer;
use std::path::Path;

pub struct Generator<'a> {
    writer: &'a dyn Writer,
    pm: &'a dyn PackageManager,
}

impl<'a> Generator<'a> {
    pub fn new(writer: &'a dyn Writer, pm: &'a dyn PackageManager) -> Self {
        Self { writer, pm }
    }

    pub async fn execute(&self, project_dir: &Path, plan: Plan) -> anyhow::Result<()> {
        // 1) files
        for f in plan.files {
            self.writer
                .write(project_dir.join(&f.path), &f.content)
                .await?;
        }

        // 2) installs
        for req in plan.installs {
            self.pm.install(project_dir, req).await?;
        }

        // 3) template files
        create_template_files(project_dir, self.writer).await?;

        Ok(())
    }
}
