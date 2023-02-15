# Pagefind Default UI


Pagefind is a fully static search library that aims to perform well on large sites, while using as little of your users' bandwidth as possible. 

Pagefind runs after any static site generator and automatically indexes the built static files. Pagefind then outputs a static search bundle to your website, and exposes a JavaScript search API that can be used anywhere on your site.

See the [Pagefind Documentation](https://pagefind.app/) for full usage.

Quick usage:

> These code snippets assume you have already indexed your website with the Pagefind CLI.

```js
import { PagefindUI } from '@pagefind/default-ui'

window.addEventListener('DOMContentLoaded', (event) => {
    new PagefindUI({ element: "#search" });
});
```

With a bundler configuration that supports CSS:

```js
import { PagefindUI } from '@pagefind/default-ui'
import styles from "@pagefind/default-ui/css/ui.css";

window.addEventListener('DOMContentLoaded', (event) => {
    new PagefindUI({ element: "#search" });
});
```

For all configuration options, see the [Pagefind Documentation](https://pagefind.app/).