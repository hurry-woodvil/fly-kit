use super::TemplateFile;
use std::path::PathBuf;

pub struct Template;

impl Template {
    const HELLO_TEST: &str = r#"import { render, screen } from '@testing-library/react';
import { Hello } from '@/components/Hello';

test('renders greeting', () => {
  render(<Hello name="Next.js" />);
  expect(screen.getByText('Hello Next.js')).toBeInTheDocument();
});
"#;

    pub fn new(project_dir: &PathBuf) -> anyhow::Result<TemplateFile> {
        let dir = project_dir.join("__tests__");
        let file_name = "Hello.test.tsx".to_string();
        let content = Self::HELLO_TEST.to_string();
        Ok(TemplateFile {
            dir,
            file_name,
            content,
        })
    }
}
