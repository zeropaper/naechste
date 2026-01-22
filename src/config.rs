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
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs::File;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(matches!(config.rules.server_side_exports.severity, Severity::Warn));
        assert!(matches!(config.rules.component_nesting_depth.severity, Severity::Warn));
        assert!(matches!(config.rules.filename_style_consistency.severity, Severity::Warn));
        assert!(matches!(config.rules.missing_companion_files.severity, Severity::Warn));
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
        
        assert!(matches!(config.rules.server_side_exports.severity, Severity::Error));
        assert!(matches!(config.rules.component_nesting_depth.severity, Severity::Warn));
        assert_eq!(config.rules.component_nesting_depth.options.max_nesting_depth, 5);
        assert!(matches!(config.rules.filename_style_consistency.severity, Severity::Error));
        assert!(matches!(config.rules.filename_style_consistency.options.filename_style, FilenameStyle::PascalCase));
        assert!(config.rules.missing_companion_files.options.require_test_files);
        assert!(config.rules.missing_companion_files.options.require_story_files);
        
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
        
        assert!(matches!(config.rules.server_side_exports.severity, Severity::Error));
        assert!(matches!(config.rules.component_nesting_depth.severity, Severity::Warn));
        
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
    fn test_nonexistent_config_file() {
        let config_path = std::env::temp_dir().join("nonexistent-config.json");
        let result = Config::load(&config_path);
        assert!(result.is_err());
    }
}
