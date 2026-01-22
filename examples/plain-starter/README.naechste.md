# Plain Starter - Real Next.js Project Example

This is a real Next.js project created from the official Vercel `with-supabase` template. It demonstrates that `naechste` works perfectly with standard Next.js project structures.

## Project Setup

This project was created using:
```bash
npm i -g pnpm # if you don't have pnpm installed
pnpm create next-app . --yes --use-pnpm --tailwind --ts --app --turbopack --disable-git --biome
echo "!README.naechste.md" >> .gitignore
pnpm dlx shadcn@latest add button
pnpm dlx shadcn@latest add @supabase/social-auth-nextjs
pnpm dlx shadcn@latest add @supabase/password-based-auth-nextjs
pnpm dlx shadcn@latest add @supabase/dropzone-nextjs
pnpm dlx shadcn@latest add @supabase/realtime-cursor-nextjs
pnpm dlx shadcn@latest add @supabase/current-user-avatar-nextjs
pnpm dlx shadcn@latest add @supabase/realtime-avatar-stack-nextjs
pnpm dlx shadcn@latest add @supabase/realtime-chat-nextjs
pnpm dlx shadcn@latest add @supabase/infinite-query-hook
```

## Structure

The project follows standard Next.js 15 conventions:
- **App Router** with `app/` directory
- **TypeScript** configuration
- **Tailwind CSS** for styling
- **Supabase** integration
- **shadcn/ui** components in `components/ui/`

## File Naming Conventions

This project demonstrates proper Next.js file naming:
- Route files: `page.tsx`, `layout.tsx`, `route.ts`
- Components: kebab-case (`auth-button.tsx`, `theme-switcher.tsx`)
- UI components: kebab-case (`button.tsx`, `card.tsx`)
- Config files: Automatically excluded from linting

## Running the Linter

From the repository root:

```bash
# Human-readable output
naechste examples/plain-starter

# JSON output for CI/CD
naechste examples/plain-starter --format json
```

## Expected Result

This project should pass cleanly with **zero errors and zero warnings** when using the default configuration:

```
✓ No issues found!
```

## Why This Passes

1. **File naming**: All components follow kebab-case convention
2. **No server exports in client components**: Proper separation
3. **Reasonable nesting depth**: No deeply nested component structures
4. **Config files excluded**: Standard Next.js config files are automatically skipped

## Testing Custom Rules

You can modify `.next-structure-lintrc.json` to test different configurations:

### Require Test Files
```json
{
  "rules": {
    "missing_companion_files": {
      "severity": "warn",
      "options": {
        "require_test_files": true
      }
    }
  }
}
```

### Enforce PascalCase for Components
```json
{
  "rules": {
    "filename_style_consistency": {
      "severity": "error",
      "options": {
        "filename_style": "pascal-case"
      }
    }
  }
}
```

### Stricter Nesting Depth
```json
{
  "rules": {
    "component_nesting_depth": {
      "severity": "error",
      "options": {
        "max_nesting_depth": 2
      }
    }
  }
}
```
