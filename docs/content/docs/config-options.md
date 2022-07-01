---
date: 2022-06-01
title: "Configuring the Pagefind CLI"
nav_title: "Configuration options"
nav_section: CLI configuration
weight: 51
---

The Pagefind CLI has the following options:

> For configuring the Pagefind search in the browser, see [Pagefind Search Config](/docs/search-config/). For configuring the Pagefind UI, see [Pagefind UI](/docs/ui/).

## Required arguments

### Source
The location of your built static site

| CLI Flag   | ENV Variable      | Config Key |
|------------|-------------------|------------|
| `--source` | `PAGEFIND_SOURCE` | `source`   |

## Optional arguments

### Bundle directory
The folder to output search files into, relative to source. Defaults to `_pagefind`

| CLI Flag       | ENV Variable          | Config Key   |
|----------------|-----------------------|--------------|
| `--bundle-dir` | `PAGEFIND_BUNDLE_DIR` | `bundle_dir` |

