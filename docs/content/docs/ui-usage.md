---
title: "Using the Default UI"
nav_title: "Using the Default UI"
nav_section: Searching
weight: 3
---

Pagefind provides a Default UI component that supports searching, basic filtering, sub results, and metadata out of the box.

## Adding the Pagefind UI to a page

Pagefind UI can be added to any page with the following snippet. The `/pagefind/` directory and containing files will be created for you when running the Pagefind CLI.

```html
<link href="/pagefind/pagefind-ui.css" rel="stylesheet">
<script src="/pagefind/pagefind-ui.js"></script>

<div id="search"></div>
<script>
    window.addEventListener('DOMContentLoaded', (event) => {
        new PagefindUI({ element: "#search", showSubResults: true });
    });
</script>
```

This snippet is combined here for brevity, but feel free to move the JS & CSS assets alongside your existing assets, and place the `new PagefindUI` initialization script inside an existing JS file.

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

Pagefind UI can be styled manually by omitting the `/pagefind/pagefind-ui.css` stylesheet. In this case it will function as a pure HTML component.

The classnames within Pagefind UI that begin with `pagefind-ui` should be targeted. These may change, so if you are styling them yourself make sure to test new releases of Pagefind with your stylesheet. Any significant changes to this markup will be noted in a changelog.

## Using custom Pagefind UI strings

Pagefind UI will attempt to use translated text based on the language tag of the active page. If built in translations are not found, the UI will fall back to English text. Custom text can instead be supplied using the [translations](/docs/ui/#translations) option.

Languages with built in translations are listed in the [language support table](/docs/multilingual/#language-support).

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

## Overriding the title or image of a result

The Pagefind UI will look for values under the metadata keys `title`, `image`, and `image_alt`. This allows you to override these details by tagging metadata on that page, for example:

{{< diffcode >}}
```html
<h1
+    data-pagefind-meta="title">
    Hello World
</h1>
<img 
+    data-pagefind-meta="image[src], image_alt[alt]"
    src="/hero.png"
    alt="Hero Alt Text" />
```
{{< /diffcode >}}

## Re-initializing the Pagefind UI

In some cases you might need to re-initialize Pagefind. For example, if you dynamically change the language of the page without reloading, Pagefind will need to be re-initialized to reflect this langauge change.

Pagefind UI can be destroyed by running `.destroy()` on the returned object. Doing so will also tear down the initialized Pagefind instance:

{{< diffcode >}}
```js
let search = new PagefindUI({ element: "#search", showSubResults: true });
+search.destroy();
+search = new PagefindUI({ element: "#search", /* new options */ });
```
{{< /diffcode >}}

After being destroyed, initializing the Pagefind UI will look again at the active language, and use any new options you might pass in.

## Further customization

See the [Pagefind UI Configuration Reference](/docs/ui/) for all available options.
