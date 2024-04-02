import PagefindSvelte from "./svelte/ui.svelte";

let scriptBundlePath;
try {
  scriptBundlePath = new URL(document.currentScript.src).pathname.match(
    /^(.*\/)(?:pagefind-)?ui.js.*$/
  )[1];
} catch (e) {
  scriptBundlePath = "/pagefind/";
}

export class PagefindUI {
  constructor(opts) {
    this._pfs = null;

    let selector = opts.element ?? "[data-pagefind-ui]";
    let bundlePath = opts.bundlePath ?? scriptBundlePath;
    let pageSize = opts.pageSize ?? 5;
    let resetStyles = opts.resetStyles ?? true;
    let showImages = opts.showImages ?? true;
    let showSubResults = opts.showSubResults ?? false;
    let excerptLength = opts.excerptLength ?? 0;
    let processResult = opts.processResult ?? null;
    let processTerm = opts.processTerm ?? null;
    let showEmptyFilters = opts.showEmptyFilters ?? true;
    let openFilters = opts.openFilters ?? [];
    let debounceTimeoutMs = opts.debounceTimeoutMs ?? 300;
    let mergeIndex = opts.mergeIndex ?? [];
    let translations = opts.translations ?? [];
    let autofocus = opts.autofocus ?? false;
    let sort = opts.sort ?? null;

    // Remove the UI-specific config before passing it along to the Pagefind backend
    delete opts["element"];
    delete opts["bundlePath"];
    delete opts["pageSize"];
    delete opts["resetStyles"];
    delete opts["showImages"];
    delete opts["showSubResults"];
    delete opts["excerptLength"];
    delete opts["processResult"];
    delete opts["processTerm"];
    delete opts["showEmptyFilters"];
    delete opts["openFilters"];
    delete opts["debounceTimeoutMs"];
    delete opts["mergeIndex"];
    delete opts["translations"];
    delete opts["autofocus"];
    delete opts["sort"];

    const dom =
      selector instanceof HTMLElement
        ? selector
        : document.querySelector(selector);
    if (dom) {
      this._pfs = new PagefindSvelte({
        target: dom,
        props: {
          base_path: bundlePath,
          page_size: pageSize,
          reset_styles: resetStyles,
          show_images: showImages,
          show_sub_results: showSubResults,
          excerpt_length: excerptLength,
          process_result: processResult,
          process_term: processTerm,
          show_empty_filters: showEmptyFilters,
          open_filters: openFilters,
          debounce_timeout_ms: debounceTimeoutMs,
          merge_index: mergeIndex,
          translations,
          autofocus,
          sort,
          pagefind_options: opts,
        },
      });
    } else {
      console.error(`Pagefind UI couldn't find the selector ${selector}`);
    }
  }

  triggerSearch(term) {
    this._pfs.$$set({ trigger_search_term: term });
  }

  triggerFilters(filters) {
    let selected_filters = {};
    for (let [filter, key] of Object.entries(filters)) {
      if (Array.isArray(key)) {
        for (let val of key) {
          selected_filters[`${filter}:${val}`] = true;
        }
      } else {
        selected_filters[`${filter}:${key}`] = true;
      }
    }
    this._pfs.$$set({ selected_filters });
  }

  destroy() {
    this._pfs.$destroy();
  }
}
