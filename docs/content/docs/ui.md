---
date: 2022-06-01
title: "Using the default Pagefind UI"
nav_title: "Pagefind UI"
nav_section: Searching
weight: 5
---

Pagefind provides a UI component that supports searching, filtering, and metadata out of the box.

## Adding the Pagefind UI to a page

Pagefind UI can be added to any page with the following snippet. The `/_pagefind/` directory and containing files will be created for you when running the Pagefind CLI.

```html
<link href="/_pagefind/pagefind-ui.css" rel="stylesheet">
<script src="/_pagefind/pagefind-ui.js" type="text/javascript"></script>

<div id="search"></div>
<script>
    window.addEventListener('DOMContentLoaded', (event) => {
        new PagefindUI({ element: "#search" });
    });
</script>
```

## Customising the styles

The Pagefind UI is styled using CSS custom properties which can be overridden. To tweak the existing stylesheet, set any of the following variables on your site:

```css
--pagefind-ui-scale: 1;
--pagefind-ui-primary: #034ad8;
--pagefind-ui-text: #393939;
--pagefind-ui-background: #ffffff;
--pagefind-ui-border: #eeeeee;
--pagefind-ui-tag: #eeeeee;
--pagefind-ui-border-width: 2px;
--pagefind-ui-border-radius: 8px;
--pagefind-ui-image-border-radius: 8px;
--pagefind-ui-image-box-ratio: 3 / 2;
--pagefind-ui-font: sans-serif;
```

If your website features a dark/light toggle using a classname, a good idea is to set the colour variables alongside that class. For example, the following snippet will swap Pagefind to a darker theme when the page body contains a `dark` class:

```css
body.dark {
  --pagefind-ui-primary: #eeeeee;
  --pagefind-ui-text: #eeeeee;
  --pagefind-ui-background: #152028;
  --pagefind-ui-border: #152028;
  --pagefind-ui-tag: #152028;
}
```

## Styling Pagefind UI yourself

Pagefind UI can be styled manually by omitting the `/_pagefind/pagefind-ui.css` stylesheet. In this case it will function as a pure HTML component.

The classnames within Pagefind UI that begin with `pagefind-ui` should be targeted. These may change, so if you are styling them yourself make sure to test new releases of Pagefind with your stylesheet. Changes to classnames will be highlighted in the release notes, but will not be signalled by a major release.

## Using custom Pagefind UI strings

Pagefind UI will attempt to use translated text based on the language tag of the active page. If built in tanslations are not found, the UI will fall back to English text. Custom text can instead be supplied using the [translations](#translations) option.

## Overriding the URL of a result

The Pagefind UI will look for a value under the metadata key `url`, and use that for result links if present. This allows you to override the URL of a single page by tagging metadata on that page, for example:

{{< diffcode >}}
```html
<link 
+    data-pagefind-meta="url[href]"
    rel="canonical" 
    href="https://example.com/other-url">
```
{{< /diffcode >}}

## PagefindUI options

These options configure Pagefind UI itself. Any extra keys in this object will be passed on to [configure the Pagefind search API](/docs/search-config/).

### Element

```javascript
new PagefindUI({ element: "#search" });
```

A selector for the HTML element to attach Pagefind UI to. This is the only required argument.

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
+    bundlePath: "/subpath/_pagefind/"
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
