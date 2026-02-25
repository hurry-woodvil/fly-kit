use crate::core::project::feature::Feature;
use crate::core::project::model::{
    DependencyKind, InstallPlan, InstallSpec, PackageName, PackageVersion, ProjectFile, Version,
};

pub struct PrettierFeature;

impl Feature for PrettierFeature {
    fn id(&self) -> &'static str {
        "prettier"
    }

    fn install_plans(&self) -> Vec<InstallPlan> {
        vec![InstallPlan {
            kind: DependencyKind::Dev,
            packages: vec![InstallSpec {
                name: PackageName("prettier"),
                version: PackageVersion(Version::Latest),
            }],
        }]
    }

    fn files(&self) -> Vec<ProjectFile> {
        vec![
            ProjectFile {
                path: ".prettierrc.json".into(),
                content: include_str!("../../templates/.prettierrc.json"),
            },
            ProjectFile {
                path: ".prettierignore".into(),
                content: include_str!("../../templates/.prettierignore"),
            },
        ]
    }
}
