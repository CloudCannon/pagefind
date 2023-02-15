export { Input } from "./components/input";
export { ResultList } from "./components/resultList";
export { Summary } from "./components/summary";

/*
- Add some styles for the two components implemented thus far at `css/ui.css`
- Wire up the build.js for `modular` to match `default`
- Wire up the GH action for `modular` to match `default`
- Add `files` to `package.json`
*/


let scriptBundlePath;
try {
    scriptBundlePath = new URL(document.currentScript.src).pathname.match(/^(.*\/)(?:pagefind-)?modular-ui.js.*$/)[1];
} catch (e) {
    scriptBundlePath = "/_pagefind/";
    console.warn(`Pagefind couldn't determine the base of the bundle from the javascript import path. Falling back to the default of ${scriptBundlePath}.`);
    // TODO(modular): Ensure bundlePath is available on Instance
    console.warn("You can configure this by passing a bundlePath option to PagefindComposable Instance");
    console.warn(`[DEBUG: Loaded from ${document?.currentScript?.src ?? "unknown"}]`);
}

export class Instance {
    constructor(opts = {}) {
        this.__pagefind__ = null;
        this.__initializing__ = null;
        this.__searchID__ = 0;
        this.__hooks__ = {
            "search": [],
            "filters": [],
            "loading": [],
            "results": []
        };

        this.components = [];

        this.searchTerm = "";
        this.searchFilters = {};
        this.searchResult = {};
        this.availableFilters = null;

        this.options = {
            bundlePath: opts.bundlePath ?? scriptBundlePath,
            //TODO: USE resetStyles: opts.resetStyles ?? true,
            //TODO: USE processResult: opts.processResult ?? null,
            //TODO: USE processTerm: opts.processTerm ?? null,
            mergeIndex: opts.mergeIndex ?? [],
            //TODO: USE translations: opts.translations ?? [],
        }

        delete opts["bundlePath"];
        delete opts["resetStyles"];
        delete opts["processResult"];
        delete opts["processTerm"];
        delete opts["debounceTimeoutMs"];
        delete opts["mergeIndex"];
        delete opts["translations"];

        // Remove the UI-specific config before passing it along to the Pagefind backend
        this.pagefindOptions = opts;
    }

    add(component) {
        component?.register?.(this);
        this.components.push(component);
    }

    on(event, callback) {
        if (!this.__hooks__[event]) {
            const supportedEvents = Object.keys(this.__hooks__).join(", ");
            console.error(`[Pagefind Composable]: Unknown event type ${event}. Supported events: [${supportedEvents}]`);
            return;
        }
        if (typeof callback !== "function") {
            console.error(`[Pagefind Composable]: Expected callback to be a function, received ${typeof callback}`);
            return;
        }
        this.__hooks__[event].push(callback);
    }

    triggerLoad() {
        this.__load__();
        // this.components.forEach(component => component?.triggerLoad?.());
    }

    triggerSearch(term, filters) {
        this.searchTerm = term;
        this.searchFilters = filters;
        this.__dispatch__("search", term, filters);
        this.__search__(term, filters);
    }

    __dispatch__(e, ...args) {
        this.__hooks__[e]?.forEach(hook => hook?.(...args));
    }

    async __clear__() {
        this.__dispatch__("results", {results: []});
        this.availableFilters = await this.__pagefind__.filters();
        this.__dispatch__("filters", this.availableFilters);
    }

    async __search__(term, filters) {
        this.__dispatch__("loading");
        await this.__load__();
        const thisSearch = ++this.__searchID__;

        if (!term || !term.length) {
            return this.__clear__();
        }

        const results = await this.__pagefind__.search(term, { filters });
        if (results && this.__searchID__ === thisSearch) {
            if (results.filters && Object.keys(results.filters)?.length) {
                this.availableFilters = results.filters;
                this.__dispatch__("filters", this.availableFilters);
            }
            this.searchResult = results;
            this.__dispatch__("results", this.searchResult);
        }
    }

    async __load__() {
        if (this.__initializing__) return;
        this.__initializing__ = true;
        if (!this.__pagefind__) {
            let imported_pagefind = await import(`${this.options.bundlePath}pagefind.js`);
            await imported_pagefind.options(this.pagefindOptions || {});
            for (const index of this.options.mergeIndex) {
                if (!index.bundlePath) {
                    throw new Error(
                        "mergeIndex requires a bundlePath parameter"
                    );
                }
                const url = index.bundlePath;
                delete index["bundlePath"];
                await imported_pagefind.mergeIndex(url, index);
            }
            this.__pagefind__ = imported_pagefind;
        }
        this.availableFilters = await this.__pagefind__.filters();
        this.__dispatch__("filters", this.availableFilters);
    }
}
