---
date: 2022-06-01
title: "Pagefind CLI configuration sources"
nav_title: "Config sources"
nav_section: Installing
weight: 2
---

Pagefind can be configured through CLI flags, environment variables, or configuration files. Values will be merged from all sources, with CLI flags overriding environment variables, and environment variables overriding configuration files.

## Config files

Pagefind will look for a `pagefind.toml`, `pagefind.yml`, or `pagefind.json` file in the directory that you have run the command in.

```yaml
# pagefind.yml
source: public
bundle_dir: _pagefind
```
```bash
npx pagefind
```

## Environment variables

Pagefind will load any values via a `PAGEFIND_*` environment variable.

```bash
export PAGEFIND_BUNDLE_DIR="_pagefind"
PAGEFIND_SOURCE="public" npx pagefind
```

## CLI flags

Pagefind can be passed CLI flags directly.

```bash
npx pagefind --source public --bundle-dir _pagefind
```
