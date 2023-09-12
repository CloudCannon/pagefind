---
title: "Showing multiple results per page"
nav_title: "Multiple results per page"
nav_section: Searching
weight: 2
---

Pagefind is able to provide context on which sections of a page match a search term, based on the HTML `id` attributes found on the page.

No configuration is needed when indexing your site. When searching the index, you can choose whether or not to split each page into multiple results.

## Showing sub results with the Default UI

If you are using the Default UI package, set the `showSubResults` option to `true`:

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    showSubResults: true // Defaults to false
});
```
{{< /diffcode >}}

This will split the page on headings (`h1` → `h6`) that have `id` attributes that can be linked to.
A maximum of three sub results will be show per page, and sections with the most hits will be given priority if more than three exist.

## Retrieving sub results using the JavaScript API

If you are using the search API directly, Pagefind will provide calculated sub results within the data for each result:

{{< diffcode >}}
```js
const pagefind = await import("/pagefind/pagefind.js");
const search = await pagefind.search("static");
const oneResult = await search.results[0].data();
```
{{< /diffcode >}}

Which will return an object with the following structure:

{{< diffcode >}}
```js
{
  /* ... other result keys ... */
  "url": "/url-of-the-page/",
  "excerpt": "A small snippet of the <mark>static</mark> content, with the search term(s) highlighted in &lt;mark&gt; elements.",
~  "sub_results": [
~    {
~        /* ... other result keys ... */
~        "title": "The title from the first h1 element on the page",
~        "url": "/url-of-the-page/",
~        "excerpt": "A small snippet of the <mark>static</mark> content, with the search term(s) highlighted in &lt;mark&gt; elements"
~    },
~    {
~        /* ... other result keys ... */
~        "title": "Inner text of some heading",
~        "url": "/url-of-the-page/#id-of-the-h2",
~        "excerpt": "A snippet of the <mark>static</mark> content, scoped between this anchor and the next one",
~        "anchor": { /* ... anchor details ... */ }
~    }
~  ]
}
```
{{< /diffcode >}}

These results are split on headings (`h1` → `h6`) that have `id` attributes that can be linked to.

If there are matches for the search term on the page _before_ the first heading with an ID, the first sub result in this list will be the URL and title of the page itself, and will not contain an `anchor` key. All other sub results will have a URL linking directly to that heading, and will have an `anchor` key with details about the element.

This means that all page results are guaranteed to have at least one sub result for either the page itself, one or more headings, or the page itself **and** one or more headings.

## Calculating custom sub results using the JavaScript API

Pagefind's precalculated sub results are derived when data is loaded in the browser, and this data is also exposed so that consumers of the JavaScript API can calculate sub results that suit their data set better.

Within the data for a page result, the `anchors`, `locations`, and `content` keys provide the data required to construct sub results:

{{< diffcode >}}
```js
{
  /* ... other result keys ... */
  "url": "/url-of-the-page/",
  "excerpt": "A small snippet of the <mark>static</mark> content, with the search term(s) highlighted in &lt;mark&gt; elements.",
~  "content": "The processed text content of this page ...",
~  "locations": [ 4, 18, 70 ],
  "weighted_locations": [
    {
        "weight": 1,
        "location": 4
    },
    {
        "weight": 1,
        "location": 18
    },
    {
        "weight": 2,
        "location": 70
    }
  ],
~  "anchors": [
~    {
~        "element": "h2",
~        "id": "id-of-the-h2",
~        "text": "Inner text of some heading",
~        "location": 14
~    },
~    {
~        "element": "div",
~        "id": "id-of-the-div",
~        "location": 56
~    }
~  ]
}
```
{{< /diffcode >}}

The `anchors` key contains a list of elements on the page that have IDs, and the relative position of that element in the page content. This lists **all** elements, regardless of the search term.

The `locations` key can be cross referenced with the list of `anchors` to determine sub results. In the above example, we know that there are matching words at locations `4`, `18`, and `70`. Looking at our list of anchors, we can see that this reflects a hit before any element IDs, a hit after our `h2#id-of-the-h2`, and a final hit after our `div#id-of-the-div`.

The `content` key can be split on whitespace, and the `locations` will index into this content at the correct positions. This allows you to slice the content for each region of the page if you choose, and to generate a highlighted excerpt using that sliced content.

Also available is the `weighted_locations` list, which can be used to further prioritise sections of the page if they contain higher value words.
