use super::TemplateFile;
use std::path::PathBuf;

pub struct Template;

impl Template {
    const LAYOUT: &str = r#"import './globals.css';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ja">
      <body>{children}</body>
    </html>
  );
}
"#;

    pub fn new(project_dir: &PathBuf) -> anyhow::Result<TemplateFile> {
        let dir = project_dir.join("app");
        let file_name = "layout.tsx".to_string();
        let content = Self::LAYOUT.to_string();
        Ok(TemplateFile {
            dir,
            file_name,
            content,
        })
    }
}
