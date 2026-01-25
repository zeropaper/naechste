# Quick Start: Installing naechste from GitHub Packages

This guide will help you install and use `naechste` from GitHub Packages.

## Step 1: Create a GitHub Token

1. Go to https://github.com/settings/tokens
2. Click **"Generate new token (classic)"**
3. Give it a name: `naechste-npm-package`
4. Select scope: **`read:packages`**
5. Click **"Generate token"**
6. **Copy the token** (you won't see it again!)

## Step 2: Configure NPM

### Option A: Project-level (Recommended)

Create a `.npmrc` file in your project root:

```bash
# .npmrc
@zeropaper:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=YOUR_TOKEN_HERE
```

### Option B: Global (All Projects)

Create or edit `~/.npmrc`:

```bash
# ~/.npmrc
@zeropaper:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=YOUR_TOKEN_HERE
```

### Option C: Environment Variable (CI/CD)

```bash
export GITHUB_TOKEN=your_token_here
```

Then use this in `.npmrc`:
```bash
@zeropaper:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${GITHUB_TOKEN}
```

## Step 3: Install naechste

```bash
# Using pnpm (recommended)
pnpm add -D @zeropaper/naechste

# Using npm
npm install --save-dev @zeropaper/naechste

# Using yarn
yarn add -D @zeropaper/naechste
```

## Step 4: Run naechste

```bash
# Via package manager
pnpm naechste
npx naechste
yarn naechste

# With options
pnpm naechste --format json
pnpm naechste /path/to/project
pnpm naechste --config custom.json
```

## Step 5: Add to package.json Scripts

```json
{
  "scripts": {
    "lint:structure": "naechste",
    "lint:structure:json": "naechste --format json"
  }
}
```

Then run:
```bash
pnpm lint:structure
```

## Configuration

Create `naechste.json` in your project root:

```json
{
  "rules": {
    "server_side_exports": {
      "severity": "error"
    },
    "component_nesting_depth": {
      "severity": "warn",
      "options": {
        "max_nesting_depth": 3
      }
    },
    "filename_style_consistency": {
      "severity": "warn",
      "options": {
        "filename_style": "kebab-case"
      }
    },
    "missing_companion_files": {
      "severity": "warn",
      "options": {
        "require_test_files": false,
        "require_story_files": false
      }
    }
  }
}
```

## Troubleshooting

### Error: 401 Unauthorized

- Check your GitHub token is valid
- Ensure token has `read:packages` scope
- Verify `.npmrc` configuration

### Error: 404 Not Found

- Package may not be published yet
- Check package name: `@zeropaper/naechste`
- Verify registry URL: `https://npm.pkg.github.com`

### Binary not found after install

- Check postinstall script ran: `npm rebuild @zeropaper/naechste`
- Verify platform is supported (Linux, macOS, Windows on x64/ARM64)
- Check binary exists: `ls node_modules/@zeropaper/naechste/bin/`

### Permission denied (Unix)

```bash
chmod +x node_modules/@zeropaper/naechste/bin/naechste
```

## GitHub Actions Setup

Add to your `.github/workflows/lint.yml`:

```yaml
name: Lint Structure
on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      
      - name: Configure NPM for GitHub Packages
        run: |
          echo "@zeropaper:registry=https://npm.pkg.github.com" > .npmrc
          echo "//npm.pkg.github.com/:_authToken=${{ secrets.GITHUB_TOKEN }}" >> .npmrc
      
      - name: Install dependencies
        run: pnpm install
      
      - name: Lint Next.js structure
        run: pnpm naechste --format json
```

## Next Steps

- Read the [full documentation](README.md)
- Check out [usage examples](docs/USAGE_GUIDE.md)
- See [CI/CD integration examples](examples/github-actions.md)

## Support

For issues or questions:
- Open an issue: https://github.com/zeropaper/naechste/issues
- Check existing issues for solutions
