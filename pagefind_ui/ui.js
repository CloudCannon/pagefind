import PagefindUi from './svelte/ui.svelte';

(() => {
    let base_path = "/_pagefind/";
    try {
        base_path = new URL(document.currentScript.src).pathname.match(/^(.*\/)(?:pagefind-)?ui.js.*$/)[1];
    } catch (e) {
        console.warn("Pagefind couldn't determine the base of the bundle from the import path. Falling back to the default.");
    }

    const dom = document.querySelector("[data-pagefind-ui]");
    new PagefindUi({
        target: dom,
        props: {
            base_path
        }
    })
})();
