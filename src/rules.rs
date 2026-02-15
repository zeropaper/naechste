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

/// Check file organization rules
pub fn check_file_organization(
    project_root: &Path,
    all_files: &[std::path::PathBuf],
    config: &Config,
    diagnostics: &mut DiagnosticCollection,
) {
    use crate::config::{RequireKind};
    use crate::utils;
    use regex::Regex;
    use std::collections::HashMap;
    
    let checks = &config.rules.file_organization.options.file_organization_checks;
    
    if checks.is_empty() {
        return;
    }
    
    // Build import index for when_imported_by checks
    let import_index = utils::build_import_index(all_files, project_root);
    
    // Pre-compile regex patterns for all checks
    let mut compiled_patterns: HashMap<String, Vec<Regex>> = HashMap::new();
    for check in checks {
        if let Some(when_imported) = &check.when_imported_by {
            let mut patterns = Vec::new();
            for pattern_str in &when_imported.import_path_matches {
                if let Ok(pattern) = Regex::new(pattern_str) {
                    patterns.push(pattern);
                }
            }
            compiled_patterns.insert(check.id.clone(), patterns);
        }
    }
    
    // Process each check
    for check in checks {
        // Find files matching the pattern
        for file in all_files {
            // Check if file matches the glob pattern
            if !utils::matches_glob(file, &check.r#match.glob, project_root) {
                continue;
            }
            
            // Check if file is excluded
            if utils::is_excluded(file, &check.r#match.exclude_glob, project_root) {
                continue;
            }
            
            // Check require conditions (sibling files)
            for require in &check.require {
                match require {
                    RequireKind::SiblingExact { name } => {
                        if let Some(parent) = file.parent() {
                            let sibling_path = parent.join(name);
                            if !sibling_path.exists() {
                                diagnostics.add(Diagnostic {
                                    severity: config.rules.file_organization.severity,
                                    rule: format!("file-organization:{}", check.id),
                                    message: format!(
                                        "Missing required companion file '{}' next to '{}'",
                                        name,
                                        file.display()
                                    ),
                                    file: file.clone(),
                                    line: None,
                                });
                            }
                        }
                    }
                    RequireKind::SiblingGlob { glob } => {
                        if let Some(parent) = file.parent() {
                            let siblings = utils::find_sibling_by_glob(parent, glob);
                            if siblings.is_empty() {
                                diagnostics.add(Diagnostic {
                                    severity: config.rules.file_organization.severity,
                                    rule: format!("file-organization:{}", check.id),
                                    message: format!(
                                        "Missing required companion file matching '{}' next to '{}'",
                                        glob,
                                        file.display()
                                    ),
                                    file: file.clone(),
                                    line: None,
                                });
                            }
                        }
                    }
                }
            }
            
            // Check when_imported_by and enforce_location
            if let (Some(when_imported), Some(enforce_loc)) = (&check.when_imported_by, &check.enforce_location) {
                // Get files that import this file
                let normalized_file = file.canonicalize().unwrap_or_else(|_| file.clone());
                if let Some(importers) = import_index.get(&normalized_file) {
                    // Check if any importer matches the importer_glob
                    for importer in importers {
                        if !utils::matches_glob(importer, &when_imported.importer_glob, project_root) {
                            continue;
                        }
                        
                        // Get import specifiers from this importer
                        let import_specs = utils::extract_imports(importer);
                        
                        // Check if any import specifier matches the patterns (using pre-compiled regexes)
                        let mut matches_import_pattern = false;
                        if let Some(patterns) = compiled_patterns.get(&check.id) {
                            for spec in &import_specs {
                                for pattern in patterns {
                                    if pattern.is_match(spec) {
                                        matches_import_pattern = true;
                                        break;
                                    }
                                }
                                if matches_import_pattern {
                                    break;
                                }
                            }
                        }
                        
                        if matches_import_pattern {
                            // Check if file is under required location
                            if !utils::is_under_any_prefix(file, &enforce_loc.must_be_under, project_root) {
                                let msg = enforce_loc.message.clone().unwrap_or_else(|| {
                                    format!(
                                        "File is imported by '{}' but is not located under any of: {}",
                                        importer.display(),
                                        enforce_loc.must_be_under.join(", ")
                                    )
                                });
                                
                                diagnostics.add(Diagnostic {
                                    severity: config.rules.file_organization.severity,
                                    rule: format!("file-organization:{}", check.id),
                                    message: msg,
                                    file: file.clone(),
                                    line: None,
                                });
                                break; // Only report once per file
                            }
                        }
                    }
                }
            }
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
    fn test_file_organization_sibling_exact_missing() {
        use crate::config::{OrganizationCheck, MatchPattern, RequireKind};
        
        let temp_dir = std::env::temp_dir().join("naechste-tests-file-org-sibling-exact");
        fs::create_dir_all(&temp_dir).ok();
        
        let page_file = temp_dir.join("page.tsx");
        create_temp_file(&page_file, "export default function Page() {}");
        
        let mut config = get_test_config();
        config.rules.file_organization.severity = crate::config::Severity::Error;
        config.rules.file_organization.options.file_organization_checks = vec![
            OrganizationCheck {
                id: "page-needs-user-story".to_string(),
                description: Some("Every page.tsx must have a User-Story.us.md".to_string()),
                r#match: MatchPattern {
                    glob: "**/page.tsx".to_string(),
                    exclude_glob: vec![],
                },
                require: vec![
                    RequireKind::SiblingExact { name: "User-Story.us.md".to_string() }
                ],
                when_imported_by: None,
                enforce_location: None,
            }
        ];
        
        let all_files = vec![page_file.clone()];
        let mut diagnostics = DiagnosticCollection::new();
        check_file_organization(&temp_dir, &all_files, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 1);
        assert!(diagnostics.diagnostics[0].message.contains("User-Story.us.md"));
        assert!(diagnostics.diagnostics[0].rule.contains("page-needs-user-story"));
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_file_organization_sibling_exact_exists() {
        use crate::config::{OrganizationCheck, MatchPattern, RequireKind};
        
        let temp_dir = std::env::temp_dir().join("naechste-tests-file-org-sibling-exists");
        fs::create_dir_all(&temp_dir).ok();
        
        let page_file = temp_dir.join("page.tsx");
        create_temp_file(&page_file, "export default function Page() {}");
        
        let us_file = temp_dir.join("User-Story.us.md");
        create_temp_file(&us_file, "# User Story");
        
        let mut config = get_test_config();
        config.rules.file_organization.options.file_organization_checks = vec![
            OrganizationCheck {
                id: "page-needs-user-story".to_string(),
                description: None,
                r#match: MatchPattern {
                    glob: "**/page.tsx".to_string(),
                    exclude_glob: vec![],
                },
                require: vec![
                    RequireKind::SiblingExact { name: "User-Story.us.md".to_string() }
                ],
                when_imported_by: None,
                enforce_location: None,
            }
        ];
        
        let all_files = vec![page_file.clone()];
        let mut diagnostics = DiagnosticCollection::new();
        check_file_organization(&temp_dir, &all_files, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_file_organization_sibling_glob_missing() {
        use crate::config::{OrganizationCheck, MatchPattern, RequireKind};
        
        let temp_dir = std::env::temp_dir().join("naechste-tests-file-org-glob-missing");
        fs::create_dir_all(&temp_dir).ok();
        
        let button_file = temp_dir.join("Button.tsx");
        create_temp_file(&button_file, "export const Button = () => {}");
        
        let mut config = get_test_config();
        config.rules.file_organization.options.file_organization_checks = vec![
            OrganizationCheck {
                id: "component-needs-stories".to_string(),
                description: None,
                r#match: MatchPattern {
                    glob: "**/*.tsx".to_string(),
                    exclude_glob: vec!["**/page.tsx".to_string()],
                },
                require: vec![
                    RequireKind::SiblingGlob { glob: "*.stories.tsx".to_string() }
                ],
                when_imported_by: None,
                enforce_location: None,
            }
        ];
        
        let all_files = vec![button_file.clone()];
        let mut diagnostics = DiagnosticCollection::new();
        check_file_organization(&temp_dir, &all_files, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 1);
        assert!(diagnostics.diagnostics[0].message.contains("*.stories.tsx"));
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_file_organization_sibling_glob_exists() {
        use crate::config::{OrganizationCheck, MatchPattern, RequireKind};
        
        let temp_dir = std::env::temp_dir().join("naechste-tests-file-org-glob-exists");
        fs::create_dir_all(&temp_dir).ok();
        
        let button_file = temp_dir.join("Button.tsx");
        create_temp_file(&button_file, "export const Button = () => {}");
        
        let story_file = temp_dir.join("Button.stories.tsx");
        create_temp_file(&story_file, "export default {}");
        
        let mut config = get_test_config();
        config.rules.file_organization.options.file_organization_checks = vec![
            OrganizationCheck {
                id: "component-needs-stories".to_string(),
                description: None,
                r#match: MatchPattern {
                    glob: "**/*.tsx".to_string(),
                    exclude_glob: vec![],
                },
                require: vec![
                    RequireKind::SiblingGlob { glob: "*.stories.tsx".to_string() }
                ],
                when_imported_by: None,
                enforce_location: None,
            }
        ];
        
        let all_files = vec![button_file.clone()];
        let mut diagnostics = DiagnosticCollection::new();
        check_file_organization(&temp_dir, &all_files, &config, &mut diagnostics);
        
        assert_eq!(diagnostics.diagnostics.len(), 0);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_file_organization_exclude_glob() {
        use crate::config::{OrganizationCheck, MatchPattern, RequireKind};
        
        let temp_dir = std::env::temp_dir().join("naechste-tests-file-org-exclude");
        fs::create_dir_all(&temp_dir).ok();
        
        let page_file = temp_dir.join("page.tsx");
        create_temp_file(&page_file, "export default function Page() {}");
        
        let button_file = temp_dir.join("Button.tsx");
        create_temp_file(&button_file, "export const Button = () => {}");
        
        let mut config = get_test_config();
        config.rules.file_organization.options.file_organization_checks = vec![
            OrganizationCheck {
                id: "component-needs-stories".to_string(),
                description: None,
                r#match: MatchPattern {
                    glob: "**/*.tsx".to_string(),
                    exclude_glob: vec!["**/page.tsx".to_string()],
                },
                require: vec![
                    RequireKind::SiblingGlob { glob: "*.stories.tsx".to_string() }
                ],
                when_imported_by: None,
                enforce_location: None,
            }
        ];
        
        let all_files = vec![page_file.clone(), button_file.clone()];
        let mut diagnostics = DiagnosticCollection::new();
        check_file_organization(&temp_dir, &all_files, &config, &mut diagnostics);
        
        // Only Button.tsx should be checked (page.tsx is excluded)
        assert_eq!(diagnostics.diagnostics.len(), 1);
        assert!(diagnostics.diagnostics[0].file.to_str().unwrap().contains("Button.tsx"));
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_file_organization_location_enforcement() {
        use crate::config::{OrganizationCheck, MatchPattern, WhenImportedBy, EnforceLocation};
        
        let temp_dir = std::env::temp_dir().join("naechste-tests-file-org-location");
        fs::create_dir_all(&temp_dir).ok();
        
        // Create a UI component in the wrong location
        let wrong_location = temp_dir.join("lib");
        fs::create_dir_all(&wrong_location).ok();
        let button_file = wrong_location.join("Button.tsx");
        create_temp_file(&button_file, "export const Button = () => {}");
        
        // Create an app file that imports it
        let app_dir = temp_dir.join("app");
        fs::create_dir_all(&app_dir).ok();
        let page_file = app_dir.join("page.tsx");
        create_temp_file(&page_file, "import { Button } from '@/lib/Button';");
        
        let mut config = get_test_config();
        config.rules.file_organization.options.file_organization_checks = vec![
            OrganizationCheck {
                id: "ui-must-live-in-components".to_string(),
                description: None,
                r#match: MatchPattern {
                    glob: "**/*.tsx".to_string(),
                    exclude_glob: vec![],
                },
                require: vec![],
                when_imported_by: Some(WhenImportedBy {
                    importer_glob: "app/**".to_string(),
                    import_path_matches: vec!["^@/lib/".to_string()],
                }),
                enforce_location: Some(EnforceLocation {
                    must_be_under: vec!["components".to_string()],
                    message: Some("UI components must live under components/".to_string()),
                }),
            }
        ];
        
        let all_files = vec![button_file.clone(), page_file.clone()];
        let mut diagnostics = DiagnosticCollection::new();
        check_file_organization(&temp_dir, &all_files, &config, &mut diagnostics);
        
        // Should report that Button.tsx is in the wrong location
        assert_eq!(diagnostics.diagnostics.len(), 1);
        assert!(diagnostics.diagnostics[0].message.contains("UI components must live under components/"));
        
        fs::remove_dir_all(&temp_dir).ok();
    }
}

