---
date: 2022-06-01
title: "Configuring the Pagefind search in the browser"
nav_title: "Pagefind search config"
nav_section: Searching
weight: 7
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

If interfacing with Pagefind directly, options can be passed via `pagefind.options()`:

{{< diffcode >}}
```js
const pagefind = await import("/_pagefind/pagefind.js");
+pagefind.options({
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
    "bundlePath": "/subpath/_pagefind/"
}
```

Overrides the bundle directory. In most cases this should be automatically detected by the import URL. Set this if search isn't working and you are seeing a console warning that this path could not be detected.
