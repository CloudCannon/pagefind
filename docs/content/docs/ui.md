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
----pagefind-ui-image-border-radius: 8px;
--pagefind-ui-image-box-ratio: 3 / 2;
--pagefind-ui-font: sans-serif;
```

## Styling Pagefind UI yourself

Pagefind UI can be styled manually by omitting the `/_pagefind/pagefind-ui.css` stylesheet. In this case it will function as a pure HTML component.

The classnames within Pagefind UI that begin with `pagefind-ui` should be targeted. These may change, so if you are styling them yourself make sure to test new releases of Pagefind with your stylesheet. Changes to classnames will be highlighted in the release notes, but will not be signalled by a major release.

## PagefindUI options

These options configure Pagefind UI itself. Any extra keys in this object will be passed on to [configure the Pagefind search API](/docs/search-config/).

### Element

```javascript
new PagefindUI({ element: "#search" });
```

A selector for the HTML element to attach Pagefind UI to. This is the only required argument.

### Show images

```javascript
new PagefindUI({
    element: "#search",
    showImages: false
});
```

Whether to show an image alongside each search result. Defaults to `true`.

### Reset styles

```javascript
new PagefindUI({
    element: "#search",
    resetStyles: false
});
```

By default, Pagefind UI applies a CSS reset to itself. Pass `false` to omit this and inherit from your site styles.

### Bundle path

```javascript
new PagefindUI({
    element: "#search",
    bundlePath: "/subpath/_pagefind/"
});
```

Overrides the bundle directory. In most cases this should be automatically detected from the URL of `pagefind-ui.js`. Set this if search isn't working and you are seeing a console warning that this path could not be detected.
