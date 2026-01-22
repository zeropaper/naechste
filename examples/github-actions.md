# GitHub Actions Workflow Example

Here's how to integrate `naechste` into your GitHub Actions CI/CD pipeline:

```yaml
name: Lint Next.js Structure

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  lint:
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
      
      - name: Cache Cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Build naechste
        run: cargo build --release
      
      - name: Run linter
        run: ./target/release/naechste --format json > lint-results.json
      
      - name: Upload lint results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: lint-results
          path: lint-results.json
      
      - name: Comment PR with results
        if: github.event_name == 'pull_request' && failure()
        uses: actions/github-script@v6
        with:
          script: |
            const fs = require('fs');
            const results = JSON.parse(fs.readFileSync('lint-results.json', 'utf8'));
            const diagnostics = results.diagnostics;
            
            const errors = diagnostics.filter(d => d.severity === 'error');
            const warnings = diagnostics.filter(d => d.severity === 'warn');
            
            const comment = `## 🔍 Next.js Structure Lint Results
            
            - ❌ ${errors.length} error(s)
            - ⚠️ ${warnings.length} warning(s)
            
            ### Errors
            ${errors.map(e => `- **${e.rule}**: ${e.message} in \`${e.file}\``).join('\n')}
            
            ### Warnings
            ${warnings.map(w => `- **${w.rule}**: ${w.message} in \`${w.file}\``).join('\n')}
            `;
            
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: comment
            });
```

## Alternative: Using Pre-built Binary

For faster CI runs, you can build once and upload the binary as a release artifact:

```yaml
- name: Download naechste
  run: |
    curl -L https://github.com/zeropaper/naechste/releases/latest/download/naechste-linux-x86_64 -o naechste
    chmod +x naechste

- name: Run linter
  run: ./naechste --format json
```
