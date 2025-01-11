<script lang="ts">
    // Directly import our search API from a layer behind the public API
    import { Pagefind } from "../../pagefind_web_js/lib/coupled_search";
    import Search from "./panels/Search.svelte";
    import TopBar from "./panels/TopBar.svelte";
    import Results from "./panels/Results.svelte";

    import { onMount } from "svelte";

    let { pagefindVersion }: { pagefindVersion: string } = $props();

    let pagefind: Pagefind | null = $state(null);
    let results: any[] = $state([]);

    const kickoff = async () => {
        pagefind = new Pagefind({
            // NB: This assumed we are always loaded at `/{pagefind_index}/playground/`
            basePath: "../",
        });

        console.log(pagefind);
    };

    const runSearch = async (e: Event) => {
        if (pagefind && e.target instanceof HTMLInputElement) {
            const searchResp = await pagefind.debouncedSearch(e.target.value);
            if (searchResp) {
                console.log(searchResp);
                results = await Promise.all(
                    searchResp.results.map((r) => r.data()),
                );
            }
        }
    };

    onMount(() => {
        kickoff();
    });
</script>

<h1>Pagefind Playground</h1>

<details open class="panel top-bar">
    <summary>Details</summary>

    <TopBar {pagefindVersion} />
</details>

<details open class="panel search">
    <summary>Search</summary>

    <Search {runSearch} />
</details>

<details open class="panel results">
    <summary>Results</summary>

    <Results {results} />
</details>

<style>
    :global(:root) {
        --bg: #222;
        --sub-bg: #333;
        --fg: #fafafa;
        --sub-fg: #dadada;
        --hl: #ff7f00;
        --fz: 14px;
        --sfz: 12px;
    }
    :global(body) {
        box-sizing: border-box;
        padding: 8px;
        margin: 0;
        width: 100vw;
        min-height: 100vh;
        font-family: ui-monospace, "Cascadia Code", "Source Code Pro", Menlo,
            Consolas, "DejaVu Sans Mono", monospace;
        font-weight: normal;
        background-color: var(--bg);
        color: var(--fg);
        font-size: var(--fz);
    }
    :global(.playground) {
        width: 100%;
        height: 100%;
        display: grid;
        grid-template-areas:
            "top-bar top-bar"
            "search search"
            "results results";
        grid-template-rows: auto 1fr auto;
        grid-template-columns: 200px 1fr;
    }

    h1 {
        font-size: 16px;
        margin: 0 0 16px 0;
        padding: 0;
    }

    .panel {
        min-height: 24px;
        padding: 0;
        border: solid 1px var(--fg);
        margin-top: -1px;
        margin-left: -1px;
        position: relative;
    }

    .panel[open] {
        padding: 16px 8px;
    }

    .panel summary {
        position: absolute;
        top: 0;
        left: 4px;
        padding: 0 4px;
        transform: translateY(-50%);
        list-style-type: none;
        font-size: var(--sfz);
        cursor: pointer;
        background-color: var(--bg);
        color: var(--sub-fg);
    }

    .panel summary::after {
        content: " [+]";
    }

    .panel[open] summary::after {
        content: " [-]";
    }

    .top-bar {
        grid-area: top-bar;
    }
    .search {
        grid-area: search;
    }
    .results {
        grid-area: results;
    }
</style>
