use crate::core::project::feature::Feature;
use crate::core::project::model::{
    DependencyKind, InstallPlan, InstallSpec, PackageName, PackageVersion, Version,
};

pub struct ReactFeature;

impl Feature for ReactFeature {
    fn id(&self) -> &'static str {
        "react"
    }

    fn install_plans(&self) -> Vec<InstallPlan> {
        vec![InstallPlan {
            kind: DependencyKind::Prod,
            packages: vec![
                InstallSpec {
                    name: PackageName("react"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("react-dom"),
                    version: PackageVersion(Version::Latest),
                },
            ],
        }]
    }
}
