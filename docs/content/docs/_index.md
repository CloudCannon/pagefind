---
title: "Getting Started with Pagefind"
nav_title: "Quick Start"
nav_section: Root
weight: 7
---

Pagefind runs after your static generator, and outputs a static search bundle to your generated site. With Pagefind, you don't need to build a search index by hand — the index is generated for you from your generated site.

Since Pagefind indexes your site _after_ it builds, we'll do things slightly out of order and add a search UI first — so that it already exists on our built site when we go to index it.

Pagefind provides a prebuilt search UI out of the box. Add the following snippet to a page of your choice:

```html
<link href="/pagefind/pagefind-ui.css" rel="stylesheet">
<script src="/pagefind/pagefind-ui.js"></script>
<div id="search"></div>
<script>
    window.addEventListener('DOMContentLoaded', (event) => {
        new PagefindUI({ element: "#search", showSubResults: true });
    });
</script>
```

> The `/pagefind/pagefind-ui.css` and `/pagefind/pagefind-ui.js` assets will be created by Pagefind when we index the site.

Now build your site to an output directory — this guide assumes that you're running `hugo` and that your site is output to the `public/` directory. Pagefind works with any set of static HTML files, so adjust these configurations as needed.

> If you're running a development server (i.e. `hugo serve`) you won't see anything yet, as Pagefind needs to index the _output_ of your build. Let's do that now.

## Indexing your site

The easiest way to run pagefind is through npx. If you don't have Node and npm installed, or want to install Pagefind another way, see the [Installing Pagefind](/docs/installation/) guide.

Run the following command from your terminal, where `--site` points to the output directory of your static site generator. We'll also add `--serve` so that we can view our final site right away.

```bash
npx -y pagefind --site public --serve
```

You should see some output along the lines of:
```
Indexed 2496 pages
Indexed 22852 words
Indexed 0 filters
Created 27 index chunks
Finished in 2.357 seconds
```

We can see that a bunch of content was indexed, and Pagefind will be running a preview server (likely on [:1414](http://localhost:1414)).

> Note that Pagefind itself does not have any server component — the search integration is fully baked into your static site. The `--serve` flag here is a shortcut for running Pagefind, followed by serving your output site through any static web server.

Loading this in your browser, you should see a search input on your page. Try searching for some content and you will see results appear from your site.

The last required step is to run Pagefind after building your site on your CMS or hosting platform. If you're a CloudCannon user, add a [`.cloudcannon/postbuild`](https://cloudcannon.com/documentation/articles/extending-your-build-process-with-hooks/) file containing the npx command above (minus the `--serve` flag). For other platforms, set up an equivalent command to run after your site build — the end goal is that Pagefind will run after every build of your site before it is deployed.

For many use cases, you can stop here and mark it as complete. Or, you can dive deeper into Pagefind and configure it to your liking — check out [Customizing the index](/docs/indexing/) for some next steps.
