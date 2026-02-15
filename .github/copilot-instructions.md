# AI Copilot Instructions for naechste

## Project Overview

**naechste** is a Rust-based CLI tool that lints Next.js file-structure conventions. It's published as:
- A Rust binary (GitHub releases)
- An NPM package (GitHub Packages) with platform-specific binaries bundled via postinstall script

**Core architecture**: Single-pass linter that walks the file system, applies configurable rules, and outputs diagnostics in human-readable or JSON format.

## Module Organization

| Module | Purpose |
|--------|---------|
| `main.rs` | CLI argument parsing (via clap), config resolution, output formatting selection |
| `config.rs` | JSON/YAML config parsing (serde), rule configuration with per-rule severity levels |
| `linter.rs` | File tree walking (walkdir), applies per-file and batch rules, manages ignored directories |
| `rules.rs` | Individual lint rule implementations; each rule is a function taking `&Path, &Config, &mut DiagnosticCollection` |
| `diagnostics.rs` | Diagnostic aggregation and output formatting (human with colors, JSON for CI) |
| `utils.rs` | Common utilities like filename parsing |

**Key insight**: Rules are stateless functions that mutate a mutable diagnostics collection. Batch rules (e.g., `check_file_organization`) receive all files at once after per-file checks.

## Critical Developer Workflows

### Build & Test
```bash
cargo build                    # Debug build
cargo build --release          # Optimized binary (2.2MB, LTO enabled)
cargo test                     # Run all unit + integration tests (60 total)
cargo test --lib              # Unit tests only
cargo test --test integration_tests
```

### Local Testing
```bash
cargo run -- examples/nextjs-sample                        # Test linter
cargo run -- . --format json | jq                         # Test JSON output
cargo run -- . --config naechste.custom.json              # Test custom config
```

### Adding a New Rule
1. Implement rule function in `rules.rs` with signature: `fn check_rulename(path: &Path, config: &Config, diagnostics: &mut DiagnosticCollection)`
2. Add config field to `RuleOptions` struct in `config.rs`
3. Call rule in `linter.rs::lint()` for per-file rules or after file collection for batch rules
4. Add integration test in `tests/integration_tests.rs`

## Project-Specific Patterns

### Configuration System
- **Format discovery**: Tries `naechste.{json,jsonc,yaml,yml}` in project order
- **Default rules**: Each rule has sensible defaults if omitted from config
- **Severity levels**: `warn` (non-fatal) or `error` (exit code 1); only errors trigger non-zero exit
- **Pattern example**: See `examples/file-organization-config.json` for complex file organization rules

### Rule Implementation Pattern
```rust
pub fn check_rule_name(path: &Path, config: &Config, diagnostics: &mut DiagnosticCollection) {
    // Skip non-component files early
    if !should_check(path) { return; }
    
    // Read file or extract metadata
    let content = fs::read_to_string(path).ok()?;
    
    // Check and emit diagnostic
    if condition_violated(&content) {
        diagnostics.add(Diagnostic {
            severity: config.rules.rule_name.severity,
            rule: "rule-name".to_string(),
            message: "Human readable message".to_string(),
            file: path.to_path_buf(),
            line: None, // Optional line numbers
        });
    }
}
```

### Ignored Directories
Hard-coded in `linter.rs::is_ignored()`: `node_modules, .next, .git, dist, build, coverage, out, .turbo`

### File Type Filtering
- Component files: `*.tsx, *.jsx, *.ts, *.js`
- Next.js special files: auto-skipped in filename style checks (`page`, `layout`, `template`, `route`, etc.)

## Release & Publishing Workflow

**Trigger**: Push a git tag matching `v*` (e.g., `git tag v0.1.2 && git push origin v0.1.2`)

**GitHub Actions** (`.github/workflows/publish.yml`):
1. Extract version from tag (e.g., `v0.1.2` → `0.1.2`)
2. Build Linux binary, create archive, upload to GitHub Release
3. **Update Cargo.toml** on main branch (depends on step 2)
4. **Publish to GitHub Packages** (depends on step 3) — ensures npm publishing only succeeds if Cargo.toml sync succeeded

**Key detail**: Both `Cargo.toml` and `package.json` must stay version-synchronized; the workflow ensures this.

## Dependencies & External Integrations

| Dependency | Role |
|-----------|------|
| `clap` | CLI argument parsing with derive macros |
| `serde/serde_json/serde_yaml/json5` | Config parsing, diagnostic JSON output |
| `walkdir` | Recursive file tree traversal |
| `regex` | Pattern matching for server exports, filename validation |
| `colored` | Human-readable console output with colors |
| `glob` | Glob pattern matching for file organization rules |

## Output Formats

**Human** (default, colored):
```
✓ No issues found!
```
or
```
[error] server-side-exports: Server-side export 'getServerSideProps' found in client component
  → src/components/Form.tsx
```

**JSON** (for CI/CD):
```json
{
  "diagnostics": [
    {
      "severity": "error",
      "rule": "server-side-exports",
      "message": "...",
      "file": "src/components/Form.tsx",
      "line": null
    }
  ]
}
```

## NPM Package Structure

- `bin/naechste`: Symlink/placeholder binary (platform-specific binary downloaded by `scripts/postinstall.js`)
- `scripts/postinstall.js`: Downloads correct binary from GitHub releases based on OS/arch
- `package.json` declares supported platforms: `darwin, linux, win32` with `x64, arm64` architectures

