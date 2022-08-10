---
date: 2022-06-01
title: "Configuring the Pagefind CLI"
nav_title: "Pagefind CLI config"
nav_section: Installing
weight: 3
---

The Pagefind CLI has the following options:

> For configuring the Pagefind search in the browser, see [Pagefind Search Config](/docs/search-config/). For configuring the Pagefind UI, see [Pagefind UI](/docs/ui/).

## Required arguments

### Source
The location of your built static site.

| CLI Flag            | ENV Variable      | Config Key |
|---------------------|-------------------|------------|
| `--source <SOURCE>` | `PAGEFIND_SOURCE` | `source`   |

## Optional arguments

### Serve
Serve the source directory after creating the search index. Useful for testing search on a local build of your site without having to serve the source directory manually.

| CLI Flag  | ENV Variable     | Config Key |
|-----------|------------------|------------|
| `--serve` | `PAGEFIND_SERVE` | `serve`    |

### Bundle directory
The folder to output search files into, relative to source. Defaults to `_pagefind`.

| CLI Flag             | ENV Variable          | Config Key   |
|----------------------|-----------------------|--------------|
| `--bundle-dir <DIR>` | `PAGEFIND_BUNDLE_DIR` | `bundle_dir` |

### Root selector
The element that Pagefind should treat as the root of the document. Defaults to `html`.

Note that filters and metadata outside of this selector will **not** be detected, all Pagefind behaviour will be limited to this element and below. In most cases, you should use the `data-pagefind-body` attribute detailed in [Customizing the index](/docs/indexing/).

| CLI Flag              | ENV Variable             | Config Key      |
|-----------------------|--------------------------|-----------------|
| `--root-selector <S>` | `PAGEFIND_ROOT_SELECTOR` | `root_selector` |

### Glob
Configures the glob used by Pagefind to discover HTML files. Defaults to `**/*.{html}`.

| CLI Flag        | ENV Variable    | Config Key |
|-----------------|-----------------|------------|
| `--glob <GLOB>` | `PAGEFIND_GLOB` | `glob`     |

### Force Language
Ignores any detected languages and creates a single index for the entire site as the provided language. Expects an ISO 639-1 code, such as `en` or `pt`.

See [Multilingual search](/docs/multilingual/) for more details.

| CLI Flag                  | ENV Variable              | Config Key       |
|---------------------------|---------------------------|------------------|
| `--force-language <LANG>` | `PAGEFIND_FORCE_LANGUAGE` | `force_language` |
