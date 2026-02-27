// module import
use crate::application::generate::Generator;
use crate::domain::{features, plan::Plan};
use crate::infrastructure::{command::package_manager::PackageManager, io::writer::Writer};
use crate::ports::dto::InitProjectRequest;

pub struct InitProject<'a> {
    pub writer: &'a dyn Writer,
    pub pm: &'a dyn PackageManager,
}

impl<'a> InitProject<'a> {
    pub async fn execute(&self, req: InitProjectRequest) -> anyhow::Result<()> {
        let project_dir = req.project_dir();

        tokio::fs::create_dir_all(project_dir).await?;

        self.pm.init(&req.project_dir()).await?;

        let feats = features::resolve(&req.features)?;
        let plan = feats.into_iter().try_fold(Plan::default(), |acc, f| {
            Ok::<_, anyhow::Error>(acc.merge(f.plan()?))
        })?;

        let generator = Generator::new(self.writer, self.pm);
        generator.execute(&req.project_dir(), plan).await?;

        Ok(())
    }
}
