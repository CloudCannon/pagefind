import PagefindSvelte from './svelte/ui.svelte';

class PagefindUI {
    constructor(opts) {
        let selector = opts.element ?? "[data-pagefind-ui]";
        let bundlePath = opts.bundlePath;
        let resetStyles = opts.resetStyles ?? true;

        if (!bundlePath) {
            try {
                bundlePath = new URL(document.currentScript.src).pathname.match(/^(.*\/)(?:pagefind-)?ui.js.*$/)[1];
            } catch (e) {
                bundlePath = "/_pagefind/";
                console.warn(`Pagefind couldn't determine the base of the bundle from the javascript import path. Falling back to the default of ${bundlePath}.`);
                console.warn("You can configure this by passing a bundlePath option to PagefindUI");
                console.warn(`[DEBUG: Loaded from ${document.currentScript.src ?? "unknown"}]`);
            }
        }

        // Remove the UI-specific config before passing it along to the Pagefind backend
        delete opts["element"];
        delete opts["bundlePath"];
        delete opts["resetStyles"];

        const dom = document.querySelector(selector);
        if (dom) {
            new PagefindSvelte({
                target: dom,
                props: {
                    base_path: bundlePath,
                    reset_styles: resetStyles,
                    pagefind_options: opts,
                }
            })
        } else {
            console.error(`Pagefind UI couldn't find the selector ${selector}`);
        }
    }
}

window.PagefindUI = PagefindUI;
