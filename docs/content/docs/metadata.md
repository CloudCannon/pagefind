---
date: 2022-06-01
title: "Returning custom metadata"
nav_title: "Custom metadata"
nav_section: Indexing
weight: 6
---

Pagefind supports returning custom metadata alongside search results with the `data-pagefind-meta` attribute.

## Default metadata

By default, Pagefind will return some metadata about each page:

- `title` will contain the contents of the first `h1` on the page
- `image` will contain the `src` of the first `img` that follows the `h1`

Both of these can be overridden by tagging metadata with those keys.

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

## Tagging metadata inline

If your metadata doesn't already exist on the page, you can simply use the syntax `key:value`

```html
<h1 data-pagefind-meta="date:2022-06-01">Hello World</h1>
```

## Notes

> The `data-pagefind-meta` attribute does not need to be within the `<body>`, or the `data-pagefind-body` tag. 

> The `data-pagefind-meta` attribute will still apply if set on or within a `data-pagefind-ignore` element.