import Pagefind from './svelte/ui.svelte';

class PagefindUi {
    constructor(opts) {
        let selector = opts.element ?? "[data-pagefind-ui]";
        let bundlePath = opts.bundlePath;

        if (!bundlePath) {
            try {
                bundlePath = new URL(document.currentScript.src).pathname.match(/^(.*\/)(?:pagefind-)?ui.js.*$/)[1];
            } catch (e) {
                bundlePath = "/_pagefind/";
                console.warn(`Pagefind couldn't determine the base of the bundle from the javascript import path. Falling back to the default of ${bundlePath}.`);
                console.warn("You can configure this by passing a bundlePath option to PagefindUi");
            }
        }

        // Remove the UI-specific config before passing it along to the Pagefind backend
        delete opts["element"];
        delete opts["bundlePath"];

        const dom = document.querySelector(selector);
        if (dom) {
            new Pagefind({
                target: dom,
                props: {
                    base_path: bundlePath,
                    pagefind_options: opts,
                }
            })
        } else {
            console.error(`Pagefind UI couldn't find the selector ${selector}`);
        }
    }
}

window.PagefindUi = PagefindUi;
