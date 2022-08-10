---
date: 2022-06-01
title: "Multilingual search"
nav_title: "Multilingual search"
nav_section: Indexing
weight: 80
---

Pagefind supports multilingual sites out of the box, with zero configuration. 

When indexing, Pagefind will look for a [`lang` attribute](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/lang) on your `html` element. Indexing will then run independently for each detected language. When Pagefind initializes in the browser it will check the same `lang` attribute and load the appropriate index. 

If you run Pagefind on a page tagged as `<html lang="pt-br">`, you will automatically search only the pages on the site with the same language. Pagefind will also adapt any stemming algorithms to the target language if supported.

## Opting out of multilingual search

Setting the [force language](/docs/config-options/#force-language) option when indexing will opt out of this feature and create one index for the site as a whole.
