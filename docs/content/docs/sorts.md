---
title: "Setting up sorting"
nav_title: "Setting up sorting"
nav_section: Sorting
weight: 20
---

Pagefind supports sorting results by tagged attributes instead of page relevancy. For a sort to be available in the browser, pages must be tagged with the `data-pagefind-sort` attribute.

## Capturing a sort value from an element

```html
<p data-pagefind-sort="date">2022-10-20</p>
```

An element tagged with `data-pagefind-sort` will capture the contents of that element and provide the given key as a sort option to the Pagefind JS API. In the above example, the page would be tagged as `date: "2022-10-20"`.

## Capturing a sort value from an attribute

If the data you want to sort by exists as an attribute, you can use the syntax `sort_key[html_attribute]`

```html
<h1 data-pagefind-sort="weight[data-weight]" data-weight="10">Hello World</h1>
```

This will capture the filter value from the attribute specified, in this case producing `weight: "10"`.

> See the [Notes](#notes) section at the bottom of this page for details on how number-like values are sorted.

## Specifying a sort inline

If your value doesn't already exist on the page, you can use the syntax `sort_key:value`:

```html
<h1 data-pagefind-sort="date:2022-06-01">Hello World</h1>
```

This will tag this page as `date: 2022-06-01`. The element this is set on does not matter, meaning this attribute can be located anywhere that is convenient in your site templating.

## Specifying multiple sorts on a single element

Sort captures may be comma separated and all will apply. The exception is specifying a sort value inline, which may only be the last item in a list.

For example:

{{< diffcode >}}
```html
<h1
    data-weight="10"
    data-date="2022-06-01"
+    data-pagefind-sort="heading, weight[data-weight], date[data-weight], author:Freeform text, captured to the end">
        Hello World
</h1>
```
{{< /diffcode >}}

This will produce the sort tags for the page:

```json
{
    "heading": "Hello World",
    "weight": "10",
    "date": "2022-06-01",
    "author": "Freeform text, captured to the end"
}
```

## Notes

> If all values tagged by a given sort key can be parsed as numbers (integers or floats) then Pagefind will sort them numerically. If any values are not parsable, all values will be sorted alphabetically. 

> Pages that omit a `data-pagefind-sort` tag for a given sorting key will be omitted from search results if that sort is applied. i.e. if a site has four pages, and three are tagged `data-pagefind-sort="date"`, sorting your search results by `date` will return three total results.

> Sort orders are precomputed while indexing the site. Due to this, if you are using the [Multisite feature](/docs/multisite/) sorting will not be fully correct. Searching across multiple indexes with a sort applied will first sort each index, and then zip them together, providing interlaced results from each index.
