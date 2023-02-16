import PagefindSvelte from './svelte/ui.svelte';

let scriptBundlePath;
try {
    scriptBundlePath = new URL(document.currentScript.src).pathname.match(/^(.*\/)(?:pagefind-)?ui.js.*$/)[1];
} catch (e) {
    scriptBundlePath = "/_pagefind/";
    console.warn(`Pagefind couldn't determine the base of the bundle from the javascript import path. Falling back to the default of ${scriptBundlePath}.`);
    console.warn("You can configure this by passing a bundlePath option to PagefindUI");
    console.warn(`[DEBUG: Loaded from ${document?.currentScript?.src ?? "unknown"}]`);
}

export class PagefindUI {
    constructor(opts) {
        this._pfs = null;

        let selector = opts.element ?? "[data-pagefind-ui]";
        let bundlePath = opts.bundlePath ?? scriptBundlePath;
        let resetStyles = opts.resetStyles ?? true;
        let showImages = opts.showImages ?? true;
        let processResult = opts.processResult ?? null;
        let processTerm = opts.processTerm ?? null;
        let showEmptyFilters = opts.showEmptyFilters ?? true;
        let debounceTimeoutMs = opts.debounceTimeoutMs ?? 300;
        let mergeIndex = opts.mergeIndex ?? [];
        let translations = opts.translations ?? [];

        // Remove the UI-specific config before passing it along to the Pagefind backend
        delete opts["element"];
        delete opts["bundlePath"];
        delete opts["resetStyles"];
        delete opts["showImages"];
        delete opts["processResult"];
        delete opts["processTerm"];
        delete opts["showEmptyFilters"];
        delete opts["debounceTimeoutMs"];
        delete opts["mergeIndex"];
        delete opts["translations"];

        const dom = document.querySelector(selector);
        if (dom) {
            this._pfs = new PagefindSvelte({
                target: dom,
                props: {
                    base_path: bundlePath,
                    reset_styles: resetStyles,
                    show_images: showImages,
                    process_result: processResult,
                    process_term: processTerm,
                    show_empty_filters: showEmptyFilters,
                    debounce_timeout_ms: debounceTimeoutMs,
                    merge_index: mergeIndex,
                    translations,
                    pagefind_options: opts
                }
            })
        } else {
            console.error(`Pagefind UI couldn't find the selector ${selector}`);
        }
    }

    triggerSearch(term) {
        this._pfs.$$set({ "trigger_search_term": term });
    }
}
