---
title: "Using the Pagefind JavaScript API"
nav_title: "Using the search API"
nav_section: Pagefind JS Reference
weight: 70
---

Pagefind can be accessed as an API directly from JavaScript. This is useful for building custom search interfaces, or integrating with existing systems and components.

## Initializing Pagefind

Anywhere on your site, import and initialize Pagefind with:

```js
const pagefind = await import("/pagefind/pagefind.js");

pagefind.init();
```

If your site is on a subpath, this should be included — e.g. in the CloudCannon documentation, we load `/documentation/pagefind/pagefind.js`.

The call to `pagefind.init()` will load the Pagefind dependencies and the metadata about the site. Calling this method is optional, and if it is omitted initialization will happen when the first searching or filtering function is called.

Calling `pagefind.init()` when your search interface gains focus will help the core dependencies load by the time a user types in their search query.

## Configuring Pagefind

Pagefind options can be set before running `pagefind.init()`:

{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");

+await pagefind.options({
+    bundlePath: "/custom-pagefind-directory/"
+});

pagefind.init();
```
{{< /diffcode >}}

Calls to `pagefind.options` may also be made after initialization, however passing in settings such as `bundlePath` will have no further impact.

See all available options in [Configuring the Pagefind search in the browser](/docs/search-config/).

## Searching

To perform a search, await `pagefind.search`:
{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");
+const search = await pagefind.search("static");
```
{{< /diffcode >}}

This will return an object with the following structure:
```js
{ 
    results: [
        {
            id: "6fceec9",
            data: async function data()
        }
    ]
}
```

At this point you will have access to the number of search results, and a unique ID for each result. Also see [Debounced search](#debounced-search) below for an alternative API.

## Loading a result

To load the data for a result, await the data function:

{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");
const search = await pagefind.search("static");
+const oneResult = await search.results[0].data();
```
{{< /diffcode >}}

Which will return an object with the following structure:

```json
{
  "url": "/url-of-the-page/",
  "excerpt": "A small snippet of the <mark>static</mark> content, with the search term(s) highlighted in &lt;mark&gt; elements.",
  "filters": {
    "author": "CloudCannon"
  },
  "meta": {
    "title": "The title from the first h1 element on the page",
    "image": "/weka.png"
  },
  "content": "The full content of the page, formatted as text. <html> will not be escaped. ...",
  "word_count": 242,
  "sub_results": [
    {
        "title": "The title from the first h1 element on the page",
        "url": "/url-of-the-page/",
        "locations": [ /* ... */ ],
        "weighted_locations": [ /* ... */ ],
        "excerpt": "A small snippet of the <mark>static</mark> content, with the search term(s) highlighted in &lt;mark&gt; elements",
    },
    {
        "title": "Inner text of some heading",
        "url": "/url-of-the-page/#id-of-the-h2",
        "locations": [ /* ... */ ],
        "weighted_locations": [ /* ... */ ],
        "excerpt": "A snippet of the <mark>static</mark> content, scoped between this anchor and the next one",
        "anchor": {
            "element": "h2",
            "id": "id-of-the-h2",
            "text": "Inner text of some heading",
            "location": 14
        }
    }
  ],
  "locations": [ 4, 9, 18 ],
  "weighted_locations": [
    {
        "weight": 1,
        "balanced_score": 10.5,
        "location": 4
    },
    {
        "weight": 1,
        "balanced_score": 15,
        "location": 9
    },
    {
        "weight": 2,
        "balanced_score": 46.7,
        "location": 18
    }
  ],
  "anchors": [
    {
        "element": "h2",
        "id": "id-of-the-h2",
        "text": "Inner text of some heading",
        "location": 14
    },
    {
        "element": "div",
        "id": "id-of-the-div",
        "location": 56
    }
  ]
}
```

> Note that `excerpt` will have HTML entities encoded before adding `<mark>` elements, so is safe to use as innerHTML. The `content` key is raw and unprocessed, so will need to be escaped by the user if necessary.

To load a "page" of results, you can run something like the following:

{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");
const search = await pagefind.search("static");
+const fiveResults = await Promise.all(search.results.slice(0, 5).map(r => r.data()));
```
{{< /diffcode >}}

## Filtering

To load the available filters, you can run:

{{< diffcode >}}
```js
const filters = await pagefind.filters();
```
{{< /diffcode >}}

This will return an object of the following structure, showing the number of search results available under the given `filter: value` combination.
```json
{
    "misc": {
        "value_one": 4,
        "value_two": 12,
        "value_three": 3
    },
    "color": {
        "Orange": 6,
        "Red": 2
    }
}
```

To filter results alongside searching, pass an options object to the search function. Filter values can be passed as strings or arrays.
{{< diffcode >}}
```js
const search = await pagefind.search("static", {
+    filters: {
+        color: "Orange",
+        misc: ["value_one", "value_three"],
+    }
});
```
{{< /diffcode >}}

If all filters have been loaded with `await pagefind.filters()`, counts will also be returned alongside each search, detailing the number of remaining items for each filter value. 

- The `filters` key contains the number of results if a given filter were to be applied in addition to the current filters.
- The `totalFilters` key contains the number of results if a given filter were to be applied instead of the current filters.
- The `unfilteredResultCount` key details the number of results for the search term alone, if no filters had been applied.

```js
{ 
    results: [
        {
            id: "6fceec9",
            data: async function data(),
        }
    ],
    unfilteredResultCount: 100,
    filters: {
        "filter": {
            "value_one": 4,
            "value_two": 0,
            "value_three": 2
        },
        "color": {
            "Orange": 1,
            "Red": 0
        }
    },
    totalFilters: {
        "filter": {
            "value_one": 4,
            "value_two": 10,
            "value_three": 2
        },
        "color": {
            "Orange": 4,
            "Red": 2
        }
    }
}
```

## Sorting results

If pages on your site have been tagged with [sort attributes](/docs/sorts/), a `sort` object can be provided to Pagefind when searching:

{{< diffcode >}}
```js
const search = await pagefind.search("static", {
+    sort: {
+        date: "asc"
+    }
});
```
{{< /diffcode >}}

This object should contain one key, matching a `data-pagefind-sort` attribute, and specify either `asc` for ascending or `desc` for descending sort order.

This will override any page relevance sorting, and will return all matching results sorted by the given attribute.

## Filtering without search

If the search term passed to Pagefind is the value `null`, Pagefind will return all results. For example, the following snippet will return all pages in the index, sorted by their date.

{{< diffcode >}}
```js
const search = await pagefind.search(null, {
    sort: {
        date: "asc"
    }
});
```
{{< /diffcode >}}

Filters will still be applied, allowing Pagefind to be used as a filtering tool instead of a searching tool:

{{< diffcode >}}
```js
const search = await pagefind.search(null, {
    filters: {
        category: "Posts"
    },
    sort: {
        date: "asc"
    }
});
```
{{< /diffcode >}}

## Debounced search

The helper function `pagefind.debouncedSearch` is available and can be used in place of `pagefind.search`:
{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");
+const search = await pagefind.debouncedSearch("static");
```
{{< /diffcode >}}

A custom debounce timeout (default: `300`) can optionally be specified as the third argument:
{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");
+const search = await pagefind.debouncedSearch("static", {/* options */}, 300);
```
{{< /diffcode >}}

This function waits for the specified duration, and then either performs the search, or returns null if a subsequent call to `pagefind.debouncedSearch` has been made. This helps with resource usage when processing large searches, and can help with flickering when rendering results in a UI.

{{< diffcode >}}
```js
const search = await pagefind.debouncedSearch("static");
+if (search === null) {
+  // a more recent search call has been made, nothing to do
+} else {
+  process(search.results);
+}
```
{{< /diffcode >}}

## Preloading search terms

If you have a debounced search input, Pagefind won't start loading indexes until you run your search query. To speed up your search query when it runs, you can use the `pagefind.preload` function as the user is typing. Note that the [Debounced search](#debounced-search) helper provided by Pagefind implements this for you under the hood.

{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");
+pagefind.preload("s");

// later...
await pagefind.search("static");
```
{{< /diffcode >}}

This function takes the same arguments as the `search` function and downloads the required indexes, stopping short of running the search query. Since indexes are chunked alphabetically, running `pagefind.preload("s")` will likely load the index required to search for `static` by the time the user has finished typing. Multiple calls to `preload` will not cause redundant network requests.

In vanilla javascript, this might look like the following:

{{< diffcode >}}
```js
const search = (term) => { /* your main search code */ };
const debouncedSearch = _.debounce(search, 300);

inputElement.addEventListener('input', (e) => {
+    pagefind.preload(e.target.value);
    debouncedSearch(e.target.value);
})
```
{{< /diffcode >}}

The `preload` function can also be passed the same filtering options as the `search` function, and will preload any necessary filter indexes.
