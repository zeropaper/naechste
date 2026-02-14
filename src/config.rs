use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub rules: Rules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules {
    #[serde(default = "default_rule_config")]
    pub server_side_exports: RuleConfig,

    #[serde(default = "default_rule_config")]
    pub component_nesting_depth: RuleConfig,

    #[serde(default = "default_rule_config")]
    pub filename_style_consistency: RuleConfig,

    #[serde(default = "default_rule_config")]
    pub missing_companion_files: RuleConfig,
    
    #[serde(default = "default_rule_config")]
    pub file_organization: RuleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    #[serde(default = "default_severity")]
    pub severity: Severity,

    #[serde(default)]
    pub options: RuleOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOptions {
    #[serde(default = "default_max_depth")]
    pub max_nesting_depth: usize,

    #[serde(default = "default_filename_style")]
    pub filename_style: FilenameStyle,

    #[serde(default)]
    pub require_test_files: bool,

    #[serde(default)]
    pub require_story_files: bool,

    /// Custom companion file patterns for additional checks
    #[serde(default)]
    pub companion_file_patterns: CompanionFilePatterns,
    
    /// File organization checks
    #[serde(default)]
    pub file_organization_checks: Vec<OrganizationCheck>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FilenameStyle {
    KebabCase,
    CamelCase,
    PascalCase,
    SnakeCase,
}

/// Custom companion file patterns configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompanionFilePatterns {
    /// Integration test patterns like ["*.test.int.ts", "*.test.int.tsx"]
    #[serde(default)]
    pub integration_tests: Vec<String>,

    /// Page user scenario patterns like ["page.us.md"]
    #[serde(default)]
    pub page_user_scenarios: Vec<String>,

    /// Custom companion file patterns (key = category name, value = list of glob patterns)
    #[serde(default)]
    pub custom: std::collections::HashMap<String, Vec<String>>,
}

/// File organization check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationCheck {
    /// Unique identifier for this check
    pub id: String,
    
    /// Optional description of what this check does
    #[serde(default)]
    pub description: Option<String>,
    
    /// Pattern to match files this check applies to
    pub r#match: MatchPattern,
    
    /// Requirements for companion files
    #[serde(default)]
    pub require: Vec<RequireKind>,
    
    /// When imported by condition
    #[serde(default)]
    pub when_imported_by: Option<WhenImportedBy>,
    
    /// Location enforcement
    #[serde(default)]
    pub enforce_location: Option<EnforceLocation>,
}

/// Pattern for matching files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPattern {
    /// Glob pattern to match files
    pub glob: String,
    
    /// Optional glob patterns to exclude
    #[serde(default)]
    pub exclude_glob: Vec<String>,
}

/// Kind of companion file requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum RequireKind {
    /// Exact sibling file name
    #[serde(rename = "sibling_exact")]
    SiblingExact { name: String },
    
    /// Sibling file matching glob
    #[serde(rename = "sibling_glob")]
    SiblingGlob { glob: String },
}

/// Condition for when a file is imported by another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhenImportedBy {
    /// Glob pattern for importer files
    pub importer_glob: String,
    
    /// Regex patterns to match import specifiers
    pub import_path_matches: Vec<String>,
}

/// Location enforcement rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforceLocation {
    /// List of allowed directory prefixes
    pub must_be_under: Vec<String>,
    
    /// Optional custom message
    #[serde(default)]
    pub message: Option<String>,
}

fn default_rule_config() -> RuleConfig {
    RuleConfig {
        severity: Severity::Warn,
        options: RuleOptions::default(),
    }
}

fn default_severity() -> Severity {
    Severity::Warn
}

fn default_max_depth() -> usize {
    3
}

fn default_filename_style() -> FilenameStyle {
    FilenameStyle::KebabCase
}

impl Default for Config {
    fn default() -> Self {
        Config {
            rules: Rules::default(),
        }
    }
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            server_side_exports: default_rule_config(),
            component_nesting_depth: default_rule_config(),
            filename_style_consistency: default_rule_config(),
            missing_companion_files: default_rule_config(),
            file_organization: default_rule_config(),
        }
    }
}

impl Default for RuleOptions {
    fn default() -> Self {
        RuleOptions {
            max_nesting_depth: default_max_depth(),
            filename_style: default_filename_style(),
            require_test_files: false,
            require_story_files: false,
            companion_file_patterns: CompanionFilePatterns::default(),
            file_organization_checks: Vec::new(),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        let config: Config = match extension.as_str() {
            "yaml" | "yml" => serde_yaml::from_str(&contents)?,
            "jsonc" => json5::from_str(&contents)?,
            // Attempt strict JSON first, then fall back to JSON5 to allow comments
            "json" | "" => {
                serde_json::from_str(&contents).or_else(|_| json5::from_str(&contents))?
            }
            // Unknown extension: try JSON, then JSON5 (JSON with comments), then YAML
            _ => serde_json::from_str(&contents)
                .or_else(|_| json5::from_str(&contents))
                .or_else(|_| serde_yaml::from_str(&contents))?,
        };
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(matches!(
            config.rules.server_side_exports.severity,
            Severity::Warn
        ));
        assert!(matches!(
            config.rules.component_nesting_depth.severity,
            Severity::Warn
        ));
        assert!(matches!(
            config.rules.filename_style_consistency.severity,
            Severity::Warn
        ));
        assert!(matches!(
            config.rules.missing_companion_files.severity,
            Severity::Warn
        ));
    }

    #[test]
    fn test_default_rule_options() {
        let options = RuleOptions::default();
        assert_eq!(options.max_nesting_depth, 3);
        assert!(matches!(options.filename_style, FilenameStyle::KebabCase));
        assert!(!options.require_test_files);
        assert!(!options.require_story_files);
    }

    #[test]
    fn test_severity_serialization() {
        let warn = serde_json::to_string(&Severity::Warn).unwrap();
        assert_eq!(warn, "\"warn\"");

        let error = serde_json::to_string(&Severity::Error).unwrap();
        assert_eq!(error, "\"error\"");
    }

    #[test]
    fn test_filename_style_serialization() {
        let kebab = serde_json::to_string(&FilenameStyle::KebabCase).unwrap();
        assert_eq!(kebab, "\"kebab-case\"");

        let camel = serde_json::to_string(&FilenameStyle::CamelCase).unwrap();
        assert_eq!(camel, "\"camel-case\"");

        let pascal = serde_json::to_string(&FilenameStyle::PascalCase).unwrap();
        assert_eq!(pascal, "\"pascal-case\"");

        let snake = serde_json::to_string(&FilenameStyle::SnakeCase).unwrap();
        assert_eq!(snake, "\"snake-case\"");
    }

    #[test]
    fn test_config_load_from_file() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test-config.json");

        let config_json = r#"{
            "rules": {
                "server_side_exports": {
                    "severity": "error"
                },
                "component_nesting_depth": {
                    "severity": "warn",
                    "options": {
                        "max_nesting_depth": 5
                    }
                },
                "filename_style_consistency": {
                    "severity": "error",
                    "options": {
                        "filename_style": "pascal-case"
                    }
                },
                "missing_companion_files": {
                    "severity": "warn",
                    "options": {
                        "require_test_files": true,
                        "require_story_files": true
                    }
                }
            }
        }"#;

        let mut file = File::create(&config_path).unwrap();
        file.write_all(config_json.as_bytes()).unwrap();

        let config = Config::load(&config_path).unwrap();

        assert!(matches!(
            config.rules.server_side_exports.severity,
            Severity::Error
        ));
        assert!(matches!(
            config.rules.component_nesting_depth.severity,
            Severity::Warn
        ));
        assert_eq!(
            config
                .rules
                .component_nesting_depth
                .options
                .max_nesting_depth,
            5
        );
        assert!(matches!(
            config.rules.filename_style_consistency.severity,
            Severity::Error
        ));
        assert!(matches!(
            config
                .rules
                .filename_style_consistency
                .options
                .filename_style,
            FilenameStyle::PascalCase
        ));
        assert!(
            config
                .rules
                .missing_companion_files
                .options
                .require_test_files
        );
        assert!(
            config
                .rules
                .missing_companion_files
                .options
                .require_story_files
        );

        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_partial_config_uses_defaults() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test-partial-config.json");

        let config_json = r#"{
            "rules": {
                "server_side_exports": {
                    "severity": "error"
                }
            }
        }"#;

        let mut file = File::create(&config_path).unwrap();
        file.write_all(config_json.as_bytes()).unwrap();

        let config = Config::load(&config_path).unwrap();

        assert!(matches!(
            config.rules.server_side_exports.severity,
            Severity::Error
        ));
        assert!(matches!(
            config.rules.component_nesting_depth.severity,
            Severity::Warn
        ));

        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_invalid_config_file() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test-invalid-config.json");

        let mut file = File::create(&config_path).unwrap();
        file.write_all(b"invalid json {").unwrap();

        let result = Config::load(&config_path);
        assert!(result.is_err());

        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_config_loads_jsonc_with_comments() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test-config.jsonc");

        let config_jsonc = r#"{
            // comment allowed in jsonc
            "rules": {
                "server_side_exports": {
                    "severity": "error"
                }
            }
        }"#;

        let mut file = File::create(&config_path).unwrap();
        file.write_all(config_jsonc.as_bytes()).unwrap();

        let config = Config::load(&config_path).unwrap();

        assert!(matches!(
            config.rules.server_side_exports.severity,
            Severity::Error
        ));

        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_config_loads_yaml() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test-config.yaml");

        let config_yaml = r#"
rules:
  missing_companion_files:
    severity: error
    options:
      require_test_files: true
"#;

        let mut file = File::create(&config_path).unwrap();
        file.write_all(config_yaml.as_bytes()).unwrap();

        let config = Config::load(&config_path).unwrap();

        assert!(matches!(
            config.rules.missing_companion_files.severity,
            Severity::Error
        ));
        assert!(
            config
                .rules
                .missing_companion_files
                .options
                .require_test_files
        );

        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_nonexistent_config_file() {
        let config_path = std::env::temp_dir().join("nonexistent-config.json");
        let result = Config::load(&config_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_organization_config_parsing() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test-file-org-config.json");
        
        let config_json = r#"{
            "rules": {
                "file_organization": {
                    "severity": "error",
                    "options": {
                        "file_organization_checks": [
                            {
                                "id": "page-needs-user-story",
                                "description": "Every page.tsx must have a User-Story.us.md",
                                "match": {
                                    "glob": "**/page.tsx"
                                },
                                "require": [
                                    {
                                        "kind": "sibling_exact",
                                        "name": "User-Story.us.md"
                                    }
                                ]
                            },
                            {
                                "id": "component-needs-stories",
                                "match": {
                                    "glob": "**/*.tsx",
                                    "exclude_glob": ["**/page.tsx", "**/layout.tsx"]
                                },
                                "require": [
                                    {
                                        "kind": "sibling_glob",
                                        "glob": "*.stories.tsx"
                                    }
                                ]
                            }
                        ]
                    }
                }
            }
        }"#;
        
        let mut file = File::create(&config_path).unwrap();
        file.write_all(config_json.as_bytes()).unwrap();
        
        let config = Config::load(&config_path).unwrap();
        
        assert!(matches!(config.rules.file_organization.severity, Severity::Error));
        assert_eq!(config.rules.file_organization.options.file_organization_checks.len(), 2);
        
        let check1 = &config.rules.file_organization.options.file_organization_checks[0];
        assert_eq!(check1.id, "page-needs-user-story");
        assert_eq!(check1.r#match.glob, "**/page.tsx");
        assert_eq!(check1.require.len(), 1);
        
        let check2 = &config.rules.file_organization.options.file_organization_checks[1];
        assert_eq!(check2.id, "component-needs-stories");
        assert_eq!(check2.r#match.exclude_glob.len(), 2);
        
        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_file_organization_with_location_enforcement() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test-file-org-loc-config.json");
        
        let config_json = r#"{
            "rules": {
                "file_organization": {
                    "severity": "warn",
                    "options": {
                        "file_organization_checks": [
                            {
                                "id": "ui-must-live-in-components",
                                "match": {
                                    "glob": "**/*.tsx"
                                },
                                "when_imported_by": {
                                    "importer_glob": "app/**",
                                    "import_path_matches": ["^@/components/ui/"]
                                },
                                "enforce_location": {
                                    "must_be_under": ["components/ui", "app/components/ui"],
                                    "message": "UI components must live under components/ui"
                                }
                            }
                        ]
                    }
                }
            }
        }"#;
        
        let mut file = File::create(&config_path).unwrap();
        file.write_all(config_json.as_bytes()).unwrap();
        
        let config = Config::load(&config_path).unwrap();
        
        assert!(matches!(config.rules.file_organization.severity, Severity::Warn));
        assert_eq!(config.rules.file_organization.options.file_organization_checks.len(), 1);
        
        let check = &config.rules.file_organization.options.file_organization_checks[0];
        assert!(check.when_imported_by.is_some());
        assert!(check.enforce_location.is_some());
        
        let when_imported = check.when_imported_by.as_ref().unwrap();
        assert_eq!(when_imported.importer_glob, "app/**");
        assert_eq!(when_imported.import_path_matches.len(), 1);
        
        let enforce_loc = check.enforce_location.as_ref().unwrap();
        assert_eq!(enforce_loc.must_be_under.len(), 2);
        assert_eq!(enforce_loc.message.as_ref().unwrap(), "UI components must live under components/ui");
        
        std::fs::remove_file(config_path).ok();
    }
}

