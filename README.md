# naechste

A fast, Rust-first CLI to enforce Next.js file-structure conventions.

## Features

- **Fast & Lightweight**: Single static binary built in Rust
- **Deterministic**: Emits consistent diagnostics across runs
- **Configurable**: Per-rule options and severity levels via `naechste.json`
- **CI/CD Ready**: JSON output format and proper exit codes for automation
- **GitHub Actions Compatible**: Easy integration with CI workflows

## Built-in Rules

### 1. Server-Side Exports (`server-side-exports`)
Detects server-side only exports in client components (those with `'use client'` directive).

**Examples:**
- ❌ `getServerSideProps` in a file with `'use client'`
- ❌ `getStaticProps` in a file with `'use client'`
- ❌ `getStaticPaths` in a file with `'use client'`
- ❌ `getInitialProps` in a file with `'use client'`

### 2. Component Nesting Depth (`component-nesting-depth`)
Enforces maximum nesting depth for components in `app/` or `pages/` directories.

**Default**: Maximum depth of 3 levels

**Example:**
- ✅ `app/components/Button.tsx` (depth: 2)
- ❌ `app/components/ui/buttons/primary/Button.tsx` (depth: 5, exceeds default of 3)

### 3. Filename Style Consistency (`filename-style-consistency`)
Enforces consistent filename naming conventions.

**Supported styles:**
- `kebab-case` (default): `my-component.tsx`
- `PascalCase`: `MyComponent.tsx`
- `camelCase`: `myComponent.tsx`
- `snake_case`: `my_component.tsx`

**Note**: Special Next.js files (`page`, `layout`, `template`, `loading`, `error`, `not-found`, `route`, `default`, `middleware`) are automatically skipped.

### 4. Missing Companion Files (`missing-companion-files`)
Ensures components have associated test and/or story files.

**Options:**
- `require_test_files`: Require `.test.tsx`, `.spec.tsx`, or `__tests__/` files
- `require_story_files`: Require `.stories.tsx` or `.story.tsx` files

## Installation

### Via NPM (Recommended)

**Prerequisites**: This package is published to GitHub Packages and requires authentication.

1. **Create or update `.npmrc`** in your project root or home directory:

```bash
# .npmrc
@zeropaper:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${GITHUB_TOKEN}
```

2. **Set your GitHub token** as an environment variable:

```bash
# Create a GitHub Personal Access Token with 'read:packages' scope at:
# https://github.com/settings/tokens

export GITHUB_TOKEN=your_github_token_here
```

3. **Install the package**:

```bash
# Using pnpm
pnpm add -D @zeropaper/naechste

# Using npm
npm install --save-dev @zeropaper/naechste

# Using yarn
yarn add -D @zeropaper/naechste
```

4. **Run the CLI**:

```bash
# Via package manager
pnpm naechste
npx naechste
yarn naechste

# Or directly
./node_modules/.bin/naechste
```

### From Source

```bash
git clone https://github.com/zeropaper/naechste
cd naechste
cargo build --release
```

The binary will be available at `target/release/naechste`.

## Usage

### Basic Usage

```bash
# Lint current directory
naechste

# Lint specific directory
naechste /path/to/nextjs/project

# Output JSON format (for CI/CD)
naechste --format json

# Use custom config file
naechste --config my-config.json
```

### Configuration

Create a `naechste.json` file in your project root:

```json
{
  "rules": {
    "server_side_exports": {
      "severity": "error",
      "options": {}
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

### Severity Levels

- `warn`: Reports issue but doesn't fail CI (exit code 0)
- `error`: Reports issue and fails CI (exit code 1)

### Output Formats

#### Human-Readable (default)

```
error: Server-side export 'getServerSideProps' found in client component [server-side-exports]
  --> src/components/MyComponent.tsx

warn: Filename 'MyComponent' does not match expected style: KebabCase [filename-style-consistency]
  --> src/components/MyComponent.tsx

✗ 1 error(s), 1 warning(s) found
```

#### JSON (for CI/CD)

```json
{
  "diagnostics": [
    {
      "severity": "error",
      "rule": "server-side-exports",
      "message": "Server-side export 'getServerSideProps' found in client component",
      "file": "src/components/MyComponent.tsx"
    }
  ]
}
```

## CI/CD Integration

### GitHub Actions

#### Using NPM Package

```yaml
name: Lint Next.js Structure
on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          registry-url: 'https://npm.pkg.github.com'
          scope: '@zeropaper'
      
      - name: Create .npmrc
        run: |
          echo "@zeropaper:registry=https://npm.pkg.github.com" > .npmrc
          echo "//npm.pkg.github.com/:_authToken=${{ secrets.GITHUB_TOKEN }}" >> .npmrc
      
      - name: Install dependencies
        run: pnpm install
      
      - name: Run naechste
        run: pnpm naechste --format json
```

#### Using Rust (Build from Source)

```yaml
name: Lint Next.js Structure
on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install naechste
        run: cargo install --path .
      
      - name: Run linter
        run: naechste --format json
```

### Exit Codes
      - name: Install naechste
        run: cargo install --path .
      
      - name: Run linter
        run: naechste --format json
```

### Exit Codes

- `0`: No errors found (warnings are OK)
- `1`: One or more errors found

## Examples

### Example 1: Client Component with Server Export

**File**: `app/components/UserProfile.tsx`
```tsx
'use client'

export function UserProfile() {
  return <div>Profile</div>
}

// ❌ This will trigger an error
export async function getServerSideProps() {
  return { props: {} }
}
```

**Output**:
```
error: Server-side export 'getServerSideProps' found in client component [server-side-exports]
  --> app/components/UserProfile.tsx
```

### Example 2: Deep Nesting

**File**: `app/features/dashboard/widgets/charts/line/LineChart.tsx`

With default config (max depth: 3), this will trigger:
```
warn: Component nesting depth 5 exceeds maximum of 3 [component-nesting-depth]
  --> app/features/dashboard/widgets/charts/line/LineChart.tsx
```

### Example 3: Filename Style

**Config**:
```json
{
  "rules": {
    "filename_style_consistency": {
      "severity": "error",
      "options": {
        "filename_style": "kebab-case"
      }
    }
  }
}
```

**Files**:
- ✅ `my-component.tsx`
- ❌ `MyComponent.tsx` → triggers error
- ❌ `myComponent.tsx` → triggers error

## Ignored Directories

The following directories are automatically ignored:
- `node_modules`
- `.next`
- `.git`
- `dist`
- `build`
- `coverage`
- `out`
- `.turbo`

## Real-World Example

The `examples/plain-starter` directory contains a complete Next.js project created with:
```bash
npx create-next-app --use-npm --tailwind --ts --app --turbopack --no-git --eslint -e with-supabase plain-starter
```

This real-world project demonstrates that `naechste` works perfectly with standard Next.js conventions and passes cleanly with zero errors when properly configured.

Run the linter on this example:
```bash
naechste examples/plain-starter
# Output: ✓ No issues found!
```

See `examples/plain-starter/README.naechste.md` for more details.

## Publishing Releases

This project uses automated releases via GitHub Actions. To publish a new version:

1. **Update version in `package.json`**:
```bash
npm version patch  # or minor, or major
```

2. **Create and push a git tag**:
```bash
git tag v0.1.1
git push origin v0.1.1
```

3. **GitHub Actions will automatically**:
   - Build binaries for all platforms (Linux, macOS, Windows on x64 and ARM64)
   - Create a GitHub release with binary artifacts
   - Publish the NPM package to GitHub Packages (`@zeropaper/naechste`)
   - Update `Cargo.toml` with the new version

### Supported Platforms

The automated build creates binaries for:
- **Linux**: x86_64, aarch64
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Windows**: x86_64

## Roadmap

- [ ] LSP (Language Server Protocol) support
- [ ] WASM build for browser usage
- [ ] Automatic fixes (--fix flag)
- [ ] Additional rules:
  - [ ] Import organization
  - [ ] Barrel file detection
  - [ ] Client/Server component boundary checks
  - [ ] Route parameter validation

## Contributing

Contributions are welcome! Please open an issue or pull request.

## License

MIT

