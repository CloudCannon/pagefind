---
title: "Filtering using the Pagefind JavaScript API"
nav_title: "Filtering with the JS API"
nav_section: Filtering
weight: 12
---

Pagefind's JavaScript API supports filtering your content, either as part of a search or as a standalone action. This is often useful when using Pagefind for a knowledge base or large blog, where filtering by categories or tags might be the primary action.


## Fetching the available filters

To load the available filters, run:

{{< diffcode >}}
```js
const filters = await pagefind.filters();
```
{{< /diffcode >}}

Filters are not initialized by default, so this function call will load the required filtering files from the index.

This function returns an object of the following structure, showing the number of total results available under each `filter: value` combination.
```json
{
    "tag": {
        "Documentation": 4,
        "Article": 12
    },
    "author": {
        "CloudCannon": 6,
        "Liam Bigelow": 12,
        "Pagefind": 1
    }
}
```

## Filtering as part of a search

To filter results alongside searching, pass an options object containing `filters` to the search function:

{{< diffcode >}}
```js
const search = await pagefind.search(
    "static",
+    {
+        filters: { author: "CloudCannon" }
+    }
);
```
{{< /diffcode >}}

This example will return all pages:
- With search hits for the word `static`
- With `author` filters associated, where one of the authors is exactly `CloudCannon`

## Filtering as a standalone action

To filter results without searching, pass a search term of `null` to the search function:

{{< diffcode >}}
```js
const search = await pagefind.search(
+    null,
    {
        filters: { author: "CloudCannon" }
    }
);
```
{{< /diffcode >}}

This example will return all pages with `author` filters associated, where one of the authors is exactly `CloudCannon`.

## Getting the remaining results available for each filter

If all filters have been loaded with `await pagefind.filters()`, counts will also be returned alongside each search, detailing the number of remaining items for each filter value. 

```js
{ 
    "results": [
        {
            "id": "6fceec9",
            "data": async function data(),
        }
    ],
    "unfilteredResultCount": 100,
    "filters": {
        "tag": {
            "Documentation": 1,
            "Article": 0
        },
        "author": {
            "CloudCannon": 4,
            "Liam Bigelow": 0,
            "Pagefind": 2
        }
    },
    "totalFilters": {
        "tag": {
            "Documentation": 4,
            "Article": 2
        },
        "author": {
            "CloudCannon": 4,
            "Liam Bigelow": 10,
            "Pagefind": 2
        }
    }
}
```

- The `filters` key contains the number of results if a given filter were to be applied in addition to the current filters.
- The `totalFilters` key contains the number of results if a given filter were to be applied instead of the current filters.
- The `unfilteredResultCount` key details the number of results for the search term alone, if no filters had been applied.

## Using compound filters

When unspecified, all filtering defaults to "AND" filtering. This means that pages must match every filter provided. For example:

{{< diffcode >}}
```js
const search = await pagefind.search("static", {
+    filters: {
+        author: ["CloudCannon", "Pagefind"],
+        tag: "Article"
+    },
});
```
{{< /diffcode >}}

This query will only match pages that have a `tag` of `Article`, **and** are authored by **both** `CloudCannon`, and `Pagefind`.

This behavior can be customized by using the keyword `any`, `all`, `none`, and `not` in your filtering options:
- `all` will match if **every** nested condition matches. Untagged conditions will default to `all`.
- `not` is the opposite of `all` and will match unless **every** nested condition matches.
- `any` will match if **one or more** of the nested conditions match.
- `none` is the opposite of `any` and will match unless **one or more** of the nested conditions match.

The syntax for filtering is very flexible, and these keywords may be nested within each other, and within arrays or objects.

### Examples:

{{< diffcode >}}
```js
const search = await pagefind.search("static", {
    filters: {
        author: {
            any: ["CloudCannon", "Pagefind"]
        },
        tag: "Article"
    },
});
```
{{< /diffcode >}}

Matches pages with a tag of `Article`, with an author of either `CloudCannon` or `Pagefind`.

***

{{< diffcode >}}
```js
const search = await pagefind.search("static", {
    filters: {
        any: {
            author: ["CloudCannon", "Pagefind"],
            tag: "Article"
        }
    },
});
```
{{< /diffcode >}}

Matches pages with a tag of `Article`, or pages authored by both `CloudCannon` and `Pagefind`.

***

{{< diffcode >}}
```js
const search = await pagefind.search("static", {
    filters: {
        any: [{
            author: "Pagefind",
            tag: "Article"
        }, {
            author: "CloudCannon",
            tag: "Documentation"
        }],
        not: {
            year: "2018"
        }
    },
});
```
{{< /diffcode >}}

Matches pages that are authored by `Pagefind` with a tag of `Article`, or pages authored by `CloudCannon` with a tag of `Documentation`, but does not match any of those pages if they have a year of `2018`.

***

To dive deeper into complex filtering, see the [compound_filtering.feature](https://github.com/CloudCannon/pagefind/blob/main/pagefind/features/compound_filtering.feature) test file in Pagefind's GitHub repository.
