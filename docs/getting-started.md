---
title: Getting Started
---

# Getting started with naechste

Install (Node toolchain example):

```
pnpm add -D naechste
pnpm naechste init
```

Or build from source (Rust):

```
cargo install --path .
naechste --help
```

Configuration

Create `naechste.json`, `naechste.yaml`, or `naechste.jsonc` in your project root. See `docs/schemas/` for the JSON schema to validate the configuration file.

Examples and full usage are available in `docs/USAGE_GUIDE.md`.
