# Release Process

This document describes how to create and publish new releases of naechste.

## Prerequisites

- Write access to the repository
- GitHub Personal Access Token (not needed for automated process, but useful for manual testing)

## Automated Release Process

The release process is fully automated via GitHub Actions. Here's what happens:

### 1. Create a New Release

```bash
# Update the version in package.json
npm version patch  # or minor, or major (e.g., 0.1.0 -> 0.1.1)

# Cargo.toml will be automatically updated by the CI workflow
# Just create and push the tag
git tag v$(node -p "require('./package.json').version")
git push origin --tags
```

Or manually:
```bash
# Create a tag manually
git tag v0.1.1
git push origin v0.1.1
```

### 2. GitHub Actions Workflow

When a tag matching `v*` is pushed, the workflow automatically:

1. **Creates a GitHub Release**
   - Tag: v0.1.1
   - Release name: Release v0.1.1

2. **Builds Cross-Platform Binaries**
   - Linux x86_64: `naechste-linux-x86_64.tar.gz`
   - Linux ARM64: `naechste-linux-aarch64.tar.gz`
   - macOS x86_64: `naechste-darwin-x86_64.tar.gz`
   - macOS ARM64: `naechste-darwin-aarch64.tar.gz`
   - Windows x86_64: `naechste-windows-x86_64.zip`

3. **Uploads Binary Artifacts**
   - All binaries are attached to the GitHub Release

4. **Updates Cargo.toml**
   - Automatically syncs version from git tag
   - Commits and pushes changes to main branch
   - Only commits if the version actually changed

5. **Publishes NPM Package**
   - Depends on Cargo.toml update succeeding
   - Package: `@zeropaper/naechste`
   - Registry: GitHub Packages (npm.pkg.github.com)
   - Version matches the git tag

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes, backward compatible

## Manual Release (Emergency)

If the automated process fails, you can create a release manually:

### 1. Build Binaries Locally

```bash
# Install cross-compilation tools if needed
cargo install cross

# Build for all platforms
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-apple-darwin
cross build --release --target x86_64-pc-windows-msvc
```

### 2. Create Archives

```bash
# Linux/macOS
tar czf naechste-linux-x86_64.tar.gz -C target/x86_64-unknown-linux-gnu/release naechste

# Windows
cd target/x86_64-pc-windows-msvc/release
7z a naechste-windows-x86_64.zip naechste.exe
```

### 3. Create GitHub Release

1. Go to https://github.com/zeropaper/naechste/releases/new
2. Create a new tag (e.g., v0.1.1)
3. Upload all binary archives
4. Publish release

### 4. Publish NPM Package

```bash
# Ensure package.json version matches
npm version 0.1.1 --no-git-tag-version

# Publish to GitHub Packages
npm publish
```

## Testing a Release

Before publishing, test the release locally:

### Test Binary Build

```bash
cargo build --release
./target/release/naechste --version
./target/release/naechste examples/plain-starter
```

### Test NPM Package Locally

```bash
# Pack the package
npm pack

# In a test project
npm install /path/to/zeropaper-naechste-0.1.1.tgz
npx naechste --version
```

## Rollback a Release

If a release has issues:

### 1. Delete the Git Tag

```bash
git tag -d v0.1.1
git push origin :refs/tags/v0.1.1
```

### 2. Delete the GitHub Release

1. Go to https://github.com/zeropaper/naechste/releases
2. Find the problematic release
3. Click "Delete"

### 3. Unpublish NPM Package (if needed)

GitHub Packages doesn't support unpublishing, but you can:
- Mark it as deprecated: `npm deprecate @zeropaper/naechste@0.1.1 "This version has been deprecated"`
- Publish a new patch version with fixes

## Troubleshooting

### Build Fails for ARM64

- Ensure cross-compilation tools are installed
- For Linux ARM64: `sudo apt-get install gcc-aarch64-linux-gnu`
- Use `cross` instead of `cargo` for cross-compilation

### NPM Publish Fails

- Verify `GITHUB_TOKEN` has `write:packages` permission
- Check `.npmrc` configuration
- Ensure package.json version is unique (not already published)

### Binary Not Working After Install

- Check platform detection in `scripts/postinstall.js`
- Verify binary permissions (Unix: `chmod +x`)
- Test manual download from GitHub releases

## Release Checklist

Before creating a release:

- [ ] All tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Version updated in `package.json`
- [ ] CHANGELOG.md updated with new changes
- [ ] README.md updated with new features
- [ ] Commit and push all changes
- [ ] Create and push tag

After release:

- [ ] Verify GitHub Release created
- [ ] Verify all binary artifacts uploaded
- [ ] Verify NPM package published
- [ ] Test installation: `pnpm add -D @zeropaper/naechste`
- [ ] Test CLI: `pnpm naechste --version`
