---
title: "Configuring what content is indexed"
nav_title: "Configuring the index"
nav_section: Indexing
weight: 2
---

You can control what content on your site is indexed via `data-pagefind-*` tags on your HTML elements.

## Limiting what sections of a page are indexed

By default, Pagefind starts indexing from your `<body>` element.

To narrow this down, you can tag your main content area with `data-pagefind-body`:

{{< diffcode >}}
```html
<body>
+    <main data-pagefind-body>
        <h1>Condimentum Nullam</h1>
        <p>Nullam id dolor id nibh ultricies.</p>
    </main>
    <aside>
        This content will not be indexed.
    </aside>
</body>
```
{{< /diffcode >}}

If `data-pagefind-body` is found anywhere on your site, any pages without this attribute will not be indexed. This means that if you tag your blog post layout with `data-pagefind-body`, other pages like your homepage will no longer appear in search results. This is usually what you want — if not, just add `data-pagefind-body` there as well.

Multiple `data-pagefind-body` elements may exist on a page, and their content will be combined.

> Note that metadata and filters that are set outside of this element will still be used. If this is not what you want, see the [root selector](/docs/config-options/#root-selector) CLI configuration option.

## Removing pages from Pagefind's index

Once a `data-pagefind-body` attribute exists on any page of your site, any pages without this attribute will not be indexed. As such, the best way to remove pages is by adding `data-pagefind-body` to the pages you **would** like to index.

If this isn't possible, see the [Pagefind CLI's glob option](/docs/config-options/#glob) to limit the files that Pagefind reads.

## Removing individual elements from the index

Pagefind has built-in elements that are not indexed. These are organizational elements such as `<nav>` and `<footer>`, or more programmatic elements such as `<script>` and `<form>`. These elements will be skipped over automatically.

If you have further elements that you don't want to include in your search index, you can tag them with `data-pagefind-ignore`:

{{< diffcode >}}
```html
<main data-pagefind-body>
    <h1>This content will be in your search index.</h1>
+    <aside data-pagefind-ignore>
+        This content will not be indexed.
+    </aside>
    <p>This content will also be in your search index.</p>
</main>
```
{{< /diffcode >}}

The `data-pagefind-ignore` attribute can optionally take a value of `index` or `all`. The default is `index`, which will exclude the element and all children from the search index while still processing filters and metadata within the element, and will still try to detect a default title or image found within this element.

Specifying `all` will exclude the element and its children from all processing.

{{< diffcode >}}
```html
<aside data-pagefind-ignore>
    <h1>This might still be detected as the page title</h1>
    <p data-pagefind-meta="a">This metadata will still appear in search results.</p>
</aside>

+<aside data-pagefind-ignore="all">
    <h1>This cannot be detected as the page title</h1>
    <p data-pagefind-meta="b">This metadata will not be picked up.</p>
</aside>
```
{{< /diffcode >}}

> To remove elements without changing your templating, see the [Pagefind CLI's exclude selectors option](/docs/config-options/#exclude-selectors).

## Adding HTML attributes to the index

Attributes of HTML elements can be added to the search index with the `data-pagefind-index-attrs` attribute:

{{< diffcode >}}
```html
<h1>Condimentum Nullam</h1>
<img src="/hero.png"
     title="Image Title"
     alt="Image Alt"
+     data-pagefind-index-attrs="title,alt" />
<p>Nullam id dolor id nibh ultricies.</p>
```
{{< /diffcode >}}

This attribute takes a comma-separated list of other attributes to include inline with the indexed content.
The above example will be indexed as: `Condimentum Nullam. Image Title. Image Alt. Nullam id dolor id nibh ultricies.`

## Indexing special characters

By default, Pagefind strips most punctuation out of the page when indexing content. Punctuation is also removed from the search term when searching.

For some sites, such as documentation for programming languages, searching for punctuation can be important. In these cases,
the default behavior can be changed using the [Include Characters](/docs/config-options/#include-characters) option when running Pagefind.

For example, given the following HTML:

```html
<p>The &lt;head&gt; tag</p>
```

Pagefind's default indexing would index `the`, `head`, and `tag`,
and a user typing in a search term of `<head>` will have their search adapted to `head`.
While this will still match the correct page, it won't distinguish between this result and a result talking about the head of a git repository.

With the [Include Characters](/docs/config-options/#include-characters) option set to `<>`, Pagefind will instead index `the`, `<head>`, `head`, and `tag`.
A search for `head` will still locate this page, while a search for `<head>` won't be rewritten and will specifically match this page.
