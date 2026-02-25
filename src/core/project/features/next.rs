use crate::core::project::feature::Feature;
use crate::core::project::model::{
    DependencyKind, InstallPlan, InstallSpec, PackageName, PackageVersion, ProjectFile, Version,
};

pub struct NextFeature;

impl Feature for NextFeature {
    fn id(&self) -> &'static str {
        "next"
    }

    fn install_plans(&self) -> Vec<InstallPlan> {
        vec![InstallPlan {
            kind: DependencyKind::Prod,
            packages: vec![InstallSpec {
                name: PackageName("next"),
                version: PackageVersion(Version::Latest),
            }],
        }]
    }

    fn files(&self) -> Vec<ProjectFile> {
        vec![
            ProjectFile {
                path: "next.config.mjs".into(),
                content: include_str!("../../templates/next.config.mjs"),
            },
            ProjectFile {
                path: "next-env.d.ts".into(),
                content: include_str!("../../templates/next-env.d.ts"),
            },
        ]
    }
}
