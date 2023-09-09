---
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

Pagefind will work automatically for any language. Explicit language support improves the quality of search results and the Pagefind UI.

If word stemming is unsupported, search results won't match across root words. If UI translations are unsupported, the Pagefind UI will be shown in English.

| Language          | UI Translations | Word Stemming |
|-------------------|-----------------|---------------|
| Afrikaans — `af`  | ✅               | ❌             |
| Arabic — `ar`     | ❌               | ✅             |
| Armenian — `hy`   | ❌               | ✅             |
| Basque — `eu`     | ❌               | ✅             |
| Catalan — `ca`    | ✅               | ✅             |
| Chinese — `zh`    | ✅               | See below     |
| Danish — `da`     | ❌               | ✅             |
| Dutch — `nl`      | ✅               | ✅             |
| English — `en`    | ✅               | ✅             |
| Finnish — `fi`    | ✅               | ✅             |
| French — `fr`     | ✅               | ✅             |
| Galician — `gl`   | ✅               | ❌             |
| German — `de`     | ✅               | ✅             |
| Greek — `el`      | ❌               | ✅             |
| Hindi — `hi`      | ✅               | ✅             |
| Hungarian — `hu`  | ❌               | ✅             |
| Indonesian — `id` | ✅               | ✅             |
| Irish — `ga`      | ❌               | ✅             |
| Italian — `it`    | ✅               | ✅             |
| Japanese — `ja`   | ✅               | See below     |
| Lithuanian — `lt` | ❌               | ✅             |
| Nepali — `ne`     | ❌               | ✅             |
| Norwegian — `no`  | ✅               | ✅             |
| Portuguese — `pt` | ✅               | ✅             |
| Romanian — `ro`   | ❌               | ✅             |
| Russian — `ru`    | ✅               | ✅             |
| Serbian — `sr`    | ✅               | ✅             |
| Spanish — `es`    | ✅               | ✅             |
| Swedish — `sv`    | ✅               | ✅             |
| Tamil — `ta`      | ✅               | ✅             |
| Turkish — `tr`    | ✅               | ✅             |
| Yiddish — `yi`    | ❌               | ✅             |

> Feel free to [open an issue](https://github.com/CloudCannon/pagefind/issues/new) if there's a language you would like better support for, or [contribute a translation](https://github.com/CloudCannon/pagefind/tree/main/pagefind_ui/translations) for Pagefind UI in your language.

## Specialized languages

> This section currently applies to Chinese and Japanese languages. Specialized languages are only supported in Pagefind's extended release, which is the default when running `npx pagefind`.

Currently when indexing, Pagefind does not support stemming for specialized languages, but does support segmentation for words not separated by whitespace.

Pagefind does not _yet_ support segmentation of the search query, so searching in the browser requires that words in the search query are separated by whitespace.

In practice, this means that on a page tagged as a `zh-` language, `每個月都` will be indexed as the words `每個`, `月`, and `都`. 

When searching in the browser, searching for `每個`, `月`, or `都` individually will work. Additionally, searching `每個 月 都` will return results containing each word in any order, and searching `"每個 月 都"` in quotes will match `每個月都` exactly.

Searching for `每個月都` will return zero results, as Pagefind is not able to segment it into words in the browser. Work to improve this is underway and will hopefully remove this limitation in the future.
