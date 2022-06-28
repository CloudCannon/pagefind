# Pagefind

## Quick Prerelease Docs

### Setup

After running a build (i.e. in a `.cloudcannon/postbuild` file):
```bash
npx pagefind@latest -s public
```

Where `public` matches your output dir — `_site` for Jekyll etc.

By default, this will index all content within your `body`. Pagefind currently has a few tags that can be used to customize this:

#### Limiting Indexing to Elements

Adding `data-pagefind-body` to an element will cause Pagefind to exlusively index content within this element and its children, instead of indexing the entire `<body>`. In most cases, you will want to add this attribute to the main element in your content layout.

If there are multiple regions you want to index, `data-pagefind-body` can be set on multiple elements on the same page.

If a `data-pagefind-body` element is found anywhere on your site, any pages without this element will be excluded from search. This means if you tag a specific region on your `post` layout with `data-pagefind-body`, your homepage will no longer be indexed (unless it too has a `data-pagefind-body` element).

Note: Filters and metadata outside a body element will still be processed.

#### Ignoring Elements

Adding `data-pagefind-ignore` to an element will exclude it from the search index. Some elements are excluded automatically, such as `<form>` and `<svg>` elements.
```html
<h1>Hello World</h1>
<p>Some content</p>
<footer data-pagefind-ignore>
    <span>This footer content will not be indexed</footer>
</footer>
```

Note: Filters and metadata inside an ignored element will still be processed.

#### Indexing Attributes

Attributes of HTML elements can be added to the main search index with the `data-pagefind-index-attrs` attribute.
```html
<img 
    src="/hero.png"
    title="Image Title"
    alt="Image Alt"
    data-pagefind-index-attrs="title,alt" />
```

#### Filters

Filters can be supplied alongside search terms to narrow the results. These can be set with `data-pagefind-filter`. By default, this will capture the contents of that element.

```html
<h1>Hello World</h1>
<p>Author: <span data-pagefind-filter="author">CloudCannon</span></p>
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

Metadata can be returned alongside the page content after a search. This can be tagged with `data-pagefind-meta` using the same syntax as `data-pagefind-filter`:

```html
<body data-pagefind-meta="date:2022-06-01">
    <h1 data-pagefind-meta="title">Hello World</h1>
    <img data-pagefind-meta="image[src]" src="/weka.png">
</body>
```

#### Local Development

Since Pagefind runs on a built site, you will currently need to build your site locally → run Pagefind → host that directory. Improving this development experience is on the roadmap.

### Configuration

Pagefind can be configured through CLI flags, environment variables, or configuration files. Values will be merged from all sources, with CLI flags overriding environment variables, and environment variables overriding configuration files.

#### Config files

Pagefind will look for a `pagefind.toml`, `pagefind.yml`, or `pagefind.json` file in the directory that you have run the command in.

```bash
echo "source: public" > pagefind.yml
npx pagefind
```

#### Environment Variables

Pagefind will load any values via a `PAGEFIND_*` environment variable.

```bash
PAGEFIND_SOURCE=public npx pagefind
```

#### CLI Flags

Pagefind can be passed CLI flags directly.

```bash
npx pagefind --source public
```

#### Configuration Options:

| flag         | env                 | config     | default   | description                                                |
|--------------|---------------------|------------|-----------|------------------------------------------------------------|
| --source     | PAGEFIND_SOURCE     | source     |           | Required: The location of your built static site           |
| --bundle-dir | PAGEFIND_BUNDLE_DIR | bundle_dir | _pagefind | The folder to output search files into, relative to source |

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
            filters: {},
        }
    ]
}
```

To load the data for a result, await the data function:

```js
const result = await search.results[0].data();
```

Which will yield:

```js
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
  "word_count": 242,
}
```

#### Filtering

To load the available filters, you can run:

```js
const filters = await pagefind.filters();
```

This will return the following object, showing the number of search results available under the given `filter: value`.
```js
{
    "filter": {
        "value_one": 4,
        "value_two": 12,
    },
    "color": {
        "Orange": 6
    }
}
```

To filter results alongside searching, pass an options object to the search function:
```js
const search = await pagefind.search("hello", {
    filters: {
        color: "Orange"
    }
});
```

If the filters have been loaded with `await pagefind.filters()`, counts will also be returned with each search object, detailing the number of remaining items for each filter value:
```js
{ 
    results: [
        {
            id: "6fceec9",
            data: async function data(),
            filters: {
                "filter": {
                    "value_one": 0,
                    "value_two": 3,
                },
                "color": {
                    "Orange": 1
                }
            },
        }
    ]
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
