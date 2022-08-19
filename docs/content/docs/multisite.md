---
date: 2022-06-01
title: "Searching multiple sites"
nav_title: "Searching multiple sites"
nav_section: Searching
weight: 80
---

Pagefind can be configured to search across multiple sites in the browser, merging results and filters into a single response. For example, given the websites `blog.example.com` and `docs.example.com`, you may want your search to contain pages from both sources.

## Merging an index in Pagefind UI

When initializing the Pagefind UI, include a `mergeIndex` option with an array of indexes to merge into the main index:

```js
new PagefindUI({
    element: "#search",
    mergeIndex: [{
        url: "https://docs.example.com/_pagefind"
    }]
})
```

## Merging an index in the Pagefind JS API

Using an initialized instance of Pagefind, await the `mergeIndex` function with another bundle path:

```js
const pagefind = await import("/_pagefind/pagefind.js");
await pagefind.mergeIndex("https://docs.example.com/_pagefind");
```

## Merging a specific language index

Pagefind will attempt to grab a matching language when merging an index, falling back to the default language for that index. You can change this behavior by passing a `language` option:

```js
// UI:
new PagefindUI({
    element: "#search",
    mergeIndex: [{
        url: "https://docs.example.com/_pagefind",
        language: "pt-br"
    }]
})

// JS API:
const pagefind = await import("/_pagefind/pagefind.js");
await pagefind.mergeIndex("https://docs.example.com/_pagefind", {
    language: "pt-br"
});
```

## Changing the weighting of individual indexes

When searching across multiple sites you may want to rank each index higher or lower than the others. This can be achieved by passing an `indexWeight` option:

```js
// UI:
new PagefindUI({
    element: "#search",
    mergeIndex: [{
        url: "https://docs.example.com/_pagefind",
        indexWeight: 2
    }]
})

// JS API:
const pagefind = await import("/_pagefind/pagefind.js");
await pagefind.mergeIndex("https://docs.example.com/_pagefind", {
    language: "pt-br",
    indexWeight: 2
});
```

## General behavior

Due to index merging happening in the browser, your additional search indexes must be configured with [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) headers if they are not on the active domain. For example, to configure these headers in a CloudCannon `routing.json` file:

```
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

Merged indexes will be searched using the WebAssembly module from your main instance. This means that merging an index from another language will use the language support from your main Pagefind instance. 
