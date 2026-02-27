use super::TemplateFile;
use std::path::PathBuf;

pub struct Template;

impl Template {
    const HOME: &str = r#"export default function HomePage() {
  return (
    <main>
      <h1>Hello Next.js</h1>
    </main>
  );
}
"#;

    pub fn new(project_dir: &PathBuf) -> anyhow::Result<TemplateFile> {
        let dir = project_dir.join("app");
        let file_name = "page.tsx".to_string();
        let content = Self::HOME.to_string();
        Ok(TemplateFile {
            dir,
            file_name,
            content,
        })
    }
}
