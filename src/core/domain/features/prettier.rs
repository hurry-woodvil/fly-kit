use crate::domain::feature::Feature;
use crate::domain::plan::{FileArtifact, Plan};
use crate::infrastructure::command::package_manager::{DependencyKind, InstallRequest};

#[derive(Debug, Default)]
pub struct Prettier;

impl Feature for Prettier {
    fn key(&self) -> &'static str {
        "prettier"
    }

    fn plan(&self) -> anyhow::Result<Plan> {
        let mut p = Plan::default();

        p.files.push(FileArtifact {
            path: ".prettierrc.json".to_string(),
            content: include_str!("./templates/.prettierrc.json").to_string(),
        });

        p.files.push(FileArtifact {
            path: ".prettierignore".to_string(),
            content: include_str!("./templates/.prettierignore").to_string(),
        });

        p.installs.push(InstallRequest {
            kind: DependencyKind::Dev,
            packages: vec!["prettier".to_string()],
        });

        Ok(p)
    }
}
