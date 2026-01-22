use crate::config::{Config, FilenameStyle};
use crate::diagnostics::{Diagnostic, DiagnosticCollection};
use regex::Regex;
use std::fs;
use std::path::Path;

/// Check for server-side exports in client components
pub fn check_server_side_exports(
    path: &Path,
    config: &Config,
    diagnostics: &mut DiagnosticCollection,
) {
    // Read file content
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    // Check if file has 'use client' directive
    let has_use_client = content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed == "'use client'" || trimmed == "\"use client\""
    });

    if !has_use_client {
        return;
    }

    // List of server-side only exports
    let server_exports = [
        "getServerSideProps",
        "getStaticProps",
        "getStaticPaths",
        "getInitialProps",
    ];

    for export in &server_exports {
        let pattern = format!(r"export\s+(const|function|async\s+function)\s+{}", export);
        if let Ok(re) = Regex::new(&pattern) {
            if re.is_match(&content) {
                diagnostics.add(Diagnostic {
                    severity: config.rules.server_side_exports.severity,
                    rule: "server-side-exports".to_string(),
                    message: format!(
                        "Server-side export '{}' found in client component",
                        export
                    ),
                    file: path.to_path_buf(),
                    line: None,
                });
            }
        }
    }
}

/// Check component nesting depth
pub fn check_component_nesting_depth(
    path: &Path,
    config: &Config,
    diagnostics: &mut DiagnosticCollection,
) {
    // Check if this is in app directory or pages directory
    let path_str = path.to_str().unwrap_or("");
    
    if !path_str.contains("/app/") && !path_str.contains("/pages/") {
        return;
    }

    // Count the nesting depth relative to app or pages directory
    let depth = if let Some(pos) = path_str.find("/app/") {
        count_depth(&path_str[pos + 5..])
    } else if let Some(pos) = path_str.find("/pages/") {
        count_depth(&path_str[pos + 7..])
    } else {
        return;
    };

    let max_depth = config.rules.component_nesting_depth.options.max_nesting_depth;
    
    if depth > max_depth {
        diagnostics.add(Diagnostic {
            severity: config.rules.component_nesting_depth.severity,
            rule: "component-nesting-depth".to_string(),
            message: format!(
                "Component nesting depth {} exceeds maximum of {}",
                depth, max_depth
            ),
            file: path.to_path_buf(),
            line: None,
        });
    }
}

fn count_depth(path_part: &str) -> usize {
    path_part.split('/').filter(|s| !s.is_empty()).count()
}

/// Check filename style consistency
pub fn check_filename_style(
    path: &Path,
    config: &Config,
    diagnostics: &mut DiagnosticCollection,
) {
    let filename = match path.file_stem() {
        Some(name) => name.to_str().unwrap_or(""),
        None => return,
    };

    // Skip special Next.js files and config files
    let special_files = [
        "page",
        "layout",
        "template",
        "loading",
        "error",
        "not-found",
        "route",
        "default",
        "middleware",
        // Config files
        "next.config",
        "tailwind.config",
        "postcss.config",
        "eslint.config",
        "tsconfig",
        "jsconfig",
        "vitest.config",
        "jest.config",
    ];

    if special_files.contains(&filename) {
        return;
    }

    let expected_style = config.rules.filename_style_consistency.options.filename_style;
    let matches_style = match expected_style {
        FilenameStyle::KebabCase => is_kebab_case(filename),
        FilenameStyle::CamelCase => is_camel_case(filename),
        FilenameStyle::PascalCase => is_pascal_case(filename),
        FilenameStyle::SnakeCase => is_snake_case(filename),
    };

    if !matches_style {
        diagnostics.add(Diagnostic {
            severity: config.rules.filename_style_consistency.severity,
            rule: "filename-style-consistency".to_string(),
            message: format!(
                "Filename '{}' does not match expected style: {:?}",
                filename, expected_style
            ),
            file: path.to_path_buf(),
            line: None,
        });
    }
}

fn is_kebab_case(s: &str) -> bool {
    let re = Regex::new(r"^[a-z][a-z0-9]*(-[a-z0-9]+)*$").unwrap();
    re.is_match(s)
}

fn is_camel_case(s: &str) -> bool {
    let re = Regex::new(r"^[a-z][a-zA-Z0-9]*$").unwrap();
    re.is_match(s) && s.chars().any(|c| c.is_uppercase())
}

fn is_pascal_case(s: &str) -> bool {
    let re = Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
    re.is_match(s) && s.chars().any(|c| c.is_lowercase())
}

fn is_snake_case(s: &str) -> bool {
    let re = Regex::new(r"^[a-z][a-z0-9]*(_[a-z0-9]+)*$").unwrap();
    re.is_match(s)
}

/// Check for missing companion files (e.g., test files, story files)
pub fn check_missing_companion_files(
    path: &Path,
    config: &Config,
    diagnostics: &mut DiagnosticCollection,
) {
    let options = &config.rules.missing_companion_files.options;
    
    if !options.require_test_files && !options.require_story_files {
        return;
    }

    // Only check component files (tsx/jsx)
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext != "tsx" && ext != "jsx" {
        return;
    }

    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    
    // Skip test and story files themselves
    if file_stem.ends_with(".test") || file_stem.ends_with(".spec") 
        || file_stem.ends_with(".stories") || file_stem.ends_with(".story") {
        return;
    }
    
    let parent = path.parent().unwrap_or(Path::new(""));

    // Check for test file
    if options.require_test_files {
        let test_patterns = [
            format!("{}.test.{}", file_stem, ext),
            format!("{}.spec.{}", file_stem, ext),
            format!("__tests__/{}.{}", file_stem, ext),
        ];

        let has_test = test_patterns.iter().any(|pattern| {
            parent.join(pattern).exists()
        });

        if !has_test {
            diagnostics.add(Diagnostic {
                severity: config.rules.missing_companion_files.severity,
                rule: "missing-companion-files".to_string(),
                message: format!("Missing test file for component '{}'", file_stem),
                file: path.to_path_buf(),
                line: None,
            });
        }
    }

    // Check for story file
    if options.require_story_files {
        let story_patterns = [
            format!("{}.stories.{}", file_stem, ext),
            format!("{}.story.{}", file_stem, ext),
        ];

        let has_story = story_patterns.iter().any(|pattern| {
            parent.join(pattern).exists()
        });

        if !has_story {
            diagnostics.add(Diagnostic {
                severity: config.rules.missing_companion_files.severity,
                rule: "missing-companion-files".to_string(),
                message: format!("Missing story file for component '{}'", file_stem),
                file: path.to_path_buf(),
                line: None,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, FilenameStyle};
    use std::fs;
    use std::io::Write;

    fn create_temp_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let mut file = fs::File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    fn get_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_is_kebab_case() {
        assert!(is_kebab_case("my-component"));
        assert!(is_kebab_case("button"));
        assert!(is_kebab_case("my-long-component-name"));
        assert!(is_kebab_case("component123"));
        assert!(is_kebab_case("component-123"));
        
        assert!(!is_kebab_case("MyComponent"));
        assert!(!is_kebab_case("my_component"));
        assert!(!is_kebab_case("myComponent"));
        assert!(!is_kebab_case("My-Component"));
        assert!(!is_kebab_case("-my-component"));
        assert!(!is_kebab_case("my-component-"));
    }

    #[test]
    fn test_is_camel_case() {
        assert!(is_camel_case("myComponent"));
        assert!(is_camel_case("myLongComponentName"));
        assert!(is_camel_case("component123Name"));
        
        assert!(!is_camel_case("MyComponent"));
        assert!(!is_camel_case("my-component"));
        assert!(!is_camel_case("my_component"));
        assert!(!is_camel_case("component"));
        assert!(!is_camel_case("COMPONENT"));
    }

    #[test]
    fn test_is_pascal_case() {
        assert!(is_pascal_case("MyComponent"));
        assert!(is_pascal_case("Button"));
        assert!(is_pascal_case("MyLongComponentName"));
        assert!(is_pascal_case("Component123"));
        
        assert!(!is_pascal_case("myComponent"));
        assert!(!is_pascal_case("my-component"));
        assert!(!is_pascal_case("my_component"));
        assert!(!is_pascal_case("COMPONENT"));
    }

    #[test]
    fn test_is_snake_case() {
        assert!(is_snake_case("my_component"));
        assert!(is_snake_case("button"));
        assert!(is_snake_case("my_long_component_name"));
        assert!(is_snake_case("component_123"));
        
        assert!(!is_snake_case("MyComponent"));
        assert!(!is_snake_case("my-component"));
        assert!(!is_snake_case("myComponent"));
        assert!(!is_snake_case("_my_component"));
        assert!(!is_snake_case("my_component_"));
    }

    #[test]
    fn test_server_side_exports_in_client_component() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-server-exports");
        fs::create_dir_all(&temp_dir).ok();
        
        let file_path = temp_dir.join("MyComponent.tsx");
        let content = r#"
'use client'

export function MyComponent() {
    return <div>Hello</div>;
}

export async function getServerSideProps() {
    return { props: {} };
}
"#;
        create_temp_file(&file_path, content);
        
        let config = get_test_config();
        let mut diagnostics = DiagnosticCollection::new();
        
        check_server_side_exports(&file_path, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 1);
        assert!(diagnostics.diagnostics[0].message.contains("getServerSideProps"));
        assert_eq!(diagnostics.diagnostics[0].rule, "server-side-exports");
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_server_side_exports_without_use_client() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-no-use-client");
        fs::create_dir_all(&temp_dir).ok();
        
        let file_path = temp_dir.join("page.tsx");
        let content = r#"
export async function getServerSideProps() {
    return { props: {} };
}

export default function Page() {
    return <div>Hello</div>;
}
"#;
        create_temp_file(&file_path, content);
        
        let config = get_test_config();
        let mut diagnostics = DiagnosticCollection::new();
        
        check_server_side_exports(&file_path, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_multiple_server_exports_detected() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-multiple-exports");
        fs::create_dir_all(&temp_dir).ok();
        
        let file_path = temp_dir.join("Component.tsx");
        let content = r#"
"use client"

export async function getServerSideProps() {}
export const getStaticProps = async () => {};
export function getStaticPaths() {}
"#;
        create_temp_file(&file_path, content);
        
        let config = get_test_config();
        let mut diagnostics = DiagnosticCollection::new();
        
        check_server_side_exports(&file_path, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 3);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_component_nesting_depth_within_limit() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-nesting-ok");
        fs::create_dir_all(&temp_dir).ok();
        
        let file_path = temp_dir.join("app/components/Button.tsx");
        create_temp_file(&file_path, "export function Button() {}");
        
        let config = get_test_config();
        let mut diagnostics = DiagnosticCollection::new();
        
        check_component_nesting_depth(&file_path, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_component_nesting_depth_exceeds_limit() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-nesting-deep");
        fs::create_dir_all(&temp_dir).ok();
        
        let file_path = temp_dir.join("app/components/ui/buttons/primary/Button.tsx");
        create_temp_file(&file_path, "export function Button() {}");
        
        let config = get_test_config();
        let mut diagnostics = DiagnosticCollection::new();
        
        check_component_nesting_depth(&file_path, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 1);
        assert_eq!(diagnostics.diagnostics[0].rule, "component-nesting-depth");
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_component_nesting_depth_custom_limit() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-nesting-custom");
        fs::create_dir_all(&temp_dir).ok();
        
        let file_path = temp_dir.join("app/components/ui/Button.tsx");
        create_temp_file(&file_path, "export function Button() {}");
        
        let mut config = get_test_config();
        config.rules.component_nesting_depth.options.max_nesting_depth = 2;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_component_nesting_depth(&file_path, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 1);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_filename_style_kebab_case() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-kebab");
        fs::create_dir_all(&temp_dir).ok();
        
        let good_file = temp_dir.join("my-component.tsx");
        create_temp_file(&good_file, "export function MyComponent() {}");
        
        let bad_file = temp_dir.join("MyComponent.tsx");
        create_temp_file(&bad_file, "export function MyComponent() {}");
        
        let mut config = get_test_config();
        config.rules.filename_style_consistency.options.filename_style = FilenameStyle::KebabCase;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_filename_style(&good_file, &config, &mut diagnostics);
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        let mut diagnostics = DiagnosticCollection::new();
        check_filename_style(&bad_file, &config, &mut diagnostics);
        assert_eq!(diagnostics.diagnostics.len(), 1);
        assert_eq!(diagnostics.diagnostics[0].rule, "filename-style-consistency");
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_filename_style_pascal_case() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-pascal");
        fs::create_dir_all(&temp_dir).ok();
        
        let good_file = temp_dir.join("MyComponent.tsx");
        create_temp_file(&good_file, "export function MyComponent() {}");
        
        let bad_file = temp_dir.join("my-component.tsx");
        create_temp_file(&bad_file, "export function MyComponent() {}");
        
        let mut config = get_test_config();
        config.rules.filename_style_consistency.options.filename_style = FilenameStyle::PascalCase;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_filename_style(&good_file, &config, &mut diagnostics);
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        let mut diagnostics = DiagnosticCollection::new();
        check_filename_style(&bad_file, &config, &mut diagnostics);
        assert_eq!(diagnostics.diagnostics.len(), 1);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_filename_style_camel_case() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-camel");
        fs::create_dir_all(&temp_dir).ok();
        
        let good_file = temp_dir.join("myComponent.tsx");
        create_temp_file(&good_file, "export function MyComponent() {}");
        
        let bad_file = temp_dir.join("MyComponent.tsx");
        create_temp_file(&bad_file, "export function MyComponent() {}");
        
        let mut config = get_test_config();
        config.rules.filename_style_consistency.options.filename_style = FilenameStyle::CamelCase;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_filename_style(&good_file, &config, &mut diagnostics);
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        let mut diagnostics = DiagnosticCollection::new();
        check_filename_style(&bad_file, &config, &mut diagnostics);
        assert_eq!(diagnostics.diagnostics.len(), 1);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_filename_style_snake_case() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-snake");
        fs::create_dir_all(&temp_dir).ok();
        
        let good_file = temp_dir.join("my_component.tsx");
        create_temp_file(&good_file, "export function MyComponent() {}");
        
        let bad_file = temp_dir.join("MyComponent.tsx");
        create_temp_file(&bad_file, "export function MyComponent() {}");
        
        let mut config = get_test_config();
        config.rules.filename_style_consistency.options.filename_style = FilenameStyle::SnakeCase;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_filename_style(&good_file, &config, &mut diagnostics);
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        let mut diagnostics = DiagnosticCollection::new();
        check_filename_style(&bad_file, &config, &mut diagnostics);
        assert_eq!(diagnostics.diagnostics.len(), 1);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_filename_special_files_skipped() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-special");
        fs::create_dir_all(&temp_dir).ok();
        
        let special_files = vec!["page.tsx", "layout.tsx", "loading.tsx", "error.tsx"];
        
        let mut config = get_test_config();
        config.rules.filename_style_consistency.options.filename_style = FilenameStyle::PascalCase;
        
        for filename in special_files {
            let file_path = temp_dir.join(filename);
            create_temp_file(&file_path, "export default function Page() {}");
            
            let mut diagnostics = DiagnosticCollection::new();
            check_filename_style(&file_path, &config, &mut diagnostics);
            assert_eq!(diagnostics.diagnostics.len(), 0, "Special file {} should be skipped", filename);
        }
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_missing_test_file_required() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-missing-test");
        fs::create_dir_all(&temp_dir).ok();
        
        let component_file = temp_dir.join("Button.tsx");
        create_temp_file(&component_file, "export function Button() {}");
        
        let mut config = get_test_config();
        config.rules.missing_companion_files.options.require_test_files = true;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_missing_companion_files(&component_file, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 1);
        assert!(diagnostics.diagnostics[0].message.contains("Missing test file"));
        assert_eq!(diagnostics.diagnostics[0].rule, "missing-companion-files");
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_test_file_exists() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-with-test");
        fs::create_dir_all(&temp_dir).ok();
        
        let component_file = temp_dir.join("Button.tsx");
        create_temp_file(&component_file, "export function Button() {}");
        
        let test_file = temp_dir.join("Button.test.tsx");
        create_temp_file(&test_file, "test('Button', () => {})");
        
        let mut config = get_test_config();
        config.rules.missing_companion_files.options.require_test_files = true;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_missing_companion_files(&component_file, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_spec_file_exists() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-with-spec");
        fs::create_dir_all(&temp_dir).ok();
        
        let component_file = temp_dir.join("Button.tsx");
        create_temp_file(&component_file, "export function Button() {}");
        
        let spec_file = temp_dir.join("Button.spec.tsx");
        create_temp_file(&spec_file, "describe('Button', () => {})");
        
        let mut config = get_test_config();
        config.rules.missing_companion_files.options.require_test_files = true;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_missing_companion_files(&component_file, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_missing_story_file_required() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-missing-story");
        fs::create_dir_all(&temp_dir).ok();
        
        let component_file = temp_dir.join("Button.tsx");
        create_temp_file(&component_file, "export function Button() {}");
        
        let mut config = get_test_config();
        config.rules.missing_companion_files.options.require_story_files = true;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_missing_companion_files(&component_file, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 1);
        assert!(diagnostics.diagnostics[0].message.contains("Missing story file"));
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_story_file_exists() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-with-story");
        fs::create_dir_all(&temp_dir).ok();
        
        let component_file = temp_dir.join("Button.tsx");
        create_temp_file(&component_file, "export function Button() {}");
        
        let story_file = temp_dir.join("Button.stories.tsx");
        create_temp_file(&story_file, "export default { component: Button }");
        
        let mut config = get_test_config();
        config.rules.missing_companion_files.options.require_story_files = true;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_missing_companion_files(&component_file, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_missing_both_test_and_story() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-missing-both");
        fs::create_dir_all(&temp_dir).ok();
        
        let component_file = temp_dir.join("Button.tsx");
        create_temp_file(&component_file, "export function Button() {}");
        
        let mut config = get_test_config();
        config.rules.missing_companion_files.options.require_test_files = true;
        config.rules.missing_companion_files.options.require_story_files = true;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_missing_companion_files(&component_file, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 2);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_non_component_files_skipped() {
        let temp_dir = std::env::temp_dir().join("naechste-tests-non-component");
        fs::create_dir_all(&temp_dir).ok();
        
        let ts_file = temp_dir.join("utils.ts");
        create_temp_file(&ts_file, "export function helper() {}");
        
        let mut config = get_test_config();
        config.rules.missing_companion_files.options.require_test_files = true;
        
        let mut diagnostics = DiagnosticCollection::new();
        check_missing_companion_files(&ts_file, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }
}
