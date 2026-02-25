use std::collections::HashSet;
use std::path::Path;

use super::feature::{Feature, FeatureContext};
use crate::core::command::package_manager::PackageManager;
use crate::core::io::writer::ProjectWriter;
use crate::core::project::app;
use crate::core::project::model::{DependencyKind, InstallPlan, InstallSpec, PackageName};

pub struct ProjectBuilder {
    features: Vec<Box<dyn Feature>>,
}

impl ProjectBuilder {
    pub fn new() -> Self {
        Self { features: vec![] }
    }

    // pub fn add_feature(mut self, f: impl Feature + 'static) -> Self {
    //     self.features.push(Box::new(f));
    //     self
    // }

    pub fn add_feature_boxed(mut self, f: Box<dyn Feature>) -> Self {
        self.features.push(f);
        self
    }

    pub async fn build(
        &self,
        project_dir: &Path,
        writer: &dyn ProjectWriter,
        pm: &dyn PackageManager,
        skip_install: bool,
        skip_create: bool,
    ) -> anyhow::Result<()> {
        if !skip_create {
            if tokio::fs::try_exists(project_dir).await? {
                anyhow::bail!(
                    "Project directory already exists: {}",
                    project_dir.display()
                );
            }
            writer.create_dir_all(project_dir).await?;
            pm.init_package_json(project_dir).await?;
            for f in &self.features {
                let ctx = FeatureContext {
                    project_dir,
                    writer,
                    pm,
                };
                f.apply(ctx).await?;
            }
            app::create_template_files(project_dir, writer).await?;
        }

        if !skip_install {
            let mut prod = Vec::<InstallSpec>::new();
            let mut dev = Vec::<InstallSpec>::new();

            for f in &self.features {
                for plan in f.install_plans() {
                    match plan.kind {
                        DependencyKind::Prod => prod.extend(plan.packages),
                        DependencyKind::Dev => dev.extend(plan.packages),
                    }
                }
            }

            prod = dedup(prod);
            dev = dedup(dev);

            if !prod.is_empty() {
                pm.install(
                    project_dir,
                    &InstallPlan {
                        kind: DependencyKind::Prod,
                        packages: prod,
                    },
                )
                .await?;
            }

            if !dev.is_empty() {
                pm.install(
                    project_dir,
                    &InstallPlan {
                        kind: DependencyKind::Dev,
                        packages: dev,
                    },
                )
                .await?;
            }
        }

        Ok(())
    }
}

fn dedup(mut specs: Vec<InstallSpec>) -> Vec<InstallSpec> {
    let mut seen = HashSet::<PackageName>::new();
    specs.retain(|s| seen.insert(s.name.clone()));
    specs
}
