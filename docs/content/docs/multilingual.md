---
date: 2022-06-01
title: "Multilingual search"
nav_title: "Multilingual search"
nav_section: Indexing
weight: 80
---

Pagefind supports multilingual sites out of the box, with zero configuration. 

When indexing, Pagefind will look for a [`lang` attribute](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/lang) on your `html` element. Indexing will then run independently for each detected language. When Pagefind initializes in the browser it will check the same `lang` attribute and load the appropriate index.

If you load Pagefind search on a page tagged as `<html lang="pt-br">`, you will automatically search only the pages on the site with the same language. Pagefind will also adapt any stemming algorithms to the target language if supported. This applies to both the Pagefind JS API and the Pagefind UI.

The Pagefind UI itself is translated into a range of languages, and will adapt automatically to the page language if possible.

## Opting out of multilingual search

Setting the [force language](/docs/config-options/#force-language) option when indexing will opt out of this feature and create one index for the site as a whole.

## Language support

Pagefind will work automatically for any language, explicit language support only improves the quality of search results and the Pagefind UI.

If word stemming is unsupported, search results won't match across root words. If UI translations are unsupported, the Pagefind UI will be shown in English.

> Feel free to [open an issue](https://github.com/CloudCannon/pagefind/issues/new) if there's a language you would like better support for, or [contribute a translation](https://github.com/CloudCannon/pagefind/tree/main/pagefind_ui/translations) for Pagefind UI in your language.

| Language          | UI Translations | Word Stemming |
|-------------------|-----------------|---------------|
| Afrikaans — `af`  | ✅               | ❌             |
| Arabic — `ar`     | ❌               | ✅             |
| Armenian — `hy`   | ❌               | ✅             |
| Basque — `eu`     | ❌               | ✅             |
| Catalan — `ca`    | ❌               | ✅             |
| Chinese — `zh`    | ✅               | ❌             |
| Danish — `da`     | ❌               | ✅             |
| Dutch — `nl`      | ❌               | ✅             |
| English — `en`    | ✅               | ✅             |
| Finnish — `fi`    | ❌               | ✅             |
| French — `fr`     | ❌               | ✅             |
| German — `de`     | ✅               | ✅             |
| Greek — `el`      | ❌               | ✅             |
| Hindi — `hi`      | ❌               | ✅             |
| Hungarian — `hu`  | ❌               | ✅             |
| Indonesian — `id` | ❌               | ✅             |
| Irish — `ga`      | ❌               | ✅             |
| Italian — `it`    | ❌               | ✅             |
| Japanese — `ja`   | ✅               | ❌             |
| Lithuanian — `lt` | ❌               | ✅             |
| Nepali — `ne`     | ❌               | ✅             |
| Norwegian — `no`  | ✅               | ✅             |
| Portuguese — `pt` | ✅               | ✅             |
| Romanian — `ro`   | ❌               | ✅             |
| Russian — `ru`    | ✅               | ✅             |
| Serbian — `sr`    | ❌               | ✅             |
| Spanish — `es`    | ❌               | ✅             |
| Swedish — `sv`    | ❌               | ✅             |
| Tamil — `ta`      | ❌               | ✅             |
| Turkish — `tr`    | ❌               | ✅             |
| Yiddish — `yi`    | ❌               | ✅             |