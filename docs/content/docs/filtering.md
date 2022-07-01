---
date: 2022-06-01
title: "Setting up filters"
nav_title: "Setting up filters"
nav_section: Indexing
weight: 4
---

Pagefind supports filtering your content while searching. This is configured through the `data-pagefind-filter` attribute.

## Tagging an element as a filter

```html
<h1>Hello World</h1>
<p>Author: <span data-pagefind-filter="author">CloudCannon</span></p>
```

An element tagged with `data-pagefind-filter` will associate that page with the filter name, and capture the contents of the element as the filter value. In the above example, the page would be tagged as `author: ["CloudCannon"]`.

Filters can have multiple values per page, so the following is also valid:

```html
<h1>Hello World</h1>
<p>Authors:
    <span data-pagefind-filter="author">CloudCannon</span>
    and
    <span data-pagefind-filter="author">Liam Bigelow</span>
</p>
```

## Tagging an attribute as a filter

If your filter values exists as an attribute, you can use the syntax `filter_name[html_attribute]`

```html
<head>
    <meta 
        data-pagefind-filter="author[content]"
        content="CloudCannon"
        property="og:site_name">
</head>
```

## Tagging a filter inline

If your value doesn't already exist on the page, you can simply use the syntax `filter_name:value`

```html
<h1 data-pagefind-filter="author:CloudCannon">Hello World</h1>
```

## Notes

> The `data-pagefind-filter` attribute does not need to be within the `<body>`, or the `data-pagefind-body` tag. 

> The `data-pagefind-filter` attribute will still apply if set on or within a `data-pagefind-ignore` element.