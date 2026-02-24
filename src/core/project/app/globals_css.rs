use crate::core::project::app;
use std::path::PathBuf;

pub struct Template;

impl Template {
    const GLOBALS: &str = "html,body{margin:0;padding:0;}";

    pub fn new(project_dir: &PathBuf) -> anyhow::Result<app::TemplateFile> {
        let dir = project_dir.join("app");
        let file_name = "globals.css".to_string();
        let content = Self::GLOBALS.to_string();
        Ok(app::TemplateFile {
            dir,
            file_name,
            content,
        })
    }
}
