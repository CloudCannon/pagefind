<script>
    import Result from "./result.svelte";
    import Filters from "./filters.svelte";
    import Reset from "./reset.svelte";

    export let base_path = "/_pagefind/";
    export let pagefind_options = {};

    let val = "";
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

    $: search(val, selected_filters);

    const init = async () => {
        if (initializing) return;
        initializing = true;
        if (!pagefind) {
            pagefind = await import(`${base_path}pagefind.js`);
            pagefind.options(pagefind_options || {});
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

    const search = async (term, raw_filters) => {
        const filters = parseSelectedFilters(raw_filters);
        if (!term && !Object.keys(filters).length) {
            searched = false;
            available_filters = initial_filters;
            return;
        }
        search_term = term || "";
        loading = true;
        searched = true;
        while (!pagefind) {
            init();
            await new Promise((resolve) => setTimeout(resolve, 50));
        }
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

    const showMore = () => {
        show += 5;
    };
</script>

<div class="pagefind-ui pagefind-reset">
    <form
        role="search"
        aria-label="Search this site"
        action="javascript:void(0);"
    >
        <input
            on:focus={init}
            bind:value={val}
            type="text"
            placeholder="Search"
        />

        <div class="pagefind-sub">
            {#if initializing}
                <Filters {available_filters} bind:selected_filters />
            {/if}

            <div class="results">
                {#if searched}
                    {#if loading}
                        {#if search_term}
                            <p class="message">
                                Searching for "{search_term}"...
                            </p>
                        {:else}
                            <p class="message">Filtering...</p>
                        {/if}
                    {:else}
                        <p class="message">
                            {searchResult.results.length} result{searchResult
                                .results.length === 1
                                ? ""
                                : "s"}
                            {search_term ? `for "${search_term}"` : ""}
                        </p>
                        <ol>
                            {#each searchResult.results.slice(0, show) as result (result.id)}
                                <Result {result} />
                            {/each}
                        </ol>
                        {#if searchResult.results.length > show}
                            <button on:click={showMore}
                                >Load more results</button
                            >
                        {/if}
                    {/if}
                {/if}
            </div>
        </div>
    </form>
</div>

<style>
    .pagefind-ui {
        width: 100%;
        color: #393939;
        font-family: system, -apple-system, ".SFNSText-Regular", "San Francisco",
            "Roboto", "Segoe UI", "Helvetica Neue", "Lucida Grande", sans-serif;
    }

    form {
        position: relative;
    }

    input {
        position: relative;
        appearance: none;
        -webkit-appearance: none;
        display: flex;
        width: 100%;
        height: 64px;
        background-color: #fff;
        padding: 0 0 0 54px;
        border: 2px solid #eeeeee;
        border-radius: 8px;
        box-sizing: border-box;
        font-weight: 700;
        font-size: 21px;
    }

    input::placeholder {
        opacity: 0.2;
    }

    form::before {
        content: "";
        position: absolute;
        display: block;
        background-color: #757575;
        mask-image: url("data:image/svg+xml,%3Csvg width='18' height='18' viewBox='0 0 18 18' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M12.7549 11.255H11.9649L11.6849 10.985C12.6649 9.845 13.2549 8.365 13.2549 6.755C13.2549 3.165 10.3449 0.255005 6.75488 0.255005C3.16488 0.255005 0.254883 3.165 0.254883 6.755C0.254883 10.345 3.16488 13.255 6.75488 13.255C8.36488 13.255 9.84488 12.665 10.9849 11.685L11.2549 11.965V12.755L16.2549 17.745L17.7449 16.255L12.7549 11.255ZM6.75488 11.255C4.26488 11.255 2.25488 9.245 2.25488 6.755C2.25488 4.26501 4.26488 2.255 6.75488 2.255C9.24488 2.255 11.2549 4.26501 11.2549 6.755C11.2549 9.245 9.24488 11.255 6.75488 11.255Z' fill='%23000000'/%3E%3C/svg%3E%0A");
        width: 18px;
        height: 18px;
        top: 23px;
        left: 20px;
        z-index: 9;
        pointer-events: none;
    }

    .pagefind-sub {
        display: flex;
        flex-direction: row;
        gap: 15px;
        flex-wrap: wrap;
        margin-top: 40px;
    }

    .results {
        flex: 2;
        min-width: min(400px, 100%);
    }

    ol {
        padding: 0;
    }

    .message {
        font-size: 16px;
        font-weight: 700;
        margin-top: 0;
    }

    button {
        margin-top: 40px;
        border: 2px solid #cfcfcf;
        border-radius: 8px;
        height: 48px;
        padding: 0 12px;
        color: #034ad8;
        font-weight: 700;
        font-size: 16px;
        cursor: pointer;
        background: #fff;
    }

    button:hover {
        border-color: #034ad8;
        background: #fff;
    }
</style>
