---
title: "Highlighting search terms"
nav_title: "Highlighting search terms"
nav_section: Searching
weight: 10
---

Pagefind includes the ability to highlight search terms on the result page.

To enable this feature, first configure Pagefind to insert a query parameter on search results.

## Configuring highlighting via the Default UI

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    highlightParam: "highlight"
});
```
{{< /diffcode >}}

## Configuring highlighting via the JavaScript API

{{< diffcode >}}
```javascript
const pagefind = await import("/pagefind/pagefind.js");
await pagefind.options({
+    highlightParam: "highlight"
});
const search = await pagefind.search("static");
```
{{< /diffcode >}}

## Enabling highlights on result pages

Once Pagefind is configured to insert query parameters, pages on your site will need to opt-in to highlighting.
This is something you can implement for your own site by looking at the query parameter, but Pagefind provides a highlighting script for convenience.

To opt-in, import `/pagefind/pagefind-highlight.js` on all pages of your site and create a new `PagefindHighlight` object.

```html
<script type="module">
    await import('/pagefind/pagefind-highlight.js');
    new PagefindHighlight({ highlightParam: "highlight" });
</script>
```

Ensure that the `highlightParam` configured here matches the `highlightParam` given to Pagefind when searching.

To see all options available to PagefindHighlight, see [Highlight Config](/docs/highlight-config).
