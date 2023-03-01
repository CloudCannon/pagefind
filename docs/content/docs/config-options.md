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

### Exclude selectors
Pass extra element selectors that Pagefind should ignore when indexing. For example, in `pagefind.yml`:

```yml
exclude_selectors:
  - "#my_navigation"
  - "blockquote > span"
  - "[id^='prefix-']"
```

All children will also be ignored, so using a `#my_navigation *` selector is not required — in other words, the semantics are the same as the [data-pagefind-ignore](/docs/indexing/#removing-individual-elements-from-the-index) attribute.

Note that currently Pagefind only supports lists of options via configuration files. If using the `--exclude-selectors` CLI flag or the `PAGEFIND_EXCLUDE_SELECTORS` environment variable, only one selector may be supplied. The selector may be a comma-separated CSS selector though, so the above example can be passed as `--exclude-selectors "#my_navigation, blockquote > span, [id^='prefix-']"`.

| CLI Flag                  | ENV Variable                 | Config Key          |
|---------------------------|------------------------------|---------------------|
| `--exclude-selectors <S>` | `PAGEFIND_EXCLUDE_SELECTORS` | `exclude_selectors` |

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

### Keep Index URL
Keeps `index.html` at the end of search result paths. By default, a file at `animals/cat/index.html` will be given the URL `/animals/cat/`. Setting this option to `true` will result in the URL `/animals/cat/index.html`.

| CLI Flag           | ENV Variable     | Config Key       |
|--------------------|------------------|------------------|
| `--keep-index-url` | `KEEP_INDEX_URL` | `keep_index_url` |

### Verbose
Prints extra logging while indexing the site. Only affects the CLI, does not impact web-facing search.

| CLI Flag    | ENV Variable       | Config Key |
|-------------|--------------------|------------|
| `--verbose` | `PAGEFIND_VERBOSE` | `verbose`  |
