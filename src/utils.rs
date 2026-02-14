use glob::Pattern;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Check if a file path matches a glob pattern
pub fn matches_glob(path: &Path, pattern: &str, base_path: &Path) -> bool {
    // Make path relative to base for matching
    let relative_path = if let Ok(rel) = path.strip_prefix(base_path) {
        rel
    } else {
        path
    };
    
    if let Ok(glob_pattern) = Pattern::new(pattern) {
        let path_str = relative_path.to_str().unwrap_or("");
        glob_pattern.matches(path_str) || glob_pattern.matches(&format!("/{}", path_str))
    } else {
        false
    }
}

/// Check if file path should be excluded based on exclude patterns
pub fn is_excluded(path: &Path, exclude_patterns: &[String], base_path: &Path) -> bool {
    exclude_patterns.iter().any(|pattern| {
        matches_glob(path, pattern, base_path)
    })
}

/// Find all files in a directory that match a sibling glob pattern
pub fn find_sibling_by_glob(dir: &Path, glob_pattern: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    
    if let Ok(pattern) = Pattern::new(glob_pattern) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if pattern.matches(&file_name) {
                        matches.push(entry.path());
                    }
                }
            }
        }
    }
    
    matches
}

/// Extract import specifiers from a file
/// Returns a list of import paths found in the file
pub fn extract_imports(file_path: &Path) -> Vec<String> {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    
    let mut imports = Vec::new();
    
    // Match: import ... from '...' or import ... from "..."
    let import_re = Regex::new(r#"import\s+.*?\s+from\s+['"]([^'"]+)['"]"#).unwrap();
    for cap in import_re.captures_iter(&content) {
        imports.push(cap[1].to_string());
    }
    
    // Match: require('...') or require("...")
    let require_re = Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
    for cap in require_re.captures_iter(&content) {
        imports.push(cap[1].to_string());
    }
    
    // Match: export ... from '...' or export ... from "..."
    let export_re = Regex::new(r#"export\s+.*?\s+from\s+['"]([^'"]+)['"]"#).unwrap();
    for cap in export_re.captures_iter(&content) {
        imports.push(cap[1].to_string());
    }
    
    imports
}

/// Resolve an import specifier to a potential file path
/// Handles relative imports (./foo, ../bar) and alias imports (@/foo)
pub fn resolve_import_path(
    import_specifier: &str,
    importer_file: &Path,
    project_root: &Path,
) -> Option<PathBuf> {
    // Handle alias imports (@/...)
    if import_specifier.starts_with("@/") {
        let relative_path = import_specifier.strip_prefix("@/")?;
        return Some(project_root.join(relative_path));
    }
    
    // Handle relative imports (./ or ../)
    if import_specifier.starts_with("./") || import_specifier.starts_with("../") {
        let importer_dir = importer_file.parent()?;
        let target = importer_dir.join(import_specifier);
        return Some(target);
    }
    
    // For non-relative, non-alias imports (node_modules), return None
    None
}

/// Try to find the actual file for an import path (handles extensions and index files)
pub fn resolve_to_actual_file(base_path: &Path) -> Option<PathBuf> {
    // Extensions to try
    let extensions = ["", ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs"];
    
    // First try with extensions directly
    for ext in &extensions {
        let candidate = PathBuf::from(format!("{}{}", base_path.display(), ext));
        if candidate.exists() && candidate.is_file() {
            return Some(candidate);
        }
    }
    
    // Try as directory with index files
    if base_path.is_dir() {
        for ext in &[".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs"] {
            let candidate = base_path.join(format!("index{}", ext));
            if candidate.exists() && candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    
    None
}

/// Build an import index: maps target files to list of importer files
pub fn build_import_index(
    files: &[PathBuf],
    project_root: &Path,
) -> HashMap<PathBuf, Vec<PathBuf>> {
    let mut index: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    
    for importer in files {
        let imports = extract_imports(importer);
        
        for import_spec in imports {
            if let Some(resolved) = resolve_import_path(&import_spec, importer, project_root) {
                if let Some(actual_file) = resolve_to_actual_file(&resolved) {
                    // Normalize paths for comparison
                    let normalized = actual_file.canonicalize().unwrap_or(actual_file);
                    index.entry(normalized).or_insert_with(Vec::new).push(importer.clone());
                }
            }
        }
    }
    
    index
}

/// Check if a path is under any of the allowed prefixes
pub fn is_under_any_prefix(path: &Path, prefixes: &[String], base_path: &Path) -> bool {
    let relative_path = if let Ok(rel) = path.strip_prefix(base_path) {
        rel
    } else {
        path
    };
    
    let path_str = relative_path.to_str().unwrap_or("");
    
    prefixes.iter().any(|prefix| {
        let normalized_prefix = prefix.trim_start_matches('/').trim_end_matches('/');
        path_str.starts_with(normalized_prefix) || path_str.starts_with(&format!("{}/", normalized_prefix))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    fn create_temp_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_matches_glob_simple() {
        let base = Path::new("/project");
        let path = Path::new("/project/app/page.tsx");
        assert!(matches_glob(path, "**/page.tsx", base));
        assert!(matches_glob(path, "app/page.tsx", base));
        assert!(!matches_glob(path, "**/layout.tsx", base));
    }

    #[test]
    fn test_matches_glob_with_wildcard() {
        let base = Path::new("/project");
        let path = Path::new("/project/components/Button.tsx");
        assert!(matches_glob(path, "**/*.tsx", base));
        assert!(matches_glob(path, "components/*.tsx", base));
        assert!(!matches_glob(path, "app/*.tsx", base));
    }

    #[test]
    fn test_is_excluded() {
        let base = Path::new("/project");
        let path = Path::new("/project/app/page.tsx");
        let excludes = vec!["**/page.tsx".to_string(), "**/layout.tsx".to_string()];
        assert!(is_excluded(path, &excludes, base));
        
        let path2 = Path::new("/project/components/Button.tsx");
        assert!(!is_excluded(path2, &excludes, base));
    }

    #[test]
    fn test_find_sibling_by_glob() {
        let temp_dir = std::env::temp_dir().join("naechste-test-sibling");
        fs::create_dir_all(&temp_dir).ok();
        
        create_temp_file(&temp_dir.join("Button.tsx"), "export const Button = () => {}");
        create_temp_file(&temp_dir.join("Button.stories.tsx"), "export default {}");
        create_temp_file(&temp_dir.join("Button.test.tsx"), "test('button', () => {})");
        create_temp_file(&temp_dir.join("other.tsx"), "export const Other = () => {}");
        
        let matches = find_sibling_by_glob(&temp_dir, "*.stories.tsx");
        assert_eq!(matches.len(), 1);
        assert!(matches[0].ends_with("Button.stories.tsx"));
        
        let test_matches = find_sibling_by_glob(&temp_dir, "*.test.tsx");
        assert_eq!(test_matches.len(), 1);
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_extract_imports_from_statements() {
        let temp_dir = std::env::temp_dir().join("naechste-test-imports");
        fs::create_dir_all(&temp_dir).ok();
        
        let file_path = temp_dir.join("test.tsx");
        let content = r#"
import { Button } from './Button';
import React from 'react';
import { Utils } from '@/lib/utils';
const fs = require('fs');
export { Helper } from '../helpers/helper';
"#;
        create_temp_file(&file_path, content);
        
        let imports = extract_imports(&file_path);
        assert_eq!(imports.len(), 5);
        assert!(imports.contains(&"./Button".to_string()));
        assert!(imports.contains(&"react".to_string()));
        assert!(imports.contains(&"@/lib/utils".to_string()));
        assert!(imports.contains(&"fs".to_string()));
        assert!(imports.contains(&"../helpers/helper".to_string()));
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_resolve_import_path_relative() {
        let importer = Path::new("/project/app/page.tsx");
        let root = Path::new("/project");
        
        let resolved = resolve_import_path("./Button", importer, root);
        assert!(resolved.is_some());
        assert!(resolved.unwrap().to_str().unwrap().contains("app"));
        
        let resolved2 = resolve_import_path("../components/Header", importer, root);
        assert!(resolved2.is_some());
        assert!(resolved2.unwrap().to_str().unwrap().contains("components"));
    }

    #[test]
    fn test_resolve_import_path_alias() {
        let importer = Path::new("/project/app/page.tsx");
        let root = Path::new("/project");
        
        let resolved = resolve_import_path("@/components/Button", importer, root);
        assert_eq!(resolved, Some(PathBuf::from("/project/components/Button")));
    }

    #[test]
    fn test_resolve_import_path_node_modules() {
        let importer = Path::new("/project/app/page.tsx");
        let root = Path::new("/project");
        
        let resolved = resolve_import_path("react", importer, root);
        assert_eq!(resolved, None);
    }

    #[test]
    fn test_resolve_to_actual_file() {
        let temp_dir = std::env::temp_dir().join("naechste-test-resolve");
        fs::create_dir_all(&temp_dir).ok();
        
        create_temp_file(&temp_dir.join("Button.tsx"), "export const Button = () => {}");
        
        let resolved = resolve_to_actual_file(&temp_dir.join("Button"));
        assert!(resolved.is_some());
        assert!(resolved.unwrap().ends_with("Button.tsx"));
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_resolve_to_actual_file_with_index() {
        let temp_dir = std::env::temp_dir().join("naechste-test-resolve-index");
        let components_dir = temp_dir.join("components");
        fs::create_dir_all(&components_dir).ok();
        
        create_temp_file(&components_dir.join("index.tsx"), "export * from './Button'");
        
        let resolved = resolve_to_actual_file(&components_dir);
        assert!(resolved.is_some());
        assert!(resolved.unwrap().ends_with("index.tsx"));
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_is_under_any_prefix() {
        let base = Path::new("/project");
        let path = Path::new("/project/components/ui/Button.tsx");
        
        let prefixes = vec!["components/ui".to_string(), "app/components".to_string()];
        assert!(is_under_any_prefix(path, &prefixes, base));
        
        let prefixes2 = vec!["app".to_string(), "lib".to_string()];
        assert!(!is_under_any_prefix(path, &prefixes2, base));
    }

    #[test]
    fn test_is_under_any_prefix_with_slashes() {
        let base = Path::new("/project");
        let path = Path::new("/project/app/components/Button.tsx");
        
        let prefixes = vec!["/app/".to_string()];
        assert!(is_under_any_prefix(path, &prefixes, base));
    }
}
