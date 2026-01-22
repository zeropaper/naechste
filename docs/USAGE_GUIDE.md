# naechste User Guide: Do's and Don'ts

This guide provides practical examples of what to do and what to avoid when using **naechste** to enforce Next.js file-structure conventions.

## Table of Contents

1. [Server-Side Exports](#server-side-exports)
2. [Component Nesting Depth](#component-nesting-depth)
3. [Filename Style Consistency](#filename-style-consistency)
4. [Missing Companion Files](#missing-companion-files)
5. [Configuration](#configuration)

---

## Server-Side Exports

**Rule**: `server-side-exports`

This rule prevents server-only functions from being exported in client components (files marked with `'use client'`).

### ✅ DO

**File**: `app/components/UserProfile.tsx`
```tsx
// Server component - no 'use client' directive
import { db } from '@/lib/db'

export async function UserProfile({ userId }: { userId: string }) {
  // This is fine - it's server-side by default in Next.js 13+ app router
  const user = await db.query('SELECT * FROM users WHERE id = ?', [userId])
  
  return (
    <div>
      <h1>{user.name}</h1>
      <p>{user.email}</p>
    </div>
  )
}
```

**File**: `app/components/UserButton.tsx`
```tsx
'use client'

import { useState } from 'react'

export function UserButton() {
  const [clicked, setClicked] = useState(false)
  
  return (
    <button onClick={() => setClicked(!clicked)}>
      {clicked ? 'Clicked' : 'Click me'}
    </button>
  )
}
```

### ❌ DON'T

**File**: `app/components/BadUserProfile.tsx`
```tsx
'use client'

import { db } from '@/lib/db'

export function BadUserProfile() {
  return <div>Profile</div>
}

// ❌ ERROR: Server-side export in client component
export async function getServerSideProps() {
  const user = await db.query('SELECT * FROM users')
  return { props: { user } }
}
```

**File**: `app/api/data-loader.tsx`
```tsx
'use client'

// ❌ ERROR: These are server-only exports
export async function getStaticProps() {
  return { props: {} }
}

export async function getStaticPaths() {
  return { paths: [], fallback: 'blocking' }
}

export async function getInitialProps() {
  return { data: [] }
}

export function DataLoader() {
  return <div>Loading...</div>
}
```

### Recommended Configuration

```json
{
  "rules": {
    "server_side_exports": {
      "severity": "error",
      "options": {}
    }
  }
}
```

**Why error level?** This is a critical issue that breaks your application.

---

## Component Nesting Depth

**Rule**: `component-nesting-depth`

This rule ensures your components and pages don't nest too deeply in the filesystem, making them harder to find and maintain.

### ✅ DO

```
app/
  page.tsx                          ✓ depth: 1
  layout.tsx                        ✓ depth: 1
  components/
    Header.tsx                      ✓ depth: 2
    Navigation.tsx                  ✓ depth: 2
    navigation/
      MainNav.tsx                   ✓ depth: 3
      Sidebar.tsx                   ✓ depth: 3
  dashboard/
    page.tsx                        ✓ depth: 2
    components/
      DashboardCard.tsx             ✓ depth: 3
```

**File**: `app/dashboard/components/stats-card.tsx`
```tsx
// ✓ Depth: 3 (within default maximum)
export function StatsCard({ label, value }) {
  return (
    <div className="card">
      <h3>{label}</h3>
      <p>{value}</p>
    </div>
  )
}
```

### ❌ DON'T

```
app/
  features/                         ❌ depth: 2
    users/                          ❌ depth: 3
      management/                   ❌ depth: 4
        components/                 ❌ depth: 5
          ui/                       ❌ depth: 6
            forms/                  ❌ depth: 7
              UserForm.tsx          ❌ depth: 8 (EXCEEDS LIMIT)
```

**File**: `app/features/users/management/admin/components/forms/inputs/TextInput.tsx`
```tsx
// ❌ Depth: 8 - way too deep!
// This file is hard to find and the structure is confusing
export function TextInput({ label, value, onChange }) {
  return (
    <div>
      <label>{label}</label>
      <input value={value} onChange={onChange} />
    </div>
  )
}
```

### How to Fix Deep Nesting

**Problem Structure**:
```
app/features/users/management/admin/components/forms/UserForm.tsx (depth: 7)
```

**Better Structure**:
```
app/
  admin/
    components/
      UserForm.tsx (depth: 3) ✓
```

**Or reorganize by feature**:
```
app/
  dashboard/
    components/
      UserForm.tsx (depth: 3) ✓
  users/
    components/
      UserCard.tsx (depth: 3) ✓
```

### Recommended Configuration

```json
{
  "rules": {
    "component_nesting_depth": {
      "severity": "warn",
      "options": {
        "max_nesting_depth": 3
      }
    }
  }
}
```

**Tip**: Set to `warn` initially to identify problematic areas, then enforce as `error` once refactored.

---

## Filename Style Consistency

**Rule**: `filename-style-consistency`

Enforce consistent naming conventions across your codebase. Choose one style and stick with it.

### ✅ DO - Kebab Case (Default)

```tsx
// ✓ All files follow kebab-case
my-component.tsx
user-profile.tsx
button.tsx
hero-section.tsx
modal-dialog.tsx
```

### ✅ DO - PascalCase

```tsx
// ✓ All files follow PascalCase
MyComponent.tsx
UserProfile.tsx
Button.tsx
HeroSection.tsx
ModalDialog.tsx
```

### ✅ DO - camelCase

```tsx
// ✓ All files follow camelCase
myComponent.tsx
userProfile.tsx
button.tsx
heroSection.tsx
modalDialog.tsx
```

### ✅ DO - snake_case

```tsx
// ✓ All files follow snake_case
my_component.tsx
user_profile.tsx
button.tsx
hero_section.tsx
modal_dialog.tsx
```

### ❌ DON'T - Mixed Styles

```tsx
// ❌ Inconsistent naming - breaks the rule!
MyComponent.tsx      (PascalCase)
userProfile.tsx      (camelCase)
button-group.tsx     (kebab-case)
modal_dialog.tsx     (snake_case)
```

### Special Cases (Automatically Ignored)

These Next.js special files are **always ignored** by this rule:

```tsx
// ✓ No warnings for these special files
page.tsx
layout.tsx
template.tsx
loading.tsx
error.tsx
not-found.tsx
route.ts
default.tsx
middleware.ts

// ✓ Config files are also ignored
next.config.js
tailwind.config.js
postcss.config.js
```

### Configuration Examples

**For kebab-case projects** (recommended for React components):
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

**For PascalCase projects** (if you prefer):
```json
{
  "rules": {
    "filename_style_consistency": {
      "severity": "error",
      "options": {
        "filename_style": "PascalCase"
      }
    }
  }
}
```

---

## Missing Companion Files

**Rule**: `missing-companion-files`

Ensure that components have associated test files and/or story files for documentation and testing.

### ✅ DO

**Structure with Test Files**:
```
components/
  Button.tsx           (Component)
  Button.test.tsx      (Unit test) ✓
  Button.stories.tsx   (Storybook story) ✓

  Modal.tsx
  Modal.spec.tsx       (Spec format also works) ✓
  Modal.stories.tsx    ✓

  Input.tsx
  __tests__/
    Input.test.tsx     (Alternative test location) ✓
```

**File**: `components/Button.tsx`
```tsx
export function Button({ label, onClick }) {
  return <button onClick={onClick}>{label}</button>
}
```

**File**: `components/Button.test.tsx`
```tsx
import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { Button } from './Button'

describe('Button', () => {
  it('renders with label', () => {
    render(<Button label="Click me" onClick={() => {}} />)
    expect(screen.getByText('Click me')).toBeInTheDocument()
  })
})
```

**File**: `components/Button.stories.tsx`
```tsx
import type { Meta, StoryObj } from '@storybook/react'
import { Button } from './Button'

const meta: Meta<typeof Button> = {
  component: Button,
}

export default meta
type Story = StoryObj<typeof Button>

export const Default: Story = {
  args: {
    label: 'Click me',
  },
}
```

### ❌ DON'T - Component Without Tests or Stories

```
components/
  Button.tsx          (Component only - no tests!)
  Input.tsx           (Component only - no stories!)
  Checkbox.tsx        (Component only - no companion files!)
```

### Recommended Configuration

**For teams requiring comprehensive testing**:
```json
{
  "rules": {
    "missing_companion_files": {
      "severity": "error",
      "options": {
        "require_test_files": true,
        "require_story_files": true
      }
    }
  }
}
```

**For teams enforcing only tests**:
```json
{
  "rules": {
    "missing_companion_files": {
      "severity": "warn",
      "options": {
        "require_test_files": true,
        "require_story_files": false
      }
    }
  }
}
```

**For teams with optional companion files**:
```json
{
  "rules": {
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

### Additional Companion Patterns Recognized

- **Integration tests**: `*.test.int.ts`, `*.test.int.tsx` (example: `MyComponent.test.int.ts`)
- **Page user-scenarios**: `page.us.md` — markdown files used as companion documentation for `page.tsx` / `page.jsx` files

### Custom Companion Patterns

You can customize which companion files are recognized (and required) via the `missing_companion_files` rule options using `companion_file_patterns`. Patterns are globs applied relative to the component/page directory.

```json
{
  "rules": {
    "missing_companion_files": {
      "severity": "warn",
      "options": {
        "require_test_files": true,
        "require_story_files": false,
        "companion_file_patterns": {
          "integration_tests": ["*.test.int.ts", "*.test.int.tsx"],
          "page_user_scenarios": ["page.us.md"]
        }
      }
    }
  }
}
```

This configuration will cause the linter to treat `MyComponent.test.int.ts` as a valid companion test file and will also look for `page.us.md` alongside `page.tsx` / `page.jsx` files when checking for page companions.


### Test File Patterns Recognized

- `.test.tsx`, `.test.ts`, `.test.jsx`, `.test.js`
- `.spec.tsx`, `.spec.ts`, `.spec.jsx`, `.spec.js`
- `__tests__/` directory with test files inside

### Story File Patterns Recognized

- `.stories.tsx`, `.stories.ts`, `.stories.jsx`, `.stories.js`
- `.story.tsx`, `.story.ts`, `.story.jsx`, `.story.js`

---

## Configuration

### Complete Example Configuration

Create `naechste.json` in your project root:

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
      "severity": "error",
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

### Severity Levels Explained

| Level | Behavior | Use Case |
|-------|----------|----------|
| `error` | Fails CI, exits with code 1 | Critical rules that must be enforced |
| `warn` | Reports but passes CI, exits with code 0 | Guidelines that should improve gradually |

### Automatically Ignored Directories

These directories are **never** linted:

- `node_modules`
- `.next`
- `.git`
- `dist`
- `build`
- `coverage`
- `out`
- `.turbo`

### Running the Linter

```bash
# Lint current directory
naechste

# Lint specific directory
naechste /path/to/project

# Output as JSON (for CI/CD)
naechste --format json

# Use custom config file
naechste --config my-config.json

# Get help
naechste --help
```

### CI/CD Integration

**GitHub Actions Example**:
```yaml
name: Lint Next.js Structure
on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install naechste
        run: cargo install --path .
      
      - name: Run linter
        run: naechste --format json
```

---

## Quick Reference

| Rule | Default Severity | Default Options | Key Takeaway |
|------|------------------|-----------------|--------------|
| `server-side-exports` | Error | (none) | Don't use server APIs in `'use client'` components |
| `component-nesting-depth` | Warn | max: 3 | Keep folder structure shallow and navigable |
| `filename-style-consistency` | Warn | kebab-case | Pick ONE naming convention and stick with it |
| `missing-companion-files` | Warn | optional | Document and test your components |

---

## Troubleshooting

### "Error: File not found: naechste.json"

**Solution**: Create the config file in your project root with default settings (or use the example above).

### "Warning: Nesting depth exceeds maximum"

**Solution**: Reorganize your component structure to be flatter. Move components up one or more levels.

**Before**:
```
app/features/users/settings/profile/components/UserForm.tsx (depth: 6)
```

**After**:
```
app/components/UserForm.tsx (depth: 2)
```

### "Error: Filename does not match expected style"

**Solution**: Rename your files to match your configured style:

```bash
# If using kebab-case
mv MyComponent.tsx my-component.tsx

# If using PascalCase
mv my-component.tsx MyComponent.tsx
```

### "Warning: Missing test file for component"

**Solution**: Add a test file next to your component:

```bash
# Create a test file
touch components/Button.test.tsx
```

---

## Best Practices

1. **Start with warnings**: Set rules to `warn` initially, then gradually enforce as `error` once your codebase is compliant.

2. **Choose one style**: For filename consistency, pick ONE convention and use it everywhere.

3. **Keep structure flat**: Aim for a maximum nesting depth of 2-3 levels for easy navigation.

4. **Test your components**: Companion test files help prevent regressions and document component behavior.

5. **Use sensible defaults**: The default configuration covers most projects well.

6. **Commit your config**: Include `naechste.json` in version control so all team members use the same rules.

---

## Getting Help

For more information:
- Check the [main README](../README.md)
- Review the [implementation summary](../IMPLEMENTATION_SUMMARY.md)
- See [example projects](../examples/)
