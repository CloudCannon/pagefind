---
title: "Pagefind UI configuration options"
nav_title: "UI config options"
nav_section: Pagefind UI Reference
weight: 61
---

These options configure Pagefind UI itself. Any extra keys in this object will be passed on to [configure the Pagefind search API](/docs/search-config/).

### Element

```javascript
new PagefindUI({ element: "#search" });
```

A selector for the HTML element to attach Pagefind UI to. This is the only required argument.

### Show sub results

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    showSubResults: true
});
```
{{< /diffcode >}}

Whether to show nested results for each heading within a matching page. Defaults to `false`.  
If `true`, a maximum of three will be shown per result.

### Show images

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    showImages: false
});
```
{{< /diffcode >}}

Whether to show an image alongside each search result. Defaults to `true`.

### Excerpt length

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    excerptLength: 15
});
```
{{< /diffcode >}}

Set the maximum length for generated excerpts. Defaults to `30`, or `12` if showing sub results.

### Process term

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    processTerm: function (term) {
+        return term.replace(/aa/g, 'ā');
+    }
});
```
{{< /diffcode >}}

Provides a function that Pagefind UI calls before performing a search. This can be used to normalize search terms to match your content. The result will not be shown to the user, in the above example the search input would still display `aa`. 


### Process result

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    processResult: function (result) {
+        result.meta.image = someCustomFunction(result.meta.image);
+        return result;
+    }
});
```
{{< /diffcode >}}

Provides a function that Pagefind UI calls before displaying each result. This can be used to fix relative URLs, rewrite titles, or any other modifications you might like to make to the raw result object returned by Pagefind. 

### Show empty filters

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    showEmptyFilters: true
});
```
{{< /diffcode >}}

By default, Pagefind UI shows filters with no results alongside the count (0). Pass `false` to hide filters that have no remaining results.

### Reset styles

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    resetStyles: false
});
```
{{< /diffcode >}}

By default, Pagefind UI applies a CSS reset to itself. Pass `false` to omit this and inherit from your site styles.

### Bundle path

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    bundlePath: "/subpath/pagefind/"
});
```
{{< /diffcode >}}

Overrides the bundle directory. In most cases this should be automatically detected from the URL of `pagefind-ui.js`. Set this if search isn't working and you are seeing a console warning that this path could not be detected.

### Debounce user input

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    debounceTimeoutMs: 500
});
```
{{< /diffcode >}}

The number of milliseconds to wait after a user stops typing before performing a search. Defaults to `300`. If you wish to disable this, set to `0`.

### Translations

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    translations: {
+        placeholder: "Search my website",
+        zero_results: "Couldn't find [SEARCH_TERM]"
+    }
});
```
{{< /diffcode >}}

A set of custom ui strings to use instead of the automatically detected language strings. See the [translations/en.json](https://github.com/CloudCannon/pagefind/blob/main/pagefind_ui/translations/en.json) file for all available keys and their initial values.

The items in square brackets such as `SEARCH_TERM` will be substituted dynamically when the text is used.
