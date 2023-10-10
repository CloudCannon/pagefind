---
title: "Indexing content using the NodeJS API"
nav_title: "Using the NodeJS API"
nav_section: References
weight: 55
---

Pagefind provides an interface to the indexing binary as a NodeJS package you can import.

There are situations where using this Node library is beneficial:
- Authors integrating Pagefind into an existing project, e.g. integrating Pagefind into the dev mode of a static site generator, can pass in-memory HTML files to Pagefind. Pagefind can also return the search index in-memory, to be hosted via the dev mode alongside the files.
- Users looking to index their site and augment that index with extra non-HTML pages can run a standard Pagefind crawl with [`addDirectory`](#indexadddirectory) and augment it with [`addCustomRecord`](#indexaddcustomrecord).
- Users looking to use Pagefind's engine for searching miscellaneous content such as PDFs or subtitles, where [`addCustomRecord`](#indexaddcustomrecord) can be used to build the entire index from scratch.

## Example usage

```js
import * as pagefind from "pagefind";

// Create a Pagefind search index to work with
const { index } = await pagefind.createIndex();

// Index all HTML files in a directory
await index.addDirectory({
    path: "public"
});

// Add extra content
await index.addCustomRecord({
    url: "/resume.pdf",
    content: "Aenean lacinia bibendum nulla sed consectetur",
    language: "en",
});

// Get the index files in-memory
const { files } = await index.getFiles();

// Or, write the index to disk
await index.writeFiles({
    outputPath: "public/pagefind"
});
```

All interactions with Pagefind are asynchronous, as they communicate with the native Pagefind binary in the background.

## pagefind.createIndex

Creates a Pagefind index that files can be added to.  
The index object returned is unique, and multiple calls to `pagefind.createIndex()` can be made without conflicts.

```js
import * as pagefind from "pagefind";

const { index } = await pagefind.createIndex();

// ... do things with `index`
```

`createIndex` optionally takes a configuration object that can apply parts of the [Pagefind CLI config](/docs/config-options/). The options available at this level are:

```js
const { index } = await pagefind.createIndex({
    rootSelector: "html",
    excludeSelectors: [".my-code-blocks"],
    forceLanguage: "en",
    keepIndexUrl: false,
    verbose: false,
    logfile: "debug.log"
});
```

See the relevant documentation for these configuration options in the [Configuring the Pagefind CLI](/docs/config-options/) documentation.

## index.addDirectory

Indexes a directory from disk using the standard Pagefind indexing behaviour.  
This is equivalent to running the Pagefind binary with `--site <dir>`.

```js
// Index all HTML files in the public directory
const { errors, page_count } = await index.addDirectory({
    path: "public"
});

// Index files in the directory that match the given glob
const { errors, page_count } = await index.addDirectory({
    path: "public",
    glob: "**/*.{html}"
});
```

If the `path` provided is relative, it will be relative to the current working directory of your Node process.

Optionally, a custom `glob` can be supplied which controls which files Pagefind will consume within the directory. The default is shown, and the `glob` option can be omitted entirely.  
See [Wax patterns documentation](https://github.com/olson-sean-k/wax#patterns) for more details.

A response with an `errors` array containing error messages indicates that Pagefind failed to process this directory.
If successful, `page_count` will be the number of pages that were added to the index.

## index.addHTMLFile

Adds a virtual HTML file to the Pagefind index. Useful for files that don't exist on disk, for example a static site generator that is serving files from memory.

```js
// Index a file as if Pagefind was indexing from disk
const { errors, file } = await index.addHTMLFile({
    sourcePath: "contact/index.html",
    content: "<html lang='en'><body> <h1>A Full HTML Document</h1> <p> . . . </p> </body></html>"
});

// Index HTML content, giving it a specific URL
const { errors, file } = await index.addHTMLFile({
    url: "/contact/",
    content: "<html lang='en'><body> <h1>A Full HTML Document</h1> <p> . . . </p> </body></html>"
});
```

The `sourcePath` should represent the path of this HTML file if it were to exist on disk. Pagefind will use this path to generate the URL. It should be relative, or absolute to a path within the current working directory.

Instead of `sourcePath`, a `url` may be supplied to explicitly set the URL of this search result.

The `content` should be the full HTML source, including the outer `<html> </html>` tags. This will be run through Pagefind's standard HTML indexing process, and should contain any required Pagefind attributes to control behaviour.

A response with an `errors` array containing error messages indicates that Pagefind failed to index this content.
If successful, the `file` object is returned containing metadata about the completed indexing.

## index.addCustomRecord

Adds a direct record to the Pagefind index. Useful for adding non-HTML content to the search results.

```js
const { errors, file } = await index.addCustomRecord({
    url: "/contact/",
    content: "My raw content to be indexed for search. Will be lightly processed by Pagefind.",
    language: "en",
    meta: {
        title: "Contact",
        category: "Landing Page"
    },
    filters: {
        tags: ["landing", "company"]
    },
    sort: {
        weight: "20"
    }
});
```

The `url`, `content`, and `language` fields are all required. `language` should be an [ISO 639-1 code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).

`meta` is optional, and is strictly a flat object of keys to string values.  
See the [Metadata documentation](https://pagefind.app/docs/metadata/) for semantics.

`filters` is optional, and is strictly a flat object of keys to arrays of string values.  
See the [Filters documentation](https://pagefind.app/docs/filtering/) for semantics.

`sort` is optional, and is strictly a flat object of keys to string values.  
See the [Sort documentation](https://pagefind.app/docs/sorts/) for semantics.  
*When Pagefind is processing an index, number-like strings will be sorted numerically rather than alphabetically. As such, the value passed in should be `"20"` and not `20`*

A response with an `errors` array containing error messages indicates that Pagefind failed to index this content.
If successful, the `file` object is returned containing metadata about the completed indexing.

## index.getFiles

Get raw data of all files in the Pagefind index. Useful for integrating a Pagefind index into the development mode of a static site generator and hosting these files yourself.

```js
const { errors, files } = await index.getFiles();

for (const file of files) {
    const output_url = `/pagefind/${file.path}`;
    // do something with the file.content Uint8Array
}
```

A response with an `errors` array containing error messages indicates that Pagefind failed to action this request.

If successful, `files` will be an array containing file objects. Each object contains a `content` key with the raw data as a Uint8Array `path` key, and a `path` key which is the relative URL this file should be served at within a bundle directory. 

## index.writeFiles

Writes the index files to disk, as they would be written when running the standard Pagefind binary directly.

```js
const { errors } = await index.writeFiles({
    outputPath: "./public/pagefind"
});
```

The `outputPath` option should contain the path to the desired Pagefind bundle directory. If relative, is relative to the current working directory of your Node process.

A response with an `errors` array containing error messages indicates that Pagefind failed to action this request.

## index.deleteIndex

Deletes the data for the given index from the Pagefind binary service. Doesn't affect any written files or data returned by `getFiles()`.

```js
await index.deleteIndex();
```

Calling `index.getFiles()` or `index.writeFiles()` doesn't consume the index, and further modifications can be made. In situations where many indexes are being created, the `deleteIndex` call helps clear out memory from the Pagefind binary service.

Reusing an `index` object after calling `index.deleteIndex()` will cause errors to be returned.

Not calling this method is fine — these indexes will be cleaned up when your Node process exits.

## pagefind.close

Closes the Pagefind service and errors out any pending tasks, stopping the linked binary altogether.  
Called on the top-level `pagefind` object.

```js
import * as pagefind from "pagefind";

const { index } = await pagefind.createIndex();

// ... do things with `index`

// clean up once complete
await pagefind.close();
```
