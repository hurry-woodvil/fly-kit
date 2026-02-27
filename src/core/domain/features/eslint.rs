use crate::domain::feature::Feature;
use crate::domain::plan::{FileArtifact, Plan};
use crate::infrastructure::command::package_manager::{DependencyKind, InstallRequest};

#[derive(Debug, Default)]
pub struct Eslint;

impl Feature for Eslint {
    fn key(&self) -> &'static str {
        "eslint"
    }

    fn plan(&self) -> anyhow::Result<Plan> {
        let mut p = Plan::default();

        p.files.push(FileArtifact {
            path: ".eslintrc.json".to_string(),
            content: include_str!("./templates/.eslintrc.json").to_string(),
        });

        p.installs.push(InstallRequest {
            kind: DependencyKind::Dev,
            packages: vec![
                "eslint".to_string(),
                "eslint-config-next".to_string(),
                "eslint-config-prettier".to_string(),
            ],
        });

        Ok(p)
    }
}
