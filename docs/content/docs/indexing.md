---
date: 2022-06-01
title: "Configuring how content is indexed"
nav_title: "Customizing the index"
nav_section: Indexing
weight: 2
---

You can control how your site is indexed by using various `data-pagefind-*` tags in your templates.

## Limiting what content is indexed

By default, Pagefind indexes all content inside your `<body>` element, with some exceptions — elements such as `<nav>`, `<script>`, and `<form>` will be skipped automatically. 

To refine indexing further, you can tag your main content area with `data-pagefind-body`:

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

> If `data-pagefind-body` is found anywhere on your site, any pages without this attribute will be removed from your index. This means that if you tag your blog post layout with `data-pagefind-body`, other pages like your homepage will no longer appear in search results. This is usually what you want — if not, just add `data-pagefind-body` there as well.

> Note that metadata and filters that are set outside of this element will still be used. If this is not what you want, see the [root selector](/docs/config-options/#root-selector) configuration option.

## Removing individual elements from the index

If you have a component that you don't want to include in your search index, you can tag it with `data-pagefind-ignore`:

```html
<main data-pagefind-body>
    <h1>Condimentum Nullam</h1>
    <aside data-pagefind-ignore>
        This content will not be indexed.
    </aside>
    <p>Nullam id dolor id nibh ultricies.</p>
</main>
```

The `data-pagefind-ignore` attribute can optionally take a value of `index` or `all`. Omitting a value implies `index`, which will exclude the element and all children from the search index, but will still process filters and metadata within the element, and will still try to detect a default title or image found within this element.

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

> To remove elements without changing your templating, see the [exclude selectors](/docs/config-options/#exclude-selectors) CLI option.

## Indexing attributes

Attributes of HTML elements can be added to the main search index with the `data-pagefind-index-attrs` attribute:

```html
<h1>Condimentum Nullam</h1>
<img src="/hero.png"
     title="Image Title"
     alt="Image Alt"
     data-pagefind-index-attrs="title,alt" />
<p>Nullam id dolor id nibh ultricies.</p>
```

This will be indexed as: `Condimentum Nullam. Image Title. Image Alt. Nullam id dolor id nibh ultricies.`
