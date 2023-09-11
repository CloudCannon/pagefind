---
title: "Migrating to Pagefind 1.0"
nav_title: "Migrating to Pagefind 1.0"
nav_section: Resources
weight: 81
---

Migrating from Pagefind 0.x to Pagefind 1.0 will work seamlessly in almost all cases, but there are some things to be aware of.

## New default output location

The only breaking change in Pagefind 1.0 is that the default output location has changed from `/_pagefind/` to `/pagefind/`.

> This change was made as some hosting providers won't serve directories with a leading underscore, and some tooling would also ignore the `_pagefind` directory in unexpected ways.

This means that any existing links to assets will need to be updated:
  - For the Default UI, this means referencing `/pagefind/pagefind-ui.js` and `/pagefind/pagefind-ui.css`.
  - For the JavaScript API, you will want to import `/pagefind/pagefind.js`.
  - If you are using [multisite search](/docs/multisite/), make sure to update your `bundlePath` references to reflect the new location.

An alternative to updating these references is to configure your [output subdirectory](/docs/config-options/#output-subdirectory) to match the 0.x default:

```bash
npx pagefind --site "public" --output-subdir "_pagefind"
```

> When indexing your site Pagefind will run in a compatibility mode if it finds any script or style referencing a `/_pagefind/` URL. This won't catch all cases, so it is highly recommended to follow one of the above steps when upgrading.

## Renamed CLI options

Pagefind 1.0 renames some CLI options. These changes are not breaking as the previous options will continue to work, but warnings will now be printed in Pagefind's output until your commands are updated.

***

The previous `source` option has been renamed to `site`:

| Config source        | Old option               | New option             |
|----------------------|--------------------------|------------------------|
| CLI flag             | `--source <PATH>`        | `--site <PATH>`        |
| Environment variable | `PAGEFIND_SOURCE=<PATH>` | `PAGEFIND_SITE=<PATH>` |
| Config file key      | `source`                 | `site`                 |

> This change was made to better reflect the fact that Pagefind modifies the directory you pass to it by default, rather than a typical `source → output` flow.

***

The previous `bundle-dir` option has been renamed to `output-subdir`:

| Config source        | Old option                   | New option                      |
|----------------------|------------------------------|---------------------------------|
| CLI flag             | `--bundle-dir <PATH>`        | `--output-subdir <PATH>`        |
| Environment variable | `PAGEFIND_BUNDLE_DIR=<PATH>` | `PAGEFIND_OUTPUT_SUBDIR=<PATH>` |
| Config file key      | `bundle_dir`                 | `output_subdir`                 |

> This change was made to better convey the fact that this path is relative to the `site` path that Pagefind is indexing.

***

In addition to the above rename, a new `output-path` option has been added:

| Config source        | New option                    |
|----------------------|-------------------------------|
| CLI flag             | `--output-path <PATH>`        |
| Environment variable | `PAGEFIND_OUTPUT_PATH=<PATH>` |
| Config file key      | `output_path`                 |

This option is equivalent, but is relative to the directory you run Pagefind in, rather than the input `site`. This option also accepts absolute paths.

## Changes to search relevancy and ranking

Changes to Pagefind's search rankings are not generally classed as breaking, but this release does change behavior that may be more noticeable than normal on your site.

[Weighting](/docs/weighting/) has been added as a concept, and headings are given a higher priority by default. This means pages with hits inside heading tags will appear higher in the search results.

Compound words are now indexed, meaning words such as `PAGEFIND_BUNDLE_DIR` or `mergeIndex` will appear in searches for `pagefind`, `bundle`, `dir`, `merge`, or `index`. This greatly improves Pagefind's ability to index and search documentation and code, but does mean that more pages will appear for some search terms.

Subwords within a compound word are weighted lower than normal content, so matches for standalone words will generally appear higher in search results.

If you find code is now over-represented in your search results, a good tip is to add `data-pagefind-weight="0.5"` to the code blocks on your site (or any rating between `0.0` and `1.0`).

## Changes to the Pagefind JS API initialization

A change has been made to the way you load and initialize the Pagefind JavaScript API, with the addition of a `pagefind.init()` function:

{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");

+pagefind.init();

let search = await pagefind.search("term");
```
{{< /diffcode >}}

Previously, importing the Pagefind javascript would immediately load your index metadata, and the Pagefind WebAssembly. This prevented configuring Pagefind before loading these assets, for example to specify a custom `bundlePath`.  
In Pagefind 1.0, you can now configure Pagefind before initializing:

{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");

+await pagefind.options({
+    bundlePath: "/some-other-pagefind-directory/"
+});
+pagefind.init();

let search = await pagefind.search("term");
```
{{< /diffcode >}}

This change is not breaking, as calling any other function such as `pagefind.search()` will trigger initialization, meaning all existing code will continue to run.

This **does** change timings for people using the JavaScript API. If your code is not updated, Pagefind will start downloading dependencies at the time of search, rather than the time of import. 

It is now recommended to add a call to `pagefind.init()` when importing the package, or when your search interface gains focus, to help dependencies load before a user types a search query.

## New features

This page summarises the notable changes to existing behavior that Pagefind 1.0 introduces.

For all ✨ new ✨ 1.0 features, see the full [release notes on GitHub](https://github.com/CloudCannon/pagefind/releases).