---
title: Pagefind
nav_title: Home
weight: 1
---
Pagefind is a fully static search library that aims to perform well on large sites, while using as little of your users' bandwidth as possible, and without hosting any infrastructure.

Pagefind runs after Hugo, Eleventy, Jekyll, Next, Astro, SvelteKit, or **any other website framework**. The installation process is always the same: Pagefind only requires a folder containing the built static files of your website, so in most cases no configuration is needed to get started.

After indexing, Pagefind adds a static search bundle to your built files, which exposes a JavaScript search API that can be used anywhere on your site. Pagefind also provides a prebuilt UI that can be used with no configuration. (You can see the prebuilt UI at the top of this page.)

The goal of Pagefind is that websites with tens of thousands of pages should be searchable by someone in their browser, while consuming as little bandwidth as possible. Pagefind's search index is split into chunks, so that searching in the browser only ever needs to load a small subset of the search index. Pagefind can run a full-text search on a 10,000 page site with a total network payload under 300kB, including the Pagefind library itself. For most sites, this will be closer to 100kB.

## Features

- Zero-config support for multilingual websites
- Rich filtering engine for knowledge bases
- Custom sort attributes
- Custom metadata tracking
- Custom content weighting
- Return results for sections of a page
- Search across multiple domains
- Index **anything** (e.g. PDFs, JSON files, or subtitles) with the NodeJS indexing library
- All features available with the same low-bandwidth footprint

## Pagefind demos

To test large instances of Pagefind, check out:

{{< card
    title="MDN, indexed by Pagefind" 
    url="mdn.pagefind.app"
    image="/mdn.svg"
    classname="dark-silhouette"
    >}}

{{< card
    title="Godot documentation, indexed by Pagefind" 
    url="godot.pagefind.app"
    image="/godot.svg"
    >}}

{{< card
    title="XKCD, indexed by Pagefind" 
    url="xkcd.pagefind.app"
    image="/xkcd.png"
    classname="dark-blend"
    >}}
