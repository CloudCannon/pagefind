---
title: "Weighting sections of the page higher or lower"
nav_title: "Weighting content"
nav_section: Indexing
weight: 3
---

Content in Pagefind's index can be ranked higher or lower. These weights will be taken into account when Pagefind ranks your search results, and will also be used when generating excerpts of your content to preview.

## Default rankings

Pagefind will boost the `h1` through `h6` tags above any other content on the page. By default, content is ranked as:

| Element            | Ranking |
|--------------------|---------|
| `h1`               | 7.0     |
| `h2`               | 6.0     |
| `h3`               | 5.0     |
| `h4`               | 4.0     |
| `h5`               | 3.0     |
| `h6`               | 2.0     |
| All other elements | 1.0     |

## Ranking content higher or lower

You can specify your own ranking via the `data-pagefind-weight` attribute:

```html
<body>
    <p data-pagefind-weight="2">
        The main description text of the page.
        If the search term matches this section,
        this page will be boosted higher in the
        result ranking.
    </p>
    <p>
        Other, less important text.
        This defaults to a weight of 1.
    </p>
    <p data-pagefind-weight="0.5">
        Very unimportant text.
        Matching words in this block are only worth half a normal word.
    </p>
</body>
```

Custom weights can be set to any number between `0.0` and `10.0`. 
