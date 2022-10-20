---
date: 2022-06-01
title: "Custom sort orders"
nav_title: "Custom sort orders"
nav_section: Indexing
weight: 90
---

For users interacting with the Pagefind JS API directly, Pagefind supports sorting results by tagged attributes instead of page relevancy. For a sort to be available in the browser, pages must be tagged with the `data-pagefind-sort` attribute.

## Sorting pages by tagged elements

```html
<p data-pagefind-sort="date">2022-10-20</p>
```

An element tagged with `data-pagefind-sort` will capture the contents of that element and provide the given key as a sort option to the Pagefind JS API.

## Sorting pages by tagged attributes

If your sort value exists as an attribute, you can use the syntax `key[html_attribute]`

```html
<h1 data-pagefind-sort="weight[data-weight]" data-weight="10">Hello World</h1>
```

## Sorting pages by tagged values

If your sort value doesn't already exist on the page, you can simply use the syntax `key:value`

```html
<h1 data-pagefind-sort="date:2022-06-01">Hello World</h1>
```

## Advanced tagging

The sort syntax follows the same rules as the metadata syntax, see [Defining multiple metadata keys on a single element](/docs/metadata/#defining-multiple-metadata-keys-on-a-single-element) for more detail.

## Notes

> If all values tagged by a given sort key can be parsed as numbers (integers or floats) then Pagefind will sort them numerically. If any values are not parsable, all values will be sorted alphabetically. 

> Pages that omit a `data-pagefind-sort` tag for a given sorting key will be omitted from search results if that sort is applied. i.e. if a site has four pages, and three are tagged `data-pagefind-sort="date"`, sorting your search results by `date` will return three total results.

> Sort orders are precomputed while indexing the site. Due to this, if you are using the [Multisite feature](/docs/multisite/) sorting will not be fully correct. Searching across multiple indexes with a sort applied will first sort each index, and then zip them together, providing interlaced results from each index.
