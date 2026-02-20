use crate::core::utils;
use anyhow::Result;
use serde_json::json;
use std::path::Path;

pub async fn create_modules(project_dir: &Path, name: &str) -> Result<()> {
    println!("▶ Writing template files...");

    utils::write(project_dir.join("package.json"), &package_json(name)).await?;
    utils::write(project_dir.join("next.config.mjs"), NEXT_CONFIG).await?;
    utils::write(project_dir.join("tsconfig.json"), TSCONFIG).await?;
    utils::write(project_dir.join("next-env.d.ts"), NEXT_ENV).await?;
    utils::write(project_dir.join(".eslintrc.json"), ESLINT).await?;
    utils::write(project_dir.join(".prettierrc.json"), PRETTIER).await?;
    utils::write(project_dir.join(".prettierignore"), PRETTIER_IGNORE).await?;
    utils::write(project_dir.join("jest.config.ts"), JEST_CONFIG).await?;
    utils::write(project_dir.join("jest.setup.ts"), JEST_SETUP).await?;

    // src structure
    utils::write(project_dir.join("src/app/layout.tsx"), LAYOUT).await?;
    utils::write(project_dir.join("src/app/page.tsx"), HOME).await?;
    utils::write(project_dir.join("src/app/globals.css"), GLOBALS).await?;
    utils::write(project_dir.join("src/app/about/page.tsx"), ABOUT).await?;
    utils::write(project_dir.join("src/components/Hello.tsx"), HELLO).await?;
    utils::write(project_dir.join("src/__tests__/Hello.test.tsx"), HELLO_TEST).await?;

    Ok(())
}

fn package_json(name: &str) -> String {
    json!({
        "name": name,
        "private": true,
        "version": "0.1.0",
        "type": "module",
        "scripts": {
            "dev": "next dev",
            "build": "next build",
            "start": "next start",
            "lint": "next lint",
            "format": "prettier . --check",
            "format:write": "prettier . --write",
            "test": "jest",
            "test:watch": "jest --watch"
        }
    })
    .to_string()
}

const NEXT_CONFIG: &str = r#"const nextConfig = {};
export default nextConfig;
"#;

const NEXT_ENV: &str = r#"/// <reference types="next" />
/// <reference types="next/image-types/global" />
"#;

const TSCONFIG: &str = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["dom", "dom.iterable", "esnext"],
    "strict": true,
    "noEmit": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "jsx": "preserve",
    "baseUrl": ".",
    "paths": { "@/*": ["src/*"] },
    "plugins": [{ "name": "next" }]
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx"],
  "exclude": ["node_modules"]
}
"#;

const ESLINT: &str = r#"{
  "extends": ["next/core-web-vitals", "prettier"]
}
"#;

const PRETTIER: &str = r#"{
  "semi": true,
  "singleQuote": true,
  "printWidth": 100
}
"#;

const PRETTIER_IGNORE: &str = ".next\nnode_modules\n";

const JEST_CONFIG: &str = r#"import nextJest from 'next/jest';
const createJestConfig = nextJest({ dir: './' });

const config = {
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts'],
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/src/$1',
  },
};

export default createJestConfig(config);
"#;

const JEST_SETUP: &str = "import '@testing-library/jest-dom';";

const LAYOUT: &str = r#"import './globals.css';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ja">
      <body>{children}</body>
    </html>
  );
}
"#;

const HOME: &str = r#"export default function HomePage() {
  return (
    <main>
      <h1>Hello Next.js</h1>
    </main>
  );
}
"#;

const GLOBALS: &str = "html,body{margin:0;padding:0;}";

const ABOUT: &str = r#"export default function AboutPage() {
  return <h1>About</h1>;
}
"#;

const HELLO: &str = r#"export function Hello({ name }: { name: string }) {
  return <p>Hello {name}</p>;
}
"#;

const HELLO_TEST: &str = r#"import { render, screen } from '@testing-library/react';
import { Hello } from '@/components/Hello';

test('renders greeting', () => {
  render(<Hello name="Next.js" />);
  expect(screen.getByText('Hello Next.js')).toBeInTheDocument();
});
"#;
