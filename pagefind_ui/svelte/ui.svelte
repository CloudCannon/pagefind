<script>
    import { onMount } from "svelte";
    import { parse as parseBCP47 } from "bcp-47";

    import Result from "./result.svelte";
    import Filters from "./filters.svelte";
    import Reset from "./reset.svelte";

    import * as translationFiles from "../translations/*.json";

    const availableTranslations = {},
        languages = translationFiles.filenames.map(
            (file) => file.match(/([^\/]+)\.json$/)[1]
        );
    for (let i = 0; i < languages.length; i++) {
        availableTranslations[languages[i]] = {
            language: languages[i],
            ...translationFiles.default[i].strings,
        };
    }

    export let base_path = "/_pagefind/";
    export let reset_styles = true;
    export let show_images = true;
    export let process_result = null;
    export let show_empty_filters = true;
    export let debounce_timeout_ms = 300;
    export let pagefind_options = {};
    export let merge_index = [];
    export let trigger_search_term = "";
    export let translations = {};
    export let input_id = null;
    export let enable_input_aria_label = true;

    let val = "";
    $: if (trigger_search_term) {
        val = trigger_search_term;
        trigger_search_term = "";
    }
    let pagefind;
    let initializing = false;

    let searchResult = [];
    let loading = false;
    let searched = false;
    let search_id = 0;
    let search_term = "";
    let show = 5;
    let initial_filters = null;
    let available_filters = null;
    let selected_filters = {};
    let automatic_translations = availableTranslations["en"];

    const translate = (key) => {
        return translations[key] ?? automatic_translations[key] ?? "";
    };

    onMount(() => {
        let lang =
            document?.querySelector?.("html")?.getAttribute?.("lang") || "en";
        let parsedLang = parseBCP47(lang.toLocaleLowerCase());

        automatic_translations =
            availableTranslations[
                `${parsedLang.language}-${parsedLang.script}-${parsedLang.region}`
            ] ||
            availableTranslations[
                `${parsedLang.language}-${parsedLang.region}`
            ] ||
            availableTranslations[`${parsedLang.language}`] ||
            availableTranslations["en"];
    });

    $: debouncedSearch(val, selected_filters);

    const init = async () => {
        if (initializing) return;
        initializing = true;
        if (!pagefind) {
            let imported_pagefind = await import(`${base_path}pagefind.js`);
            await imported_pagefind.options(pagefind_options || {});
            for (const index of merge_index) {
                if (!index.bundlePath) {
                    throw new Error(
                        "mergeIndex requires a bundlePath parameter"
                    );
                }
                const url = index.bundlePath;
                delete index["bundlePath"];
                await imported_pagefind.mergeIndex(url, index);
            }
            pagefind = imported_pagefind;
            loadFilters();
        }
    };

    const loadFilters = async () => {
        if (pagefind) {
            initial_filters = await pagefind.filters();
            if (!available_filters || !Object.keys(available_filters).length) {
                available_filters = initial_filters;
            }
        }
    };

    const parseSelectedFilters = (filters) => {
        let filter = {};
        Object.entries(filters)
            .filter(([, selected]) => selected)
            .forEach(([selection]) => {
                let [key, value] = selection.split(/:(.*)$/);
                filter[key] = filter[key] || [];
                filter[key].push(value);
            });
        return filter;
    };

    let timer;
    const debouncedSearch = async (term, raw_filters) => {
        if (!term) {
            searched = false;
            if (timer) clearTimeout(timer);
            return;
        }

        const filters = parseSelectedFilters(raw_filters);
        const executeSearchFunc = () => search(term, filters);

        if (debounce_timeout_ms > 0 && term) {
            if (timer) clearTimeout(timer);
            timer = setTimeout(executeSearchFunc, debounce_timeout_ms);
            await waitForApiInit();
            pagefind.preload(term, { filters });
        } else {
            executeSearchFunc();
        }
    };

    const waitForApiInit = async () => {
        while (!pagefind) {
            init();
            await new Promise((resolve) => setTimeout(resolve, 50));
        }
    };

    const search = async (term, filters) => {
        search_term = term || "";
        loading = true;
        searched = true;
        await waitForApiInit();

        const local_search_id = ++search_id;
        const results = await pagefind.search(term, { filters });
        if (search_id === local_search_id) {
            if (results.filters && Object.keys(results.filters)?.length) {
                available_filters = results.filters;
            }
            searchResult = results;
            loading = false;
            show = 5;
        }
    };

    const showMore = (e) => {
        e?.preventDefault();
        show += 5;
    };
</script>

<div class="pagefind-ui" class:pagefind-ui--reset={reset_styles}>
    <form
        class="pagefind-ui__form"
        role="search"
        aria-label={translate("search_label")}
        action="javascript:void(0);"
        on:submit={(e) => e.preventDefault()}
    >
        <input
            id={input_id}
            aria-label={(!input_id || enable_input_aria_label) && translate("search_label")}
            class="pagefind-ui__search-input"
            on:focus={init}
            bind:value={val}
            type="text"
            placeholder={translate("placeholder")}
        />

        <div
            class="pagefind-ui__drawer"
            class:pagefind-ui__hidden={!searched}
            aria-live="polite"
        >
            {#if initializing}
                <Filters
                    {show_empty_filters}
                    {available_filters}
                    {translate}
                    bind:selected_filters
                />
            {/if}

            {#if searched}
                <div class="pagefind-ui__results-area">
                    {#if loading}
                        {#if search_term}
                            <p class="pagefind-ui__message">
                                {translate("searching").replace(
                                    /\[SEARCH_TERM\]/,
                                    search_term
                                )}
                            </p>
                        {/if}
                    {:else}
                        <p class="pagefind-ui__message">
                            {#if searchResult.results.length === 0}
                                {translate("zero_results").replace(
                                    /\[SEARCH_TERM\]/,
                                    search_term
                                )}
                            {:else if searchResult.results.length === 1}
                                {translate("one_result")
                                    .replace(/\[SEARCH_TERM\]/, search_term)
                                    .replace(
                                        /\[COUNT\]/,
                                        new Intl.NumberFormat(
                                            translations.language
                                        ).format(1)
                                    )}
                            {:else}
                                {translate("many_results")
                                    .replace(/\[SEARCH_TERM\]/, search_term)
                                    .replace(
                                        /\[COUNT\]/,
                                        new Intl.NumberFormat(
                                            translations.language
                                        ).format(searchResult.results.length)
                                    )}
                            {/if}
                        </p>
                        <ol class="pagefind-ui__results">
                            {#each searchResult.results.slice(0, show) as result (result.id)}
                                <Result
                                    {show_images}
                                    {process_result}
                                    {result}
                                />
                            {/each}
                        </ol>
                        {#if searchResult.results.length > show}
                            <button
                                type="button"
                                class="pagefind-ui__button"
                                on:click={showMore}
                                >{translate("load_more")}</button
                            >
                        {/if}
                    {/if}
                </div>
            {/if}
        </div>
    </form>
</div>

<style>
    :root {
        --pagefind-ui-scale: 0.8;
        --pagefind-ui-primary: #393939;
        --pagefind-ui-text: #393939;
        --pagefind-ui-background: #ffffff;
        --pagefind-ui-border: #eeeeee;
        --pagefind-ui-tag: #eeeeee;
        --pagefind-ui-border-width: 2px;
        --pagefind-ui-border-radius: 8px;
        --pagefind-ui-image-border-radius: 8px;
        --pagefind-ui-image-box-ratio: 3 / 2;
        --pagefind-ui-font: system, -apple-system, ".SFNSText-Regular",
            "San Francisco", "Roboto", "Segoe UI", "Helvetica Neue",
            "Lucida Grande", sans-serif;
    }
    .pagefind-ui {
        width: 100%;
        color: var(--pagefind-ui-text);
        font-family: var(--pagefind-ui-font);
    }
    .pagefind-ui__form {
        position: relative;
    }
    .pagefind-ui__form::before {
        background-color: var(--pagefind-ui-text);
        width: calc(18px * var(--pagefind-ui-scale));
        height: calc(18px * var(--pagefind-ui-scale));
        top: calc(23px * var(--pagefind-ui-scale));
        left: calc(20px * var(--pagefind-ui-scale));
        content: "";
        position: absolute;
        display: block;
        opacity: 0.7;
        -webkit-mask-image: url("data:image/svg+xml,%3Csvg width='18' height='18' viewBox='0 0 18 18' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M12.7549 11.255H11.9649L11.6849 10.985C12.6649 9.845 13.2549 8.365 13.2549 6.755C13.2549 3.165 10.3449 0.255005 6.75488 0.255005C3.16488 0.255005 0.254883 3.165 0.254883 6.755C0.254883 10.345 3.16488 13.255 6.75488 13.255C8.36488 13.255 9.84488 12.665 10.9849 11.685L11.2549 11.965V12.755L16.2549 17.745L17.7449 16.255L12.7549 11.255ZM6.75488 11.255C4.26488 11.255 2.25488 9.245 2.25488 6.755C2.25488 4.26501 4.26488 2.255 6.75488 2.255C9.24488 2.255 11.2549 4.26501 11.2549 6.755C11.2549 9.245 9.24488 11.255 6.75488 11.255Z' fill='%23000000'/%3E%3C/svg%3E%0A");
        mask-image: url("data:image/svg+xml,%3Csvg width='18' height='18' viewBox='0 0 18 18' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M12.7549 11.255H11.9649L11.6849 10.985C12.6649 9.845 13.2549 8.365 13.2549 6.755C13.2549 3.165 10.3449 0.255005 6.75488 0.255005C3.16488 0.255005 0.254883 3.165 0.254883 6.755C0.254883 10.345 3.16488 13.255 6.75488 13.255C8.36488 13.255 9.84488 12.665 10.9849 11.685L11.2549 11.965V12.755L16.2549 17.745L17.7449 16.255L12.7549 11.255ZM6.75488 11.255C4.26488 11.255 2.25488 9.245 2.25488 6.755C2.25488 4.26501 4.26488 2.255 6.75488 2.255C9.24488 2.255 11.2549 4.26501 11.2549 6.755C11.2549 9.245 9.24488 11.255 6.75488 11.255Z' fill='%23000000'/%3E%3C/svg%3E%0A");
        -webkit-mask-size: 100%;
        mask-size: 100%;
        z-index: 9;
        pointer-events: none;
    }
    .pagefind-ui__search-input {
        height: calc(64px * var(--pagefind-ui-scale));
        padding: 0 0 0 calc(54px * var(--pagefind-ui-scale));
        background-color: var(--pagefind-ui-background);
        border: var(--pagefind-ui-border-width) solid var(--pagefind-ui-border);
        border-radius: var(--pagefind-ui-border-radius);
        font-size: calc(21px * var(--pagefind-ui-scale));
        position: relative;
        appearance: none;
        -webkit-appearance: none;
        display: flex;
        width: 100%;
        box-sizing: border-box;
        font-weight: 700;
    }
    .pagefind-ui__search-input::placeholder {
        opacity: 0.2;
    }
    .pagefind-ui__drawer {
        gap: calc(60px * var(--pagefind-ui-scale));
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
    }
    .pagefind-ui__hidden {
        display: none;
    }
    .pagefind-ui__results-area {
        min-width: min(calc(400px * var(--pagefind-ui-scale)), 100%);
        flex: 1000;
        margin-top: calc(20px * var(--pagefind-ui-scale));
    }
    .pagefind-ui__results {
        padding: 0;
    }
    .pagefind-ui__message {
        box-sizing: content-box;
        font-size: calc(16px * var(--pagefind-ui-scale));
        height: calc(24px * var(--pagefind-ui-scale));
        padding: calc(20px * var(--pagefind-ui-scale)) 0;
        display: flex;
        align-items: center;
        font-weight: 700;
        margin-top: 0;
    }
    .pagefind-ui__button {
        margin-top: calc(40px * var(--pagefind-ui-scale));
        border: var(--pagefind-ui-border-width) solid var(--pagefind-ui-border);
        border-radius: var(--pagefind-ui-border-radius);
        height: calc(48px * var(--pagefind-ui-scale));
        padding: 0 calc(12px * var(--pagefind-ui-scale));
        font-size: calc(16px * var(--pagefind-ui-scale));
        color: var(----pagefind-ui-primary);
        background: var(--pagefind-ui-background);
        width: 100%;
        text-align: center;
        font-weight: 700;
        cursor: pointer;
    }
    .pagefind-ui__button:hover {
        border-color: var(--pagefind-ui-primary);
        color: var(--pagefind-ui-primary);
        background: var(--pagefind-ui-background);
    }
</style>
