# Changelog

<!-- 
    Add changes to the Unreleased section during development.
    Do not change this header — the GitHub action that releases
    this project will edit this file and add the version header for you.
    The Unreleased block will also be used for the GitHub release notes.
-->

## Unreleased

* Oops — fixes the npx wrapper on Windows

## v0.6.0 (August 5, 2022)

### Features & Improvements

* Added prebuilt Windows binaries
  * This adds Windows support for running Pagefind via `npx pagefind`
  * Windows binaries can also be downloaded via the [GitHub Releases](https://github.com/CloudCannon/pagefind/releases)

## v0.5.3 (July 30, 2022)

* Tēnā koutou katoa — Fixed an issue where Pagefind could not search for words containing some special characters.

## v0.5.2 (July 29, 2022)

* Support Apple Silicon Macs (via Rosetta 2)

## v0.5.1 (July 29, 2022)

* Fixed a form submission error in Pagefind UI causing trouble for some content security policies
* Fixed a visual quirk in the Pagefind UI filters on Safari

## v0.5.0 (July 26, 2022)

### Features & Improvements

* The glob that Pagefind uses for finding files to index can now be configured. See [Config > Glob](https://pagefind.app/docs/config-options/#glob)
* Added a `data-pagefind-ignore="all"` option that does not process filters or metadata within the target element. See [Indexing > Removing individual elements from the index](https://pagefind.app/docs/indexing/#removing-individual-elements-from-the-index)
* Added a `data-pagefind-default-meta` attribute that can provide fallback values for metadata that could not be found on the page. See [Metadata > Defining default metadata](https://pagefind.app/docs/metadata/#defining-default-metadata)
* UI: Pagefind UI will now check for a `url` key in a page's metadata that should be used over the generated URL. See [UI > Overriding the URL of a result](https://pagefind.app/docs/ui/#overriding-the-url-of-a-result)
* UI: Added a configuration option for hiding images from the Pagefind UI. See [UI > Show images](https://pagefind.app/docs/ui/#show-images) 
* UI: Added a configuration option to hide filter values that have no available results given the search query and existing filters. See [UI > Show empty filters](https://pagefind.app/docs/ui/#show-empty-filters)
* UI: The Pagefind UI filter panel will now default to expanded if there are sufficiently few filters

### Fixes & Tweaks

* Server gzip support
  * Pagefind implements its own gzip handling, but would fail in the rare case that a server detected the gzipped files and served them such that a browser would decompress them. Pagefind will now identify that these files have already been decompressed rather than error.
* UI: HTML entity improvements
  * Fixed the remaining elements in the Pagefind UI that did not correctly render HTML entities.
* UI: Filter state improvements
  * Previously, deleting a search term would reset the selected filter values and the open filter groups. This state is now preserved when the search input is empty.
* UI: The Pagefind UI JS and CSS files are now correctly minified

## v0.4.1 (July 7, 2022)

### Fixes & Tweaks

* Hash fragment contents using the entire file, to prevent stale content
* Use the image_alt correctly in Pagefind UI

## v0.4.0 (July 6, 2022)

### Features & Improvements

* An automatic `image_alt` metadata value will be included when returning an automatic `image` metadata value. See [Metadata > Default metadata](https://pagefind.app/docs/metadata/#default-metadata)
* Multiple filter and metadata keys set can be set per element. See [Metadata > Defining multiple keys](https://pagefind.app/docs/metadata/#defining-multiple-metadata-keys-on-a-single-element)
* A root selector can now be configured to further restrict Pagefind indexing. See [Config > Root selector](https://pagefind.app/docs/config-options/#root-selector)
* If re-running Pagefind over an output directory, existing hashed files will be reused if present, which will improve hot build speeds for large sites.
* Added latest version to the header of the [documentation](https://pagefind.app).

### Fixes & Tweaks

* Added `<template>` elements to the ignored text index list
* Multiple exact matches on a page will no longer be returned as separate results

## v0.3.2 (July 4, 2022)

Tidied up the search API output to remove some not-yet-implemented fields.

## v0.3.1 (July 4, 2022)

Changed images in Pagefind UI to contain rather than cover. Relevant CSS variables have changed slightly.

## v0.3.0 (July 2, 2022)

Added a `--serve` option to the Pagefind CLI that will host the site on a local development server after building the search index.

## v0.2.0 (July 1, 2022)

The first stable 0.x release of Pagefind.
