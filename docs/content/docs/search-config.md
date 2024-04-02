---
title: "Configuring the Pagefind search in the browser"
nav_title: "Search API config"
nav_section: References
weight: 60
---

The behaviour of the Pagefind search API can be configured in the browser.

## Configuring via Pagefind UI

If using the Pagefind UI, the options object will be passed through to Pagefind:

{{< diffcode >}}
```js
new PagefindUI({
    element: "#search",
+    baseUrl: "/",
+    // ... more search options
});
```
{{< /diffcode >}}

## Configuring via the Pagefind API

If interfacing with Pagefind directly, options can be passed via awaiting `pagefind.options()`:

{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");
+await pagefind.options({
+    baseUrl: "/",
+    // ... more search options
+});
```
{{< /diffcode >}}

## Available options

### Base URL

```json
{
    "baseUrl": "/docs/"
}
```

Defaults to "/". If hosting a site on a subpath, `baseUrl` can be provided, and will be appended to the front of all search result URLs.

### Bundle path

```json
{
    "bundlePath": "/subpath/pagefind/"
}
```

Overrides the bundle directory. In most cases this should be automatically detected by the import URL. Set this if search isn't working and you are seeing a console warning that this path could not be detected.

### Excerpt length

```json
{
    "excerptLength": 15
}
```

Set the maximum length for generated excerpts. Defaults to `30`.

### Highlight query parameter

```json
{
    "highlightParam": "highlight"
}
```

If set, Pagefind will add the search term as a query parameter under the same name. 

If using the [Pagefind highlight script](/docs/highlighting/), make sure this is configured to match.

### Ranking

See [customize ranking](/docs/ranking/)

### Index weight

See [multisite search > weighting](/docs/multisite/#changing-the-weighting-of-individual-indexes)

### Merge filter

See [multisite search > filtering](/docs/multisite/#filtering-results-by-index)
