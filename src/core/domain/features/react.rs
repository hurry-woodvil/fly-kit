use crate::domain::feature::Feature;
use crate::domain::plan::Plan;
use crate::infrastructure::command::package_manager::{DependencyKind, InstallRequest};

#[derive(Debug, Default)]
pub struct React;

impl Feature for React {
    fn key(&self) -> &'static str {
        "jest"
    }

    fn plan(&self) -> anyhow::Result<Plan> {
        let mut p = Plan::default();

        p.installs.push(InstallRequest {
            kind: DependencyKind::Prod,
            packages: vec!["react".to_string(), "react-dom".to_string()],
        });

        Ok(p)
    }
}
