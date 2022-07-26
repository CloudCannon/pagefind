import PagefindSvelte from './svelte/ui.svelte';

let scriptBundlePath;
try {
    scriptBundlePath = new URL(document.currentScript.src).pathname.match(/^(.*\/)(?:pagefind-)?ui.js.*$/)[1];
} catch (e) {
    scriptBundlePath = "/_pagefind/";
    console.warn(`Pagefind couldn't determine the base of the bundle from the javascript import path. Falling back to the default of ${bundlePath}.`);
    console.warn("You can configure this by passing a bundlePath option to PagefindUI");
    console.warn(`[DEBUG: Loaded from ${document?.currentScript?.src ?? "unknown"}]`);
}

class PagefindUI {
    constructor(opts) {
        let selector = opts.element ?? "[data-pagefind-ui]";
        let bundlePath = opts.bundlePath ?? scriptBundlePath;
        let resetStyles = opts.resetStyles ?? true;
        let showEmptyFilters = opts.showEmptyFilters ?? true;

        // Remove the UI-specific config before passing it along to the Pagefind backend
        delete opts["element"];
        delete opts["bundlePath"];
        delete opts["resetStyles"];
        delete opts["showEmptyFilters"];

        const dom = document.querySelector(selector);
        if (dom) {
            new PagefindSvelte({
                target: dom,
                props: {
                    base_path: bundlePath,
                    reset_styles: resetStyles,
                    show_empty_filters: showEmptyFilters,
                    pagefind_options: opts,
                }
            })
        } else {
            console.error(`Pagefind UI couldn't find the selector ${selector}`);
        }
    }
}

window.PagefindUI = PagefindUI;
