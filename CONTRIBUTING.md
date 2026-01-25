# Contributing to naechste

Thank you for your interest in contributing to naechste! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- Node.js 16+ (for NPM package testing)
- pnpm, npm, or yarn (optional, for NPM package testing)

### Getting Started

1. Clone the repository:
```bash
git clone https://github.com/zeropaper/naechste.git
cd naechste
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Run the linter on the example project:
```bash
cargo run -- examples/nextjs-sample
```

## Project Structure

```
naechste/
├── src/
│   ├── main.rs              # CLI entry point and argument parsing
│   ├── config.rs            # Configuration file parsing
│   ├── diagnostics.rs       # Diagnostic output formatting
│   ├── linter.rs            # Main linting logic and file walking
│   └── rules.rs             # Built-in linting rules
├── tests/
│   └── integration_tests.rs # End-to-end CLI tests
├── examples/
│   ├── nextjs-sample/       # Sample Next.js project for testing
│   └── github-actions.md    # CI/CD integration examples
└── Cargo.toml               # Project dependencies and metadata
```

## Running Tests

### Unit Tests

Run all unit tests (49 tests covering config, diagnostics, linter, and rules):
```bash
cargo test --lib
```

### Integration Tests

Run end-to-end CLI tests (11 tests):
```bash
cargo test --test integration_tests
```

### All Tests

Run both unit and integration tests (60 total):
```bash
cargo test
```

### Testing the NPM Package Locally

1. Build the Rust binary:
```bash
cargo build --release
```

2. Create archives for your platform:
```bash
# Linux/macOS
tar czf naechste-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m).tar.gz -C target/release naechste

# Windows (PowerShell)
Compress-Archive -Path target/release/naechste.exe -DestinationPath naechste-windows-x86_64.zip
```

3. Test NPM package installation:
```bash
# Create a test directory
mkdir /tmp/test-naechste
cd /tmp/test-naechste
npm init -y

# Copy the archive to simulate a GitHub release
# Then install the package
npm pack /path/to/naechste
npm install ./zeropaper-naechste-0.1.0.tgz

# Test the CLI
npx naechste --version
```

### Test Coverage by Module

- **config.rs**: 8 tests
  - Default configuration
  - Config file loading and parsing
  - Partial config with defaults
  - Invalid config handling
  - Serialization/deserialization

- **diagnostics.rs**: 7 tests
  - Diagnostic collection
  - Error and warning counting
  - JSON serialization
  - Human-readable output formatting

- **linter.rs**: 9 tests
  - Directory walking and file filtering
  - Ignored directories (node_modules, .next, etc.)
  - File type detection
  - Multi-file processing

- **rules.rs**: 25 tests
  - Server-side exports detection
  - Component nesting depth validation
  - Filename style checking (all 4 styles)
  - Companion files validation

- **integration_tests.rs**: 11 tests
  - CLI help and version
  - Exit code validation
  - JSON vs human output
  - Config file resolution
  - Multi-file scenarios

## Adding a New Rule

To add a new linting rule:

1. **Add configuration** in `src/config.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules {
    // ... existing rules ...
    #[serde(default = "default_rule_config")]
    pub my_new_rule: RuleConfig,
}
```

2. **Implement the rule** in `src/rules.rs`:
```rust
pub fn check_my_new_rule(
    path: &Path,
    config: &Config,
    diagnostics: &mut DiagnosticCollection,
) {
    // Your rule logic here
}
```

3. **Call the rule** in `src/linter.rs`:
```rust
rules::check_my_new_rule(file_path, config, &mut diagnostics);
```

4. **Add tests** in `src/rules.rs`:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_my_new_rule() {
        // Test your rule
    }
}
```

## Code Style

- Follow Rust standard formatting (use `cargo fmt`)
- Run clippy for lints: `cargo clippy`
- Ensure all tests pass before submitting
- Write tests for new functionality

## Testing Your Changes

1. Run all tests:
```bash
cargo test
```

2. Test with the example project:
```bash
cargo run -- examples/nextjs-sample
```

3. Test with JSON output:
```bash
cargo run -- examples/nextjs-sample --format json
```

4. Build release version:
```bash
cargo build --release
./target/release/naechste examples/nextjs-sample
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-new-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit your changes (`git commit -am 'Add new feature'`)
6. Push to your branch (`git push origin feature/my-new-feature`)
7. Create a Pull Request

## Pull Request Guidelines

- Describe what your PR does
- Include tests for new functionality
- Update documentation if needed
- Ensure all tests pass
- Keep PRs focused on a single feature or fix

## Release Process

See [RELEASE.md](RELEASE.md) for detailed information about creating releases.

Quick summary:
1. Update version in `package.json`
2. Create and push a git tag (e.g., `v0.1.1`)
3. GitHub Actions automatically builds and publishes

## Questions?

Feel free to open an issue for any questions or concerns!
