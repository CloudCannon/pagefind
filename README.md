# Pagefind

## Quick Prerelease Docs

### Setup

After running a build (i.e. in a `.cloudcannon/postbuild` file):
```bash
npx pagefind@latest -s public
```

Where `public` matches your output dir — `_site` for Jekyll etc.

This will currently index all content within your `body`. Pagefind currently has a few tags that can be used to customize this:

#### Ignoring Elements

Adding `data-pagefind-ignore` to an element will exclude it from the search index. Some elements are excluded automatically, such as `form` and `svg` elements.
```html
<body>
    <h1>Hello World</h1>
    <p>Some content</p>
    <footer data-pagefind-ignore>
        <span>This footer content will not be indexed</footer>
    </footer>
</body>
```

#### Filters

Filters can be supplied alongside search terms to narrow the results.

Filters can be set with `data-pagefind-filter`. By default, this will capture the contents of that element.

```html
<body>
    <h1>Hello World</h1>
    <p>Author: <span data-pagefind-filter="author">CloudCannon</span></p>
</body>
```

Filters can also be set inline:
```html
<body data-pagefind-filter="author:CloudCannon">
    <h1>Hello World</h1>
</body>
```

Or based on an attribute:

```html
<head>
    <meta 
        data-pagefind-filter="author[content]"
        content="CloudCannon"
        property="og:site_name">
</head>
```

#### Metadata

Metadata can be returned alongside the page content after a search.

Any metadata can be tagged with `data-pagefind-meta` using the same syntax as `data-pagefind-filter`:

```html
<body data-pagefind-meta="date:2022-06-01">
    <h1 data-pagefind-meta="title">Hello World</h1>
    <img data-pagefind-meta="image[src]" src="/weka.png">
</body>
```

### Usage

Anywhere on your site, you can initialize Pagefind with:
```js
const pagefind = await import("/_pagefind/pagefind.js");
```

This will load the Pagefind library and the metadata about the site.

If your site is on a subpath, this should be included. i.e. in the CloudCannon documentation, we load `/documentation/_pagefind/pagefind.js`.

To perform a search, use the `pagefind` object you initialized above:
```js
const search = await pagefind.search("hello");
```

This will return the following object:
```js
{ 
    results: [
        {
            id: "6fceec9",
            data: async function data(),
        }
    ]
}
```

To load the data for a result, await the data attribute:

```js
const result = await search.results[0].data();
```

Which will yield:

```js
{
  "url": "/url-of-the-page/",
  "title": "The title from the first h1 element on the page",
  "excerpt": "A small snippet of the <mark>content</mark>, with the <mark>search</mark> term(s) highlighted in mark elements."
  "filters": {
      "author": "CloudCannon"
  },
  "meta": {
      "image": "/weka.png"
  },
  "content": "The full content of the page, formatted as text. Cursus Ipsum Risus Ullamcorper...",
  "word_count": 242,
}
```

#### Basic Bad Vanilla JS Example

```js
// Ideally this import is lazy-initialized, i.e. when a search input is focused
const pagefind = await import("/_pagefind/pagefind.js");
const resultEl = document.querySelector("ul");

const search = async (term, filters) => {
    const search = await pagefind.search(term, { filters });

    // Add placeholder elements for results
    resultEl.innerHTML = search.results.map(r => `<li><h6>...</h6>...</li>`).join('');

    const first_five = await Promise.all(search.results.slice(0, 5).map(r => r.data()));

    // Present loaded results
    resultEl.innerHTML = first_five.map(r => [
            `<li>`,
            `<h6><a href="${r.url}">${r.title}</a></h6>`,
            `${r.excerpt}`,
            `</li>`
        ].join('')).join('');
}

search("hello", { author: "CloudCannon" });
```
