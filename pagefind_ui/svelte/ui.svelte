<script>
    import Result from "./result.svelte";

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

    $: search(val);

    const init = async () => {
        if (initializing) return;
        initializing = true;
        if (!pagefind) {
            pagefind = await import(`${base_path}pagefind.js`);
            pagefind.options(pagefind_options || {});
        }
    };

    const search = async (term) => {
        if (!term) return;
        search_term = term;
        loading = true;
        searched = true;
        while (!pagefind) {
            init();
            await new Promise((resolve) => setTimeout(resolve, 100));
        }
        const local_search_id = Math.random();
        search_id = local_search_id;
        const results = await pagefind.search(term);
        if (search_id === local_search_id) {
            searchResult = results;
            loading = false;
        }
    };
</script>

<div class="pagefind-ui">
    <input on:focus={init} bind:value={val} type="text" placeholder="Search" />

    {#if searched}
        {#if loading}
            <p class="message">Searching for {search_term}...</p>
        {:else}
            <p class="message">
                {searchResult.results.length} search result{searchResult.results
                    .length === 1
                    ? ""
                    : "s"}
            </p>
            <ul>
                {#each searchResult.results.slice(0, show) as result (result.id)}
                    <Result {result} />
                {/each}
            </ul>
        {/if}
    {/if}
</div>

<style>
    .pagefind-ui {
        width: 100%;
        max-width: 800px;
        margin: 0 auto;
        font-family: system, -apple-system, ".SFNSText-Regular", "San Francisco",
            "Roboto", "Segoe UI", "Helvetica Neue", "Lucida Grande", sans-serif;
    }

    input {
        display: block;
        width: 100%;
        -webkit-appearance: none;
        background-color: #efefef;
        color: #444;
        margin-right: 6px;
        margin-bottom: 6px;
        padding: 10px;
        border: none;
        border-radius: 6px;
        outline: none;
    }

    ul {
        padding: 0;
    }
</style>
