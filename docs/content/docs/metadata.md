---
date: 2022-06-01
title: "Returning custom metadata"
nav_title: "Custom metadata"
nav_section: Indexing
weight: 6
---

Pagefind supports returning custom metadata alongside search results with the `data-pagefind-meta` attribute.

## Automatic metadata

Pagefind will return some automatic metadata about each page:

- `title` will contain the contents of the first `h1` on the page
- `image` will contain the `src` of the first `img` that follows the `h1`
- `image_alt` will contain the `alt` of the first `img` that follows the `h1`

All of these can be overridden by tagging metadata with the same keys.

## Tagging an element as metadata

```html
<h1 data-pagefind-meta="title">Hello World</h1>
```

An element tagged with `data-pagefind-meta` will store the contents of that element and return it alongside the search results.

Each metadata key can only have one value per page.

## Tagging an attribute as metadata

If your metadata exists as an attribute, you can use the syntax `key[html_attribute]`

```html
<img data-pagefind-meta="image[src]" src="/hero.png" />
```

You can comma separate multiple meta attributes:

```html
<img data-pagefind-meta="image[src], image_alt[alt]" src="/hero.png" alt="Hero Alt Text" />
```

## Tagging metadata inline

If your metadata doesn't already exist on the page, you can simply use the syntax `key:value`

```html
<h1 data-pagefind-meta="date:2022-06-01">Hello World</h1>
```

## Defining multiple metadata keys on a single element

You can comma separate multiple metadata keys, with the exception of inline metadata, which may only be the last item in a list.

Usage:

```html
<a href="/" 
   title="Homepage"
   data-pagefind-meta="link_text, link_title[title], other:Freeform text, captured to the end">

   Hello World
</a>
```

This will generate the metadata:

```json
{
    "link_text": "Hello World",
    "link_title": "Homepage",
    "other": "Freeform text, captured to the end"
}
```

## Defining default metadata

All of the above tags can also be supplied as a `data-pagefind-default-meta` attribute. All logic is the same, except that automatic metadata and any `data-pagefind-meta` attributes will take priority.

For example, to fall back to a social image if no image is found on the page:

```html
<head>
    <meta data-pagefind-default-meta="image[content]" content="/social.png" property="og:image">
</head>
```

## Notes

> The `data-pagefind-meta` attribute does not need to be within the `<body>`, or the `data-pagefind-body` tag. This includes automatic metadata, which will be found even if outside the `data-pagefind-body` tag.

> The `data-pagefind-meta` attribute will still apply if set on or within a `data-pagefind-ignore` element.

> `image_alt` will not be automatically set if you define your own `image` metadata key. If defining your own metadata on an `img` element, `data-pagefind-meta="image[src], image_alt[alt]"` will retrieve both values.
