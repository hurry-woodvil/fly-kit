use crate::infrastructure::command::package_manager::InstallRequest;

#[derive(Debug, Clone)]
pub struct FileArtifact {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Default, Clone)]
pub struct Plan {
    pub files: Vec<FileArtifact>,
    pub installs: Vec<InstallRequest>,
}

impl Plan {
    pub fn merge(mut self, other: Plan) -> Plan {
        self.files.extend(other.files);
        self.installs.extend(other.installs);
        self
    }
}
