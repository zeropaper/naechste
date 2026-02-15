---
title: naechste
description: A fast, Rust-first CLI to enforce Next.js file-structure conventions
---

# naechste

`naechste` helps teams bring consistent file and naming conventions to their frontend codebases (Next.js, Vite, Astro, ...).

Why this exists:

- JavaScript/React projects typically lack enforced file-structure conventions.
- `naechste` provides rules, diagnostics, and optional scaffolding to make projects consistent and easier to maintain.

Features

- Lint rules for server/client conventions, filename casing, component nesting depth, and file-organization checks.
- Configurable via `naechste.json`, `naechste.yaml` or `naechste.jsonc`.
- Can be used locally, in CI, or by developer tools/agents to scaffold and enforce conventions.

Getting started

See the getting-started guide: [Getting Started](getting-started.md)

Schemas

The configuration JSON schemas are published in the repository under `docs/schemas/`. You can fetch them directly:

- /docs/schemas/naechste.json
- /docs/schemas/naechste-0.1.2-beta.2.json

Contribution

If you'd like to help, open a PR — the site is published from the `docs/` folder via GitHub Pages.
