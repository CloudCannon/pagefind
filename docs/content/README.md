---
date: 2022-06-01T00:00:00.000Z
title: Pagefind
nav_title: Home
weight: 1
---
Pagefind is a fully static search library that aims to perform well on large sites, while using as little of your users' bandwidth as possible, and without hosting any infrastructure.

Pagefind runs after Hugo, Eleventy, Jekyll, Next, Astro, SvelteKit, or **any other SSG**. The installation process is always the same: Pagefind only requires a folder containing the built static files of your website, so in most cases no configuration is needed to get started.

After indexing, Pagefind adds a static search bundle to your built files, which exposes a JavaScript search API that can be used anywhere on your site. Pagefind also provides a prebuilt UI that can be used with no configuration (you can see the prebuilt UI at the top of this page).

The goal of Pagefind is that websites with tens of thousands of pages should be searchable by someone in their browser, while consuming a reasonable amount of bandwidth. Pagefind's search index is split into chunks, so that searching in the browser only ever needs to load a small subset of the search index. Pagefind can run a full-text search on a 10,000 page site with a total network payload under 300KB, including the Pagefind library itself. For most sites, this will be closer to 100KB.

To play with a larger instance of Pagefind, check out our [xkcd index demo](https://xkcd.pagefind.app/). To learn more about how Pagefind works under the hood, and the problems it solves, check out the Pagefind talk from HugoConf 2022 below, or read the [release post on the CloudCannon blog](https://cloudcannon.com/blog/introducing-pagefind/).