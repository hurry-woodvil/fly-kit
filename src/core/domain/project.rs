use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Project {
    pub dir: PathBuf,
    pub features: Vec<String>,
}
