---
date: 2022-06-01
title: "Getting Started with Pagefind"
nav_title: "Getting Started"
nav_section: Root
weight: 7
---

Pagefind runs after your static generator, and outputs a static search bundle to your generated site. Unlike many search libraries, you don't need to build a search index by hand — the index is generated for you from your generated site.

To get started, we'll use the search UI that Pagefind provides out of the box. Add the following snippet to a page of your choice:

```html
<link href="/_pagefind/pagefind-ui.css" rel="stylesheet">
<script src="/_pagefind/pagefind-ui.js" type="text/javascript"></script>
<div id="search"></div>
<script>
    new PagefindUI({ element: "#search" });
</script>
```

> The `/_pagefind/pagefind-ui.css` and `/_pagefind/pagefind-ui.js` assets will be created by Pagefind in the next step.

Now build your site to an output directory — this guide assumes that you're running `hugo` and that your site is output to the `public/` directory. Pagefind works with any set of static HTML files, so adjust these configurations as needed.

> If you're running a development server (i.e. `hugo serve`) you won't see anything yet, as Pagefind needs to index the _output_ of your build. Let's do that now.

## Indexing your site

The easiest way to run pagefind is through npx, where `--source` point to the output directory of your static site generator:

```bash
npx -y pagefind --source public
```

You should see some output along the lines of:
```
Indexed 2496 pages
Indexed 22852 words
Indexed 0 filters
Created 27 index chunks
Finished in 2.357 seconds
```

We can see that a bunch of content was indexed. Since Pagefind has modified your generated static site, we won't be able to see it through your SSG's development server. Instead, you'll need to host your output directory youself — a quick way to do so on most UNIX systems is by running `python3 -m http.server` from your output directory.

Loading this in your browser, you should see a search input on your page. Have a play, and bask in how easy that was to integrate.

The last required step is to run Pagefind after building your site on your CMS or hosting platform. If you're a CloudCannon user, add a [`.cloudcannon/postbuild`](https://cloudcannon.com/documentation/articles/extending-your-build-process-with-hooks/) file containing the npx command above. For other platforms, setup an equivalent command to run after your site build.

For many use cases, you can stop here and mark it as complete. Or, you can dive deeper into Pagefind and configure it to your liking — check out [Index Configuration](/docs/index-configuration/) to get started.
