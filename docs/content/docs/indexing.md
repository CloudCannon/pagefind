---
title: "Configuring what content is indexed"
nav_title: "Customizing the index"
nav_section: Indexing
weight: 2
---

You can control what content on your site is indexed via `data-pagefind-*` tags on your HTML elements.

## Limiting what sections of a page are indexed

By default, Pagefind starts indexing from your `<body>` element.

To narrow this down, you can tag your main content area with `data-pagefind-body`:

```html
<body>
    <main data-pagefind-body>
        <h1>Condimentum Nullam</h1>
        <p>Nullam id dolor id nibh ultricies.</p>
    </main>
    <aside>
        This content will not be indexed.
    </aside>
</body>
```

If `data-pagefind-body` is found anywhere on your site, any pages without this attribute will not be indexed. This means that if you tag your blog post layout with `data-pagefind-body`, other pages like your homepage will no longer appear in search results. This is usually what you want — if not, just add `data-pagefind-body` there as well.

Multiple `data-pagefind-body` elements may exist on a page, and their content will be combined.

> Note that metadata and filters that are set outside of this element will still be used. If this is not what you want, see the [root selector](/docs/config-options/#root-selector) configuration option.

## Removing pages from Pagefind's index

Once a `data-pagefind-body` attribute exists on any page of your site, any pages without this attribute will not be indexed. As such, the best way to remove pages is by adding `data-pagefind-body` to the pages you **would** like to index.

If this isn't possible, see the [Pagefind CLI's glob option](/docs/config-options/#glob) to limit the files that Pagefind reads.

## Removing individual elements from the index

Pagefind has some built-in elements that are not indexed. These are organizational elements such as `<nav>` and `<footer>`, or more programmatic elements such as `<script>` and `<form>`. These elements will be skipped over automatically.

If you have further elements that you don't want to include in your search index, you can tag them with `data-pagefind-ignore`:

```html
<main data-pagefind-body>
    <h1>Condimentum Nullam</h1>
    <aside data-pagefind-ignore>
        This content will not be indexed.
    </aside>
    <p>Nullam id dolor id nibh ultricies.</p>
</main>
```

The `data-pagefind-ignore` attribute can optionally take a value of `index` or `all`. Omitting a value implies `index`, which will exclude the element and all children from the search index while still processing filters and metadata within the element, and will still try to detect a default title or image found within this element.

Specifying `all` will exclude the element and its children from all processing.

```html
<aside data-pagefind-ignore>
    <h1>This might still be detected as the page title</h1>
    <p data-pagefind-meta="a">This metadata will still appear in search results.</p>
</aside>

<aside data-pagefind-ignore="all">
    <h1>This cannot be detected as the page title</h1>
    <p data-pagefind-meta="b">This metadata will not be processed.</p>
</aside>
```

> To remove elements without changing your templating, see the [Pagefind CLI's exclude selectors option](/docs/config-options/#exclude-selectors).

## Adding HTML attributes to the index

Attributes of HTML elements can be added to the search index with the `data-pagefind-index-attrs` attribute:

```html
<h1>Condimentum Nullam</h1>
<img src="/hero.png"
     title="Image Title"
     alt="Image Alt"
     data-pagefind-index-attrs="title,alt" />
<p>Nullam id dolor id nibh ultricies.</p>
```

This attribute takes a comma-separated list of other attributes to include inline with the indexed content.  
The above example will be indexed as: `Condimentum Nullam. Image Title. Image Alt. Nullam id dolor id nibh ultricies.`

## Ranking content higher with weights

By default, Pagefind will boost the `h1` through `h6` tags above any other content on the page. 

You can also use your own custom ranking via the `data-pagefind-weight` attribute:

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

The default weight of body content is `1`, and you can set a custom weight of any number from `0.0` to `10.0`. 
