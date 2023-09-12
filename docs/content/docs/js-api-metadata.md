---
title: "Getting metadata with the Pagefind JavaScript API"
nav_title: "Metadata in the JS API"
nav_section: Metadata
weight: 12
---

Pagefind's JavaScript API returns the metadata of your pages automatically alongside your search result data.

## Getting metadata from a search result

{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");
const search = await pagefind.search("static");
+const oneResult = await search.results[0].data();
```
{{< /diffcode >}}

Here, `oneResult` will contain:

{{< diffcode >}}
```js
{
  /* ... other result keys ... */
  "url": "/url-of-the-page/",
  "excerpt": "A small snippet of the <mark>static</mark> content, with the search term(s) highlighted in &lt;mark&gt; elements.",
~  "meta": {
~    "title": "The title from the first h1 element on the page",
~    "image": "/weka.png",
~    "my-custom-key": "My custom metadata content",
~  }
}
```
{{< /diffcode >}}