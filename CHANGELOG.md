# Changelog

<!-- 
    Add changes to the Unreleased section during development.
    Do not change this header — the GitHub action that releases
    this project will edit this file and add the version header for you.
    The Unreleased block will also be used for the GitHub release notes.
-->

## Unreleased

## v0.10.1 (November 23, 2022)

* Changed HTML parsing to a non-strict mode that will no longer error when encountering parsing ambiguities
* Updated the npm wrapper to respect an exit code returned from the main Pagefind binary

## v0.10.0 (November 15, 2022)

### Features & Improvements
* Added the ability to exclude custom selectors via Pagefind config. See the [exclude selectors](/docs/config-options/#exclude-selectors) documentation

### Fixes & Tweaks
* Fixed an issue where running a multi-site search through Pagefind UI wouldn't wait for all search indexes to be ready

## v0.9.3 (November 7, 2022)

* When the search term `null` is passed, Pagefind returns all results with filters applied.
  * In the case of a `null` search and an empty filters object, Pagefind would previously return **zero** results.
  * Pagefind will now return **all** results in this case.

## v0.9.2 (November 6, 2022)

* Pagefind can now automatically read gzipped HTML files as its source
* Pagefind's automatic metadata now falls back to the `title` of a page if there is no `h1` element
* Fixed a couple of inconsistent url formatting issues on Windows

## v0.9.1 (October 26, 2022)

* Fix Windows release assets once more

## v0.9.0 (October 26, 2022)

### Important Changes
* Removed `<header>` from the list of elements that Pagefind automatically ignores
  * If this element contains content you do not want to be indexed, you will now need to add `data-pagefind-ignore`

### Features & Improvements
* Added sorting functionality to Pagefind, see the [Sorting documentation](https://pagefind.app/docs/sorts/) and the [JS API Sorting usage](https://pagefind.app/docs/api/#sorting-results)
* Added the functionality to filter an index without searching, by passing `null` as the search query
* Added support for custom Pagefind UI strings, see [Using custom Pagefind UI strings](https://pagefind.app/docs/ui/#using-custom-pagefind-ui-strings)
* Added a default debounce to the user input for Pagefind UI, and a corresponding `debounceTimeoutMs` option, see [Debounce user input](https://pagefind.app/docs/ui/#debounce-user-input)
  * Many thanks to @dprothero for the contribution! 💝
* Added a hook to process results before showing them in Pagefind UI, see the [processResult documentation](https://pagefind.app/docs/ui/#process-result)

### Fixes & Tweaks
* Fixed running Pagefind on Windows via the npx wrapper
* Pagefind now throws an error if a completely empty index is produced for whatever reason
* Fixed a bug where having exactly one known and one unknown language would drop the known language pages
* Fixed issue where `two<br/>words` would be indexed as `twowords` rather than the correct `two words`
* Added `<style>` to the list of elements that Pagefind automatically ignores
* Fixed the Pagefind UI `showEmptyFilters` option to work as expected
* Fixed issue where adding a filter to a search with zero results would return all results for the filter
* Fixed uncommon bug around chunk boundaries
  * For example: If your first search index chunk started with the word `hello` and you searched for `h`, Pagefind would previously not load the `hello` chunk and would instead return zero results.

## v0.8.1 (September 12, 2022)

* Pagefind now gracefully skips pages that fail HTML parsing, and provides more context when these errors are hit.

## v0.8.0 (August 23, 2022)

### Important Changes
* For those using the JS API directly, the `pagefind.options` function is now async. This will not break current usage, but using newer options may require `await pagefind.options({ ... })` for them to be applied

### Features & Improvements
* Added Multisite search support, allowing you to search multiple indexes from one Pagefind instance. See the new [Multisite documentation](https://pagefind.app/docs/multisite/) for more information
* Added a preload function to the JS search API, allowing you to warm Pagefind up before a search, or while the user is typing. See the [Preload documentation](https://pagefind.app/docs/api/#preloading-search-terms) for more information
* Added a `timings` object to the JS search API response

### Fixes & Tweaks
* Passing a non-existent filter to the search function would previously be silently ignored. This will now return zero results
* Setting your baseURL to an external domain such as `https://example.com` would previously be prepended with a `/`. This is now handled correctly and will link off-site
* Pagefind would previously index search entities such as `&quot;` without unescaping them. This is now fixed and these characters will be skipped
* Searching for only punctuation would previously return all pages currently loaded into the Pagefind index. This will now return zero results
* Fixed a regression causing searches for hyphenated-phrases to return zero results
* Fixed Pagefind UI failing to match complex language codes such as `zh-hans-tw` to less complex translation files such as `zh-tw`

## v0.7.1 (August 13, 2022)

* Added French translations for Pagefind UI — thanks [@nfriedli](https://github.com/nfriedli)! 
* Fixed standard & extended release archives attached to GitHub releases to be correctly assigned

## v0.7.0 (August 12, 2022)

### Features & Improvements

* Multilingual support
  * Pagefind now works out of the box for multilingual sites
  * Pagefind UI is now translated into `af`, `de`, `en`, `ja`, `no`, `pt`, `ru`, & `zh`
  * See the [multilingual documentation](https://pagefind.app/docs/multilingual/) for more information on this feature release
  * This release adds a `pagefind_extended` binary release, which is larger than the `pagefind` release but includes support for indexing Chinese and Japanese languages
    * `pagefind_extended` is now the default when running `npx pagefind`. The smaller `pagefind` release is still available via the [GitHub Release](https://github.com/CloudCannon/pagefind/releases) attachments

### Fixes & Tweaks

* Improved Pagefind logging and added a `--verbose` flag with extra information
* Added warnings when Pagefind encounters pages without outer `<html>` elements
* Added a console warning when Pagefind detects that a cached `pagefind.js` file was loaded alongside a search index from a newer release

## v0.6.1 (August 5, 2022)

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
