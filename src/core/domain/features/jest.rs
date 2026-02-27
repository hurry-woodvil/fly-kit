use crate::domain::feature::Feature;
use crate::domain::plan::{FileArtifact, Plan};
use crate::infrastructure::command::package_manager::{DependencyKind, InstallRequest};

#[derive(Debug, Default)]
pub struct Jest;

impl Feature for Jest {
    fn key(&self) -> &'static str {
        "jest"
    }

    fn plan(&self) -> anyhow::Result<Plan> {
        let mut p = Plan::default();

        p.files.push(FileArtifact {
            path: "jest.config.ts".to_string(),
            content: include_str!("./templates/jest.config.ts").to_string(),
        });

        p.files.push(FileArtifact {
            path: "jest.setup.ts".to_string(),
            content: include_str!("./templates/jest.setup.ts").to_string(),
        });

        p.installs.push(InstallRequest {
            kind: DependencyKind::Dev,
            packages: vec![
                "jest".to_string(),
                "@testing-library/react".to_string(),
                "@testing-library/jest-dom".to_string(),
                "@testing-library/user-event".to_string(),
                "jest-environment-jsdom".to_string(),
            ],
        });

        Ok(p)
    }
}
