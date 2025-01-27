---
title: "Pagefind CLI configuration options"
nav_title: "CLI config options"
nav_section: References
weight: 51
---

The Pagefind CLI has the following options.
These can be set via any [configuration source](/docs/config-sources/).

> These configuration options only apply when running the Pagefind indexing tool on your site.
> For configuring Pagefind search in the browser, see [Pagefind Search Config](/docs/search-config/).
> For configuring the Pagefind Default UI, see [Pagefind UI](/docs/ui/).

## Required arguments

### Site
The location of your built static site.

| CLI Flag        | ENV Variable    | Config Key |
|-----------------|-----------------|------------|
| `--site <PATH>` | `PAGEFIND_SITE` | `site`     |

## Optional arguments

### Serve
Serve the site directory after creating the search index. Useful for testing search on a local build of your site without having to serve the site directory manually.

| CLI Flag  | ENV Variable     | Config Key |
|-----------|------------------|------------|
| `--serve` | `PAGEFIND_SERVE` | `serve`    |

### Output subdirectory
The folder to output the search bundle into, relative to the processed site. Defaults to `pagefind`.

| CLI Flag                | ENV Variable             | Config Key      |
|-------------------------|--------------------------|-----------------|
| `--output-subdir <DIR>` | `PAGEFIND_OUTPUT_SUBDIR` | `output_subdir` |

### Output path
The folder to output the search bundle into, relative to the working directory. Overrides `output-subdir` if supplied.

| CLI Flag               | ENV Variable           | Config Key    |
|------------------------|------------------------|---------------|
| `--output-path <PATH>` | `PAGEFIND_OUTPUT_PATH` | `output_path` |

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

### Include characters
Prevents Pagefind from stripping the provided characters when indexing content.
Allows users to search for words including these characters.

See [Indexing special characters](/docs/indexing/#indexing-special-characters) for more documentation.

Care is needed if setting this argument via the CLI, as special characters may be interpreted by your shell.
Configure this via a [configuration file](/docs/config-sources/#config-files) if you encounter issues.

```yml
include_characters: "<>$"
```

| CLI Flag                   | ENV Variable                  | Config Key          |
|----------------------------|-------------------------------|---------------------|
| `--include-characters <S>` | `PAGEFIND_INCLUDE_CHARACTERS` | `include_characters` |

### Glob
Configures the glob used by Pagefind to discover HTML files. Defaults to `**/*.{html}`.
See [Wax patterns documentation](https://github.com/olson-sean-k/wax#patterns) for more details.

| CLI Flag        | ENV Variable    | Config Key |
|-----------------|-----------------|------------|
| `--glob <GLOB>` | `PAGEFIND_GLOB` | `glob`     |

### Force language
Ignores any detected languages and creates a single index for the entire site as the provided language. Expects an ISO 639-1 code, such as `en` or `pt`.

See [Multilingual search](/docs/multilingual/) for more details.

| CLI Flag                  | ENV Variable              | Config Key       |
|---------------------------|---------------------------|------------------|
| `--force-language <LANG>` | `PAGEFIND_FORCE_LANGUAGE` | `force_language` |

### Keep index URL
Keeps `index.html` at the end of search result paths. By default, a file at `animals/cat/index.html` will be given the URL `/animals/cat/`. Setting this option to `true` will result in the URL `/animals/cat/index.html`.

| CLI Flag           | ENV Variable     | Config Key       |
|--------------------|------------------|------------------|
| `--keep-index-url` | `KEEP_INDEX_URL` | `keep_index_url` |

### Write playground
Writes the Pagefind playground files to `/playground` within your bundle directory. For most sites, this will make the Pagefind playground available at `/pagefind/playground/`.

This defaults to false, so playground files are not written to your live site. Playground files are always available when running Pagefind with `--serve`.

| CLI Flag             | ENV Variable       | Config Key         |
|----------------------|--------------------|--------------------|
| `--write-playground` | `WRITE_PLAYGROUND` | `write_playground` |

### Verbose
Prints extra logging while indexing the site. Only affects the CLI, does not impact web-facing search.

| CLI Flag    | ENV Variable       | Config Key |
|-------------|--------------------|------------|
| `--verbose` | `PAGEFIND_VERBOSE` | `verbose`  |

### Quiet
Only logs errors and warnings while indexing the site. Only affects the CLI, does not impact web-facing search.

| CLI Flag  | ENV Variable     | Config Key |
|-----------|------------------|------------|
| `--quiet` | `PAGEFIND_QUIET` | `quiet`  |

### Silent
Only logs errors while indexing the site. Only affects the CLI, does not impact web-facing search.

| CLI Flag  | ENV Variable     | Config Key |
|-----------|------------------|------------|
| `--silent` | `PAGEFIND_SILENT` | `silent`  |

### Logfile
Writes logs to the given logfile, in addition to the console. Replaces the file on each run.

| CLI Flag           | ENV Variable       | Config Key |
|--------------------|--------------------|------------|
| `--logfile <PATH>` | `PAGEFIND_LOGFILE` | `logfile`  |
