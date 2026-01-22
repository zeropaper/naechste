use crate::config::Severity;
use colored::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub rule: String,
    pub message: String,
    pub file: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DiagnosticCollection {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticCollection {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, Severity::Error))
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, Severity::Error))
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, Severity::Warn))
            .count()
    }
}

pub fn print_human(collection: &DiagnosticCollection) {
    if collection.diagnostics.is_empty() {
        println!("{}", "✓ No issues found!".green().bold());
        return;
    }

    for diagnostic in &collection.diagnostics {
        let severity_str = match diagnostic.severity {
            Severity::Error => "error".red().bold(),
            Severity::Warn => "warn".yellow().bold(),
        };

        let file_path = diagnostic.file.display();
        let location = if let Some(line) = diagnostic.line {
            format!("{}:{}", file_path, line)
        } else {
            format!("{}", file_path)
        };

        println!(
            "{}: {} [{}]",
            severity_str,
            diagnostic.message,
            diagnostic.rule.cyan()
        );
        println!("  {} {}", "-->".blue(), location);
        println!();
    }

    let error_count = collection.error_count();
    let warning_count = collection.warning_count();

    if error_count > 0 {
        println!(
            "{} {} error(s), {} warning(s) found",
            "✗".red().bold(),
            error_count,
            warning_count
        );
    } else {
        println!(
            "{} {} warning(s) found",
            "⚠".yellow().bold(),
            warning_count
        );
    }
}

pub fn print_json(collection: &DiagnosticCollection) {
    let json = serde_json::to_string_pretty(collection).unwrap();
    println!("{}", json);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_diagnostic_collection_new() {
        let collection = DiagnosticCollection::new();
        assert_eq!(collection.diagnostics.len(), 0);
        assert!(!collection.has_errors());
        assert_eq!(collection.error_count(), 0);
        assert_eq!(collection.warning_count(), 0);
    }

    #[test]
    fn test_add_diagnostic() {
        let mut collection = DiagnosticCollection::new();
        
        collection.add(Diagnostic {
            severity: Severity::Warn,
            rule: "test-rule".to_string(),
            message: "Test warning".to_string(),
            file: PathBuf::from("test.ts"),
            line: Some(10),
        });
        
        assert_eq!(collection.diagnostics.len(), 1);
        assert_eq!(collection.warning_count(), 1);
        assert_eq!(collection.error_count(), 0);
        assert!(!collection.has_errors());
    }

    #[test]
    fn test_has_errors() {
        let mut collection = DiagnosticCollection::new();
        
        collection.add(Diagnostic {
            severity: Severity::Warn,
            rule: "test-rule".to_string(),
            message: "Test warning".to_string(),
            file: PathBuf::from("test.ts"),
            line: None,
        });
        
        assert!(!collection.has_errors());
        
        collection.add(Diagnostic {
            severity: Severity::Error,
            rule: "test-rule".to_string(),
            message: "Test error".to_string(),
            file: PathBuf::from("test.ts"),
            line: None,
        });
        
        assert!(collection.has_errors());
    }

    #[test]
    fn test_error_count() {
        let mut collection = DiagnosticCollection::new();
        
        collection.add(Diagnostic {
            severity: Severity::Error,
            rule: "rule1".to_string(),
            message: "Error 1".to_string(),
            file: PathBuf::from("test1.ts"),
            line: None,
        });
        
        collection.add(Diagnostic {
            severity: Severity::Warn,
            rule: "rule2".to_string(),
            message: "Warning 1".to_string(),
            file: PathBuf::from("test2.ts"),
            line: None,
        });
        
        collection.add(Diagnostic {
            severity: Severity::Error,
            rule: "rule3".to_string(),
            message: "Error 2".to_string(),
            file: PathBuf::from("test3.ts"),
            line: None,
        });
        
        assert_eq!(collection.error_count(), 2);
        assert_eq!(collection.warning_count(), 1);
    }

    #[test]
    fn test_diagnostic_serialization() {
        let diagnostic = Diagnostic {
            severity: Severity::Error,
            rule: "test-rule".to_string(),
            message: "Test message".to_string(),
            file: PathBuf::from("test.ts"),
            line: Some(42),
        };
        
        let json = serde_json::to_string(&diagnostic).unwrap();
        assert!(json.contains("\"severity\":\"error\""));
        assert!(json.contains("\"rule\":\"test-rule\""));
        assert!(json.contains("\"message\":\"Test message\""));
        assert!(json.contains("\"line\":42"));
    }

    #[test]
    fn test_diagnostic_serialization_without_line() {
        let diagnostic = Diagnostic {
            severity: Severity::Warn,
            rule: "test-rule".to_string(),
            message: "Test message".to_string(),
            file: PathBuf::from("test.ts"),
            line: None,
        };
        
        let json = serde_json::to_string(&diagnostic).unwrap();
        assert!(!json.contains("\"line\""));
    }

    #[test]
    fn test_collection_serialization() {
        let mut collection = DiagnosticCollection::new();
        
        collection.add(Diagnostic {
            severity: Severity::Error,
            rule: "rule1".to_string(),
            message: "Error message".to_string(),
            file: PathBuf::from("error.ts"),
            line: Some(10),
        });
        
        collection.add(Diagnostic {
            severity: Severity::Warn,
            rule: "rule2".to_string(),
            message: "Warning message".to_string(),
            file: PathBuf::from("warn.ts"),
            line: None,
        });
        
        let json = serde_json::to_string(&collection).unwrap();
        assert!(json.contains("\"diagnostics\""));
        assert!(json.contains("\"rule1\""));
        assert!(json.contains("\"rule2\""));
    }
}
