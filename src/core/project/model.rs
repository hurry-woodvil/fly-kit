use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    Prod,
    Dev,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageName(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    Latest,
    Pin(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageVersion(pub Version);

#[derive(Debug, Clone)]
pub struct InstallSpec {
    pub name: PackageName,
    pub version: PackageVersion,
}

#[derive(Debug, Clone)]
pub struct InstallPlan {
    pub kind: DependencyKind,
    pub packages: Vec<InstallSpec>,
}

#[derive(Debug, Clone)]
pub struct ProjectFile {
    pub path: PathBuf,
    pub content: &'static str,
}
