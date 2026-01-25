# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-25

### Added
- NPM package publishing to GitHub Packages (@zeropaper/naechste)
- Automated release workflow via GitHub Actions
- Cross-platform binary builds (Linux x64/ARM64, macOS x64/ARM64, Windows x64)
- Postinstall script for automatic binary download
- Authentication documentation for GitHub Packages
- Installation guide (INSTALL.md)
- Release process documentation (RELEASE.md)
- GitHub Actions CI/CD integration examples
- Matrix build strategy for multi-platform support
- Binary hosting via GitHub release artifacts

### Changed
- README.md updated with NPM installation instructions
- CONTRIBUTING.md updated with NPM testing guidelines
- .gitignore updated to exclude NPM artifacts

### Infrastructure
- package.json - NPM wrapper configuration
- scripts/postinstall.js - Platform-specific binary installer
- .github/workflows/publish.yml - Automated release pipeline
- .npmrc.template - Authentication configuration template
- bin/naechste - CLI stub script

## Initial Release - Pre-NPM

### Features
- Rust-based CLI for Next.js file structure linting
- Four built-in rules:
  - server-side-exports
  - component-nesting-depth
  - filename-style-consistency
  - missing-companion-files
- JSON and human-readable output formats
- Configurable severity levels (warn/error)
- CI/CD ready with proper exit codes
- Comprehensive test coverage (60 tests)
- Documentation and examples

[Unreleased]: https://github.com/zeropaper/naechste/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/zeropaper/naechste/releases/tag/v0.1.0
