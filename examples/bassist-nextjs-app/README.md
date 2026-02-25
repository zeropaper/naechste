# Bassist Next.js App Example

This directory contains a sample Next.js application structure demonstrating Bassist conventions and violations.

## Structure

The app follows the Bassist route group organization:
- `(admin)` - Admin dashboard domain
- `(auth)` - Authentication domain
- `(profiles)` - User profiles domain

## Violations Included

This example intentionally includes violations to test naechste's Bassist rules:

1. **bassist-locale-nesting**: `app/(profiles)/page.tsx` is not in `[locale]/` directory
2. **bassist-service-client-restriction**: `app/(admin)/lib/profiles.ts` uses service client in production
3. **bassist-i18n-hook-usage**: Wrong hooks used in client/server components
4. **bassist-supabase-client-imports**: Wrong Supabase client imports
5. **bassist-i18n-namespaces**: Missing namespace dots in i18n keys
6. **bassist-domain-isolation**: Cross-domain imports from sibling domain's lib/
7. **bassist-api-route-structure**: API routes outside api/ directories
8. **bassist-test-colocation**: Tests in root tests/ directory
9. **bassist-test-naming**: Test files with wrong extensions

## Testing

Run naechste with the Bassist preset:

```bash
# Using CLI flag
naechste examples/bassist-nextjs-app --preset bassist

# Using config file
naechste examples/bassist-nextjs-app --config examples/bassist-config.json
```

Expected: Multiple diagnostics reported for various violations.
