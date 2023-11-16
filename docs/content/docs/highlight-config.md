---
title: "Highlight API Config"
nav_title: "Highlight API Config"
nav_section: References
weight: 65

---

`PagefindHighlight` accepts an object with any of the following options:

### markContext

```js
new PagefindHighlight({ markContext: "[data-pagefind-body]" })
```

The area in which to highlight text. Defaults to `[data-pagefind-body]` if any `[data-pagefind-body]` elements can be found, otherwise `document.body`. This can be a CSS selector string, a DOM element, an array of DOM elements, or a NodeList.

### highlightParam

```js
new PagefindHighlight({ highlightParam: "highlight" })
```

The name of the query parameter that Pagefind uses to determine which terms to highlight. This must match the [Pagefind highlightParam config](/docs/search-config/#highlight-query-parameter).

### markOptions

An object with the same shape as the [Mark.js options](https://markjs.io/#mark) object, except that the `separateWordSearch` option is not supported.

This option defaults to:

```js
{
  className: "pagefind-highlight",
  exclude: ["[data-pagefind-ignore]", "[data-pagefind-ignore] *"],
}
```

If either `className` or exclude are not specified they will default to the above. To disable the default `exclude` behavior, pass an empty array to `exclude`. To not add a class to the highlight elements, pass an empty string to `className`.

### addStyles

This is a boolean that determines whether or not Pagefind will add minimal styles to the highlighted elements. If set to `false`, Pagefind will not add any styles to the page. This option defaults to `true`.
