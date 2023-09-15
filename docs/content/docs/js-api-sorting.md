---
title: "Sorting using the Pagefind JavaScript API"
nav_title: "Sorting with the JS API"
nav_section: Sorting
weight: 21
---

Pagefind's JavaScript API supports sorting your content when searching. Doing so will override the default rankings, and will return all matching results sorted by the given attribute.

## Sorting as part of a search

If pages on your site have been tagged with [sort attributes](/docs/sorts/), a `sort` object can be provided to Pagefind when searching:

{{< diffcode >}}
```js
const search = await pagefind.search("static", {
+    sort: {
+        date: "asc"
+    }
});
```
{{< /diffcode >}}

This object should contain one key, matching a `data-pagefind-sort` attribute, and specify either `asc` for ascending or `desc` for descending sort order.
