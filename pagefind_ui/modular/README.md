# Pagefind Modular UI

> The Pagefind Modular UI is under development, and as it currently stands should be treated as a prerelease version. If you rely on this on a production website, make sure to pin your Pagefind versions.

Pagefind is a fully static search library that aims to perform well on large sites, while using as little of your users' bandwidth as possible. 

Pagefind runs after any static site generator and automatically indexes the built static files. Pagefind then outputs a static search bundle to your website, and exposes a JavaScript search API that can be used anywhere on your site.

See the [Pagefind Documentation](https://pagefind.app/) for full usage.

The Pagefind Modular UI allows you to build a search UI out of Modules, all connected to one instance of Pagefind. With this, rich search experiences can be quickly created, and the look+feel of your website can more easily be matched.

## Quick usage

> These code snippets assume you have already indexed your website with the Pagefind CLI.

### Quick usage via output files
The Pagefind CLI outputs assets for the Modular UI that can be loaded directly:

```html
<link href="/pagefind/pagefind-modular-ui.css" rel="stylesheet">
<script src="/pagefind/pagefind-modular-ui.js"></script>

<script>
    window.addEventListener('DOMContentLoaded', (event) => {
        const instance = new PagefindModularUI.Instance();
        instance.add(new PagefindModularUI.Input({
            containerElement: "#search"
        }));
        instance.add(new PagefindModularUI.ResultList({
            containerElement: "#results"
        }));
    });
</script>
```

If using the output files, all code snippets below will require the `PagefindModularUI` prefix to access modules.

### Quick usage via npm
The Modular UI is also distributed as an NPM package:

```js
import { Instance, Input, ResultList } from "@pagefind/modular-ui";

const instance = new Instance({
    bundlePath: "/pagefind/"
});
instance.add(new Input({
    containerElement: "#searchbox"
}));
instance.add(new ResultList({
    containerElement: "#searchresults"
}));
```

With a bundler configuration that supports CSS:

```js
import styles from "@pagefind/modular-ui/css/ui.css";
```

## Instance

```js
const instance = new Instance({
    bundlePath: "/pagefind/"
});
```

An `Instance` serves as the central hub that all modules are connected to, and facilitates communication between each module and the Pagefind JS API.

| Option       | Description                                                                                                                         |
|--------------|-------------------------------------------------------------------------------------------------------------------------------------|
| `bundlePath` | See [UI > Bundle path](https://pagefind.app/docs/ui/#bundle-path)                                                                   |
| `mergeIndex` | See [Searching additional sites from Pagefind UI](https://pagefind.app/docs/multisite/#searching-additional-sites-from-pagefind-ui) |

| Method        | Description                          |
|---------------|--------------------------------------|
| `add(module)` | Connects a module to this `Instance` |

## Modules

The Modular UI currently ships with a small handful of prebuilt modules, and more will be added in future releases.

### Input

```js
instance.add(new Input({
    containerElement: "#searchbox"
}));
// or
instance.add(new Input({
    inputElement: "#existinginput"
}));
```

| Option              | Description                                                                                                                        |
|---------------------|------------------------------------------------------------------------------------------------------------------------------------|
| `containerElement`  | A selector to an element that a new search input should be placed within                                                           |
| `inputElement`      | A selector to an existing `<input />` element that should be used as the search input. _(NB: No Pagefind styling will be applied)_ |
| `debounceTimeoutMs` | Number of ms (default: `300`) to wait before performing a search while a user is typing                                            |

### ResultList

```js
instance.add(new ResultList({
    containerElement: "#results"
}));
```

| Option                | Description                                                               |
|-----------------------|---------------------------------------------------------------------------|
| `containerElement`    | A selector to an element that the results should be placed within         |
| `placeholderTemplate` | A function that returns the template for a result that has not yet loaded |
| `resultTemplate`      | A function that returns the template for a search result                  |

```js
// Larger example:
instance.add(new ResultList({
    containerElement: "#results",
    placeholderTemplate: () => {
        return "<p>Loading...</p>";
    },
    resultTemplate: (result) => {
        const el = document.createElement("p");
        el.classList.add("my-result-class");
        el.innerText = result.meta.title;
        return el;
    }
}));
```

The template functions can return either a string, a DOM node, or an array of DOM nodes.

### FilterPills

```js
instance.add(new FilterPills({
    containerElement: "#filter",
    filter: "author"
}));
```

| Option             | Description                                                                                                 |
|--------------------|-------------------------------------------------------------------------------------------------------------|
| `containerElement` | A selector to an element that the filter pill row should be placed within                                   |
| `filter`           | Which filter this row should represent. Filter name should exist in the search index                        |
| `ordering`         | An array containing the ideal order to display filter values in. Unmatched values will appear at the end    |
| `alwaysShow`       | Whether to show the component when there are no results                                                     |
| `selectMultiple`   | Whether this component should toggle between single filter values, or allow multiple to be selected at once |


### Summary

```js
instance.add(new Summary({
    containerElement: "#summary"
}));
```

| Option             | Description                                                                   |
|--------------------|-------------------------------------------------------------------------------|
| `containerElement` | A selector to an element that the search summary text should be placed within |
| `defaultMessage`   | The text to show when there is no summary. Defaults to nothing                |

### Custom Modules

Full documentation to come for custom modules, as this syntax may change. For the adventurous, here is a template for a UI module using the event system:

```js
export class MyCustomComponent {
    constructor(opts = {}) {
        // Handle adding MyCustomComponent to the page
    }

    // This function is called by the containing Instance when this component is added
    register(instance) {
        this.instance = instance; // Store the instance so we can trigger events

        instance.on("search", (term, filters) => {
            // A new search has been started
        });

        instance.on("loading", () => {
            // A search is running and results are being loaded
        });

        instance.on("results", (results) => {
            // Search results are available
        });

        instance.on("filters", (filters) => {
            // The set of available filters has been updated
        });
    }

    // Assuming this function is triggered by some user action
    myFunction(searchTerm) {
        this.instance.triggerSearch(searchTerm);
    }
}
```

Alternatively, you can react to events from the instance directly:

```js
const instance = new Instance({
    bundlePath: "/pagefind/"
});
instance.on("results", (results) => {
    // Search results are available
});
```
