---
date: 2022-06-01
title: "Searching manually with the Pagefind JavaScript API"
nav_title: "Pagefind JS API"
nav_section: Searching
weight: 6
---

Pagefind can be accessed as an API directly from JavaScript, for you to build custom search interfaces, or integrate with existing systems and components.

## Initializing Pagefind

Anywhere on your site, you can initialize Pagefind with:

```js
const pagefind = await import("/_pagefind/pagefind.js");
```

This will load the Pagefind library and the metadata about the site. If your site is on a subpath, this should be included — e.g. in the CloudCannon documentation, we load `/documentation/_pagefind/pagefind.js`.

> If building your own search UI, it is a good idea to only run this import once your search component has received interaction. This saves the user from loading the Pagefind library on every page request.

## Searching

To perform a search, use the `pagefind` object you initialized above:
```js
const search = await pagefind.search("hello");
```

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

At this point you will have access to the number of search results, and a unique ID for each result.

## Loading a result

To load the data for a result, await the data function:

```js
const oneResult = await search.results[0].data();
```

Which will return an object with the following structure:

```json
{
  "url": "/url-of-the-page/",
  "excerpt": "A small snippet of the <mark>content</mark>, with the <mark>search</mark> term(s) highlighted in mark elements.",
  "filters": {
    "author": "CloudCannon"
  },
  "meta": {
    "title": "The title from the first h1 element on the page",
    "image": "/weka.png"
  },
  "content": "The full content of the page, formatted as text. Cursus Ipsum Risus Ullamcorper...",
  "word_count": 242
}
```

To load a "page" of results, you can run something like the following:

```js
const fiveResults = await Promise.all(search.results.slice(0, 5).map(r => r.data()));
```

## Filtering

To load the available filters, you can run:

```js
const filters = await pagefind.filters();
```

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
```js
const search = await pagefind.search("hello", {
    filters: {
        color: "Orange",
        misc: ["value_one", "value_three"],
    }
});
```

If all filters have been loaded with `await pagefind.filters()`, counts will also be returned alongside each search, detailing the number of remaining items for each filter value:
```js
{ 
    results: [
        {
            id: "6fceec9",
            data: async function data(),
        }
    ],
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
    }
}
```

## Preloading search terms

If you have a debounced search input, Pagefind won't start loading indexes until you run your search query. To speed up your search query when it runs, you can use the `pagefind.preload` function as the user is typing. 

```js
pagefind.preload("h");
```

This function takes the same arguments as the `search` function and downloads the required indexes, stopping short of running the search query. Since indexes are chunked alphabetically, running `pagefind.preload("h")` will likely load the index required to search for `hello` by the time the user has finished typing. Multiple calls to `preload` will not cause redundant network requests.

In vanilla javascript, this might look like the following:

```js
const search = (term) => { /* your main search code */ };
const debouncedSearch = _.debounce(search, 300);

inputElement.addEventListener('input', (e) => {
    pagefind.preload(e.target.value);
    debouncedSearch(e.target.value);
})
```

The `preload` function can also be passed the same filtering options as the `search` function, and will preload any necessary filter indexes.
