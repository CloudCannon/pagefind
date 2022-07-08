---
date: 2022-06-01
title: "Pagefind"
nav_title: "Home"
weight: 1
---

Pagefind is a fully static search library that aims to perform well on large sites, while using as little of your users' bandwidth as possible. 

Pagefind runs after any static site generator and automatically indexes the built static files. Pagefind then outputs a static search bundle to your website, and exposes a JavaScript search API that can be used anywhere on your site, or a prebuilt UI that can be used with no configuration.

Pagefind aims to index a site quickly, and with as little configuration as possible — in most cases no configuration is needed to get started. Additionally, websites with tens of thousands of pages should be searchable by someone in their browser, while consuming a reasonable amount of bandwidth. For a 10,000 page site, you can expect to perform a single-world search with a total network payload under 300KB — including the Pagefind javascript and webassembly libraries.

To play with a larger instance of Pagefind, check out our [xkcd index demo](https://xkcd.pagefind.app/). To learn more about how Pagefind works under the hood, and the problems it solves, check out the Pagefind talk from HugoConf 2022:

