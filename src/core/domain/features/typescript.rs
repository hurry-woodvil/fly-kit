use crate::domain::feature::Feature;
use crate::domain::plan::{FileArtifact, Plan};
use crate::infrastructure::command::package_manager::{DependencyKind, InstallRequest};

#[derive(Debug, Default)]
pub struct Typescript;

impl Feature for Typescript {
    fn key(&self) -> &'static str {
        "typescript"
    }

    fn plan(&self) -> anyhow::Result<Plan> {
        let mut p = Plan::default();

        p.files.push(FileArtifact {
            path: "tsconfig.json".to_string(),
            content: include_str!("./templates/tsconfig.json").to_string(),
        });

        p.installs.push(InstallRequest {
            kind: DependencyKind::Dev,
            packages: vec![
                "typescript".to_string(),
                "@types/node".to_string(),
                "@types/jest".to_string(),
                "@types/react".to_string(),
                "@types/react-dom".to_string(),
            ],
        });

        Ok(p)
    }
}
