use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::process;

mod config;
mod diagnostics;
mod linter;
mod rules;
mod utils;

#[derive(Parser)]
#[command(name = "naechste")]
#[command(version = "0.1.0")]
#[command(about = "A fast, Rust-first CLI to enforce Next.js file-structure conventions", long_about = None)]
struct Cli {
    /// Path to the Next.js project directory
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,

    /// Path to configuration file
    #[arg(short, long, default_value = "naechste.json")]
    config: PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// Human-readable output with colors
    Human,
    /// JSON output for CI/CD
    Json,
}

fn main() {
    let cli = Cli::parse();

    // Determine config path - if not explicitly provided, look in project directory
    let config_path = if cli.config.to_str() == Some("naechste.json") {
        // Default case: look for config in the project directory across supported formats
        find_config_in_directory(&cli.path)
    } else {
        // Explicitly provided config path
        cli.config
    };

    // Load configuration
    let config = config::Config::load(&config_path).unwrap_or_else(|e| {
        eprintln!("Warning: Could not load config file: {}", e);
        eprintln!("Using default configuration");
        config::Config::default()
    });

    // Run the linter
    let diagnostics = linter::lint(&cli.path, &config);

    // Output diagnostics
    match cli.format {
        OutputFormat::Human => diagnostics::print_human(&diagnostics),
        OutputFormat::Json => diagnostics::print_json(&diagnostics),
    }

    // Exit with appropriate code
    let exit_code = if diagnostics.has_errors() { 1 } else { 0 };
    process::exit(exit_code);
}

fn find_config_in_directory(base: &std::path::Path) -> std::path::PathBuf {
    let candidates = [
        "naechste.json",
        "naechste.jsonc",
        "naechste.yaml",
        "naechste.yml",
    ];

    for candidate in candidates {
        let path = base.join(candidate);
        if path.exists() {
            return path;
        }
    }

    // Fallback to the default JSON path even if it does not exist
    base.join("naechste.json")
}
