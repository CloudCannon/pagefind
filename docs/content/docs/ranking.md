---
title: "Customize Pagefind's result ranking"
nav_title: "Customize ranking"
nav_section: Searching
weight: 90
---

Pagefind's default search algorithm is a great choice for most circumstances, but some datasets might be improved by changing the way results are ranked.

A good example is sites with a mix of long and short pages, where the long pages tend to be the preferred result. In this case, tweaking the `pageLength` and/or `termFrequency` parameters can improve the search relevance for the specific content.

Ranking parameters are configured within the `ranking` option passed to Pagefind, which can optionally contain any or all of the available parameters.

## Configuring ranking parameters via the JavaScript API

{{< diffcode >}}
```javascript
const pagefind = await import("/pagefind/pagefind.js");
await pagefind.options({
+    ranking: {
+        termFrequency: 1.0
+    }
});
```
{{< /diffcode >}}

## Configuring ranking parameters via the Default UI

{{< diffcode >}}
```javascript
new PagefindUI({
    element: "#search",
+    ranking: {
+        termFrequency: 1.0
+    }
});
```
{{< /diffcode >}}

## Configuring Term Frequency

{{< diffcode >}}
```javascript
await pagefind.options({
+    ranking: {
+        termFrequency: 1.0 // default value
+    }
});
```
{{< /diffcode >}}

`termFrequency` changes the ranking balance between frequency of the term relative to document length, versus weighted term count.

As an example, if we were querying `search` in the sentence **"Pagefind is a search tool that can search websites"**, the term frequency of `search` is 0.22 (2 / 9 words), while the weighted term count of `search` is 2. This latter number would also include any [content with custom weights](/docs/weighting/).

- The maximum value is `1.0`, where term frequency fully applies and is the main ranking factor.
- The minimum value is `0.0`, where term frequency does not apply, and pages are ranked based on the raw sum of words and weights.
- Values between `0.0` and `1.0` will interpolate between the two ranking methods.

Reducing the `termFrequency` parameter is a good way to boost longer documents in your search results, as they no longer get penalized for having a low term frequency, and instead get promoted for having many instances of the search term.

## Configuring Term Similarity

{{< diffcode >}}
```javascript
await pagefind.options({
+    ranking: {
+        termSimilarity: 1.0 // default value
+    }
});
```
{{< /diffcode >}}

`termSimilarity` changes the ranking based on similarity of terms to the search query. Currently this only takes the length of the term into account.

Increasing this number means pages rank higher when they contain words very close to the query,
e.g. if searching for `part`, a result of `party` will boost a page higher than one containing `partition`.

The minimum value is `0.0`, where `party` and `partition` would be viewed equally.

Increasing the `termSimilarity` parameter is a good way to suppress pages that are ranking well for long extensions of search terms.

## Configuring Page Length

{{< diffcode >}}
```javascript
await pagefind.options({
+    ranking: {
+        pageLength: 0.75 // default value
+    }
});
```
{{< /diffcode >}}

`pageLength` changes the way ranking compares page lengths with the average page lengths on your site.

- The maximum value is `1.0`, where ranking will strongly favour pages that are shorter than the average page on the site, even if longer documents exist with a higher term frequency.
- The minimum value is `0.0`, where ranking will exclusively look at term frequency, regardless of how long a document is.

Decreasing the `pageLength` parameter is a good way to suppress very short pages that are undesirably ranking higher than longer pages.

## Configuring Term Saturation

{{< diffcode >}}
```javascript
await pagefind.options({
+    ranking: {
+        termSaturation: 1.4 // default value
+    }
});
```
{{< /diffcode >}}

`termSaturation` controls how quickly a term "saturates" on a page. Once a term has appeared on a page many times, further appearances have a reduced impact on the page rank.

- The maximum value is `2.0`, where pages will take a long time to saturate, giving pages with very high term frequencies a boost in ranking.
- As this value trends to 0, it does not take many terms to saturate and allow other paramaters to influence the ranking.
- The minimum value is `0.0`, where terms will saturate immediately and results will not distinguish between one term and many.

Decreasing the `termSaturation` parameter is a good way to suppress pages that are ranking well due to an extremely high number of search terms existing in their content.
