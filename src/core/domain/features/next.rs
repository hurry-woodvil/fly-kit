use crate::domain::feature::Feature;
use crate::domain::plan::{FileArtifact, Plan};
use crate::infrastructure::command::package_manager::{DependencyKind, InstallRequest};

#[derive(Debug, Default)]
pub struct Next;

impl Feature for Next {
    fn key(&self) -> &'static str {
        "next"
    }

    fn plan(&self) -> anyhow::Result<Plan> {
        let mut p = Plan::default();

        p.files.push(FileArtifact {
            path: "next.config.mjs".to_string(),
            content: include_str!("./templates/next.config.mjs").to_string(),
        });

        p.files.push(FileArtifact {
            path: "next-env.d.ts".to_string(),
            content: include_str!("./templates/next-env.d.ts").to_string(),
        });

        p.installs.push(InstallRequest {
            kind: DependencyKind::Prod,
            packages: vec!["next".to_string()],
        });

        Ok(p)
    }
}
