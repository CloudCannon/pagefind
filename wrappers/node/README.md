# Pagefind Static Search

Pagefind is a fully static search library that aims to perform well on large sites, while using as little of your users’ bandwidth as possible, and without hosting any infrastructure.

The full documentation on using Pagefind can be found at https://pagefind.app/.

This packages houses a wrapper for running the precompiled Pagefind binary, and also serves as a NodeJS indexing library that can be integrated into existing tools.

## Running Pagefind through NPX

This is the recommended way of running Pagefind on a static site.

```
npx pagefind --source "public"
```

For more details on using the Pagefind binary, see [Installing and running Pagefind](https://pagefind.app/docs/installation/#running-via-npx), and the rest of the Pagefind documentation.

## Using Pagefind as a Node library

This package also provides an interface to the Pagefind binary directly as a package you can import.
This generally isn't required, and running the binary directly on your source code is the recommended approach
for the majority of use-cases.

The rest of this documentation assumes you have a solid understanding of how to use Pagefind conventionally. Read through the [standard Pagefind documentation](https://pagefind.app/) first, if you haven't.

***

Using this indexing library is handy if you're integrating Pagefind directly into a static site generator, or for complex tasks like indexing JSON files or other non-HTML sources of content.

Example usage:

```js
import * as pagefind from "pagefind";

// Create a Pagefind search index to work with
const { index } = await pagefind.createIndex();

// Add content to it
await index.addHTMLFile({
    path: "my_file/index.html",
    content: "<html><body><h1>Testing, testing</h1></body></html>"
});

// Get the index in-memory
await index.getFiles();

// Write the index to disk
await index.writeFiles({
    bundlePath: "./public/pagefind"
});
```

All interations with Pagefind are asynchronous, as they communicate with the native Pagefind binary in the background.

### `pagefind.createIndex`

Creates a Pagefind index that files can be added to.
The index object returned is unique, and multiple calls to `pagefind.createIndex()` can be made without conflicts.

```js
import * as pagefind from "pagefind";

const { index } = await pagefind.createIndex();

// ... do things with `index`
```

`createIndex` optionally takes a configuration object that can apply parts of the [Pagefind CLI config](https://pagefind.app/docs/config-options/). The options available at this level are:

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

See the relevant documentation of each configuration option in the [Configuring the Pagefind CLI](https://pagefind.app/docs/config-options/) documentation.

### index.addDirectory

Indexes a directory from disk using the standard Pagefind indexing behaviour. This is the same action as running the Pagefind binary with `--source <dir>`. It can be handy to run this indexing step, and then add custom non-HTML records to the index before writing it to disk.

```js
const { errors, page_count } = await index.addDirectory({
    path: "public",
    glob: "**/*.{html}" // optional
});
```

If relative, `path` will be relative to the current working directory of your Node process.

Optionally, a custom `glob` can be supplied, which controls which files Pagefind will consume within the directory. The default is shown, and the `glob` option can be omitted entirely.

A response with an `errors` array containing error messages indicates that Pagefind failed to process this directory.
If successful, `page_count` will be the number of pages that were added to the index.

### index.addHTMLFile

Adds a virtual HTML file to the Pagefind index. Useful for files that don't exist on disk, for example a static site generator that is serving files from memory.

```js
const { errors, file } = await index.addHTMLFile({
    path: "contact/index.html",
    content: "<html><body> <h1>A Full HTML Document</h1> <p> . . . </p> </body></html>"
});
```

The `path` here should represent the output path of this HTML file if it were to exist on disk. Pagefind will use this path to generate the URL.

The `content` should be the full HTML source, including the outer `<html> </html>` tags. This will be run through Pagefind's standard HTML indexing process, and should contain any required Pagefind attributes to control behaviour.

A response with an `errors` array containing error messages indicates that Pagefind failed to index this content.
If successful, the `file` object is returned containing some metadata about the completed indexing.

### index.addCustomRecord

Adds a direct virtual record to the Pagefind index. Useful for adding non-HTML content to the index.

```js
const { errors, file } = await index.addHTMLFile({
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

`meta` is strictly a flat object of keys to string values. See the [Metadata documentation](https://pagefind.app/docs/metadata/) for semantics.

`filters` is strictly a flat object of keys to arrays of string values. See the [Filters documentation](https://pagefind.app/docs/filtering/) for semantics.

`sort` is strictly a flat object of keys to string values. See the [Sort documentation](https://pagefind.app/docs/sorts/) for semantics. *When Pagefind is processing an index, number-like strings will be sorted numerically rather than alphabetically.*

A response with an `errors` array containing error messages indicates that Pagefind failed to index this content.
If successful, the `file` object is returned containing some metadata about the completed indexing.

### index.getFiles

Get buffers of all files in the Pagefind index. Useful for integrating a Pagefind index into the development mode of a static site generator and hosting these files yourself.

```js
const { errors, files } = await index.getFiles();

for (const file of files) {
    console.log(file.path);
    // do something with file.content
}
```

A response with an `errors` array containing error messages indicates that Pagefind failed to action this request.

If successful, `files` will be an array containing file objects. Each object contains a `path` key, which is the URL this file should be served at, and a `content` key containing the raw Buffer of this file.

### index.writeFiles

Writes the index files to disk, as they would be written when running the standard Pagefind binary directly.

```js
const { errors } = await index.writeFiles({
    bundlePath: "./public/pagefind"
});
```

The `bundlePath` option should contain the path to the desired Pagefind bundle directory. If relative, is relative to the current working directory of your Node process.

A response with an `errors` array containing error messages indicates that Pagefind failed to action this request.

### index.deleteIndex

Deletes the data for the given index from the Pagefind binary service. Doesn't affect any written files or Buffers returned by `getFiles()`.

```js
await index.deleteIndex();
```

Calling `index.getFiles()` or `index.writeFiles()` doesn't consume the index, and further modifications can be made. In situations where many indexes are being created, the `deleteIndex` call helps your clear out memory from the Pagefind binary service.

Reusing the `index` object you called this on will cause errors to be returned.

Not calling this method is fine — these indexes will be cleaned up when your Node process exits.
