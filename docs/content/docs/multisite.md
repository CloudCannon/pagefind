---
date: 2022-06-01
title: "Searching multiple sites"
nav_title: "Searching multiple sites"
nav_section: Searching
weight: 80
---

Pagefind can be configured to search across multiple sites, merging results and filters into a single response. Multisite search configuration happens entirely in the browser, by pointing one Pagefind instance at multiple search bundles.

The following examples reflect Pagefind running on a website at `blog.example.com` that wants to include pages from `docs.example.com` in the search results. _(Note that in a matching setup across domains, you will need to configure CORS headers. See [Cross origin indexes](#cross-origin-indexes) below.)_

## Searching additional sites from Pagefind UI

When initializing the Pagefind UI, include a `mergeIndex` option with an array of additional indexes to merge into the main index. The URL should be the path to a pagefind bundle folder.

{{< diffcode >}}
```js
// Running on blog.example.com

new PagefindUI({
    element: "#search",
+    mergeIndex: [{
+        bundlePath: "https://docs.example.com/_pagefind"
+    }]
})
```
{{< /diffcode >}}

Pagefind options can be passed to the additional indexes alongside the `bundlePath`:

{{< diffcode >}}
```js
// Running on blog.example.com

new PagefindUI({
    element: "#search",
+    // ... options for the blog.example.com index
    mergeIndex: [{
        bundlePath: "https://docs.example.com/_pagefind",
+        // ... options for the docs.example.com index
    }]
})
```
{{< /diffcode >}}

## Searching additional sites from the Pagefind JS API

Using an initialized instance of Pagefind, await the `mergeIndex` function to add an additional index. The URL should be the path to a pagefind bundle folder.

{{< diffcode >}}
```js
// Running on blog.example.com

const pagefind = await import("/_pagefind/pagefind.js");
+await pagefind.mergeIndex("https://docs.example.com/_pagefind");
```
{{< /diffcode >}}

Pagefind options can be passed to the additional index as a second argument:

{{< diffcode >}}
```js
// Running on blog.example.com

const pagefind = await import("/_pagefind/pagefind.js");
+await pagefind.options({/* ... options for the blog.example.com index */})
await pagefind.mergeIndex(
  "https://docs.example.com/_pagefind",
+  {/* ... options for the docs.example.com index */}
);
```
{{< /diffcode >}}

## Changing the weighting of individual indexes

When searching across multiple sites you may want to rank each index higher or lower than the others. This can be achieved by passing an `indexWeight` option:

{{< diffcode >}}
```js
// UI:
new PagefindUI({
    element: "#search",
    mergeIndex: [{
        bundlePath: "https://docs.example.com/_pagefind",
+        indexWeight: 2
    }]
})

// JS API:
const pagefind = await import("/_pagefind/pagefind.js");
await pagefind.mergeIndex("https://docs.example.com/_pagefind", {
+    indexWeight: 2
});
```
{{< /diffcode >}}

## Merging a specific language index

Pagefind will attempt to grab a matching language when merging an index, falling back to the default language for that index. You can change this behavior by passing a `language` option:

{{< diffcode >}}
```js
// UI:
new PagefindUI({
    element: "#search",
    mergeIndex: [{
        bundlePath: "https://docs.example.com/_pagefind",
+        language: "pt-br"
    }]
})

// JS API:
const pagefind = await import("/_pagefind/pagefind.js");
await pagefind.mergeIndex("https://docs.example.com/_pagefind", {
+    language: "pt-br"
});
```
{{< /diffcode >}}

## Notes

### Cross origin indexes

Due to index merging happening in the browser, your additional search indexes must be configured with [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) headers if they span separate domains. For example, to configure these headers in a CloudCannon `routing.json` file:

```json
{
  "headers": [
    {
      "match": "/_pagefind/.*",
      "headers": [
        {
          "name": "Access-Control-Allow-Origin",
          "value": "*"
        }
      ]
    }
  ]
}
```

### Merging multiple languages

Merged indexes will be searched using the WebAssembly module from your main instance. This means that merging an index from another language will use the language support from your main Pagefind instance. 
