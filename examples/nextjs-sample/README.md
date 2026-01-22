# Example Next.js Project

This is a sample Next.js project with intentional file structure issues to demonstrate the `naechste` linter.

## Issues in this project:

1. **MyBadComponent.tsx** - Has server-side export (`getServerSideProps`) in a client component
2. **MyBadComponent.tsx** - Filename doesn't follow kebab-case convention
3. **DeepComponent.tsx** - Exceeds maximum nesting depth (5 levels deep, max is 3)
4. **DeepComponent.tsx** - Filename doesn't follow kebab-case convention

## Running the linter

From the repository root:

```bash
# Human-readable output
naechste examples/nextjs-sample

# JSON output for CI/CD
naechste examples/nextjs-sample --format json
```

## Expected output

You should see 1 error and 3 warnings:
- 1 error for the server-side export in client component
- 3 warnings for filename style and nesting depth issues
