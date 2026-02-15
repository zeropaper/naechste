use crate::config::Config;
use crate::diagnostics::DiagnosticCollection;
use crate::rules;
use std::path::Path;
use walkdir::WalkDir;

pub fn lint(path: &Path, config: &Config) -> DiagnosticCollection {
    let mut diagnostics = DiagnosticCollection::new();
    let mut all_files = Vec::new();

    // Walk through the project directory
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_ignored(e.path()))
    {
        if let Ok(entry) = entry {
            let file_path = entry.path();

            // Skip directories
            if !file_path.is_file() {
                continue;
            }

            // Skip non-relevant files
            if !is_relevant_file(file_path) {
                continue;
            }

            // Collect all files for batch processing
            all_files.push(file_path.to_path_buf());

            // Run per-file rules
            rules::check_server_side_exports(file_path, config, &mut diagnostics);
            rules::check_component_nesting_depth(file_path, config, &mut diagnostics);
            rules::check_filename_style(file_path, config, &mut diagnostics);
        }
    }

    // Run batch rules that need all files
    rules::check_file_organization(path, &all_files, config, &mut diagnostics);

    diagnostics
}

fn is_ignored(path: &Path) -> bool {
    let ignored_dirs = [
        "node_modules",
        ".next",
        ".git",
        "dist",
        "build",
        "coverage",
        "out",
        ".turbo",
    ];

    path.components().any(|component| {
        if let Some(name) = component.as_os_str().to_str() {
            ignored_dirs.contains(&name)
        } else {
            false
        }
    })
}

fn is_relevant_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_str().unwrap_or("");
        matches!(ext_str, "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs")
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_temp_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let mut file = fs::File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_is_ignored_node_modules() {
        assert!(is_ignored(Path::new("node_modules/package")));
        assert!(is_ignored(Path::new("./node_modules/package")));
        assert!(is_ignored(Path::new("src/node_modules/package")));
    }

    #[test]
    fn test_is_ignored_next_dir() {
        assert!(is_ignored(Path::new(".next/static")));
        assert!(is_ignored(Path::new("./.next/cache")));
    }

    #[test]
    fn test_is_ignored_git() {
        assert!(is_ignored(Path::new(".git/objects")));
        assert!(is_ignored(Path::new("./.git/config")));
    }

    #[test]
    fn test_is_ignored_build_dirs() {
        assert!(is_ignored(Path::new("dist/bundle.js")));
        assert!(is_ignored(Path::new("build/output")));
        assert!(is_ignored(Path::new("coverage/lcov")));
        assert!(is_ignored(Path::new("out/static")));
    }

    #[test]
    fn test_is_not_ignored() {
        assert!(!is_ignored(Path::new("src/components")));
        assert!(!is_ignored(Path::new("app/page.tsx")));
        assert!(!is_ignored(Path::new("pages/index.tsx")));
    }

    #[test]
    fn test_is_relevant_file_js_files() {
        assert!(is_relevant_file(Path::new("test.js")));
        assert!(is_relevant_file(Path::new("test.jsx")));
        assert!(is_relevant_file(Path::new("test.ts")));
        assert!(is_relevant_file(Path::new("test.tsx")));
        assert!(is_relevant_file(Path::new("test.mjs")));
        assert!(is_relevant_file(Path::new("test.cjs")));
    }

    #[test]
    fn test_is_relevant_file_non_js_files() {
        assert!(!is_relevant_file(Path::new("test.css")));
        assert!(!is_relevant_file(Path::new("test.json")));
        assert!(!is_relevant_file(Path::new("test.md")));
        assert!(!is_relevant_file(Path::new("test.txt")));
        assert!(!is_relevant_file(Path::new("README")));
    }

    #[test]
    fn test_lint_empty_directory() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-empty");
        fs::create_dir_all(&temp_dir).ok();
        
        let config = Config::default();
        let diagnostics = lint(&temp_dir, &config);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_lint_ignores_node_modules() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-ignore-nm");
        fs::create_dir_all(&temp_dir).ok();
        
        let file_path = temp_dir.join("node_modules/package/index.tsx");
        create_temp_file(&file_path, "'use client'\nexport const getServerSideProps = () => {}");
        
        let config = Config::default();
        let diagnostics = lint(&temp_dir, &config);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_lint_processes_app_directory() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-app-dir");
        fs::create_dir_all(&temp_dir).ok();
        
        let file_path = temp_dir.join("app/MyComponent.tsx");
        create_temp_file(&file_path, "export function MyComponent() {}");
        
        let mut config = Config::default();
        config.rules.filename_style_consistency.options.filename_style = crate::config::FilenameStyle::KebabCase;
        config.rules.filename_style_consistency.severity = crate::config::Severity::Error;
        
        let diagnostics = lint(&temp_dir, &config);
        
        assert!(diagnostics.diagnostics.len() > 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_lint_multiple_files() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-multiple");
        fs::create_dir_all(&temp_dir).ok();
        
        create_temp_file(&temp_dir.join("File1.tsx"), "export function File1() {}");
        create_temp_file(&temp_dir.join("File2.tsx"), "export function File2() {}");
        create_temp_file(&temp_dir.join("File3.tsx"), "export function File3() {}");
        
        let mut config = Config::default();
        config.rules.filename_style_consistency.options.filename_style = crate::config::FilenameStyle::KebabCase;
        
        let diagnostics = lint(&temp_dir, &config);
        
        assert_eq!(diagnostics.diagnostics.len(), 3);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_lint_ignores_non_js_files() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-non-js");
        fs::create_dir_all(&temp_dir).ok();
        
        create_temp_file(&temp_dir.join("README.md"), "# README");
        create_temp_file(&temp_dir.join("package.json"), "{}");
        create_temp_file(&temp_dir.join("styles.css"), "body {}");
        
        let config = Config::default();
        let diagnostics = lint(&temp_dir, &config);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }
}
