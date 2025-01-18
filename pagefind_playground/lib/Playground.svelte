<script lang="ts">
    // Directly import our search API from a layer behind the public API
    import { Pagefind } from "../../pagefind_web_js/lib/coupled_search";
    import "../../pagefind_web_js/types/index.d.ts";
    import { pagefindRankingDefaults } from "./defaults";
    import Search from "./panels/Search.svelte";
    import TopBar from "./panels/TopBar.svelte";
    import RankingSettings from "./panels/RankingSettings.svelte";
    import RankingPresets from "./panels/RankingPresets.svelte";
    import Results from "./panels/Results.svelte";
    import PinnedResults from "./panels/PinnedResults.svelte";

    import { onMount } from "svelte";

    let { pagefindVersion }: { pagefindVersion: string } = $props();

    let pagefind: Pagefind | null = $state(null);
    let results: PagefindSearchResult[] = $state([]);
    let pinnedResults: PinnedPagefindSearchResult[] = $state([]);
    let currentTerm: string = $state("");
    let searchKeywords: string[] = $state([]);
    let debounceSearches: number = $state(50);

    let rankingSettings: Record<string, number> = $state(
        pagefindRankingDefaults,
    );

    const kickoff = async () => {
        pagefind = new Pagefind({
            // NB: This assumed we are always loaded at `/{pagefind_index}/playground/`
            basePath: "../",
        });
        await pagefind.enterPlaygroundMode();
    };

    const runSearch = async (term: string) => {
        currentTerm = term;
        if (pagefind) {
            const searchResp = await pagefind.debouncedSearch(
                term,
                undefined,
                debounceSearches,
            );
            if (searchResp) {
                results = searchResp.results;
                searchKeywords = searchResp.search_keywords ?? [];
                updateResultPins(searchResp.results);
            }
        }
    };

    const updateSetting = async (name: string, target: number) => {
        if (pagefind) {
            rankingSettings[name] = target;
            await pagefind.options({
                ranking: rankingSettings,
            });
            runSearch(currentTerm);
        }
    };

    const updateSettings = async (target: Record<string, number>) => {
        if (pagefind) {
            rankingSettings = target;
            await pagefind.options({
                ranking: rankingSettings,
            });
            runSearch(currentTerm);
        }
    };

    const toggleResultPin = (
        position: number,
        result: PagefindSearchResult,
    ) => {
        const existingResult = pinnedResults.findIndex(
            (r) => r.last_result.id === result.id,
        );
        if (existingResult === -1) {
            pinnedResults.push({
                position,
                last_result: result,
            });
        } else {
            delete pinnedResults[existingResult];
        }
    };

    const updateResultPins = (results: PagefindSearchResult[]) => {
        for (const pinnedResult of pinnedResults) {
            const newResult = results.findIndex(
                (r) => r.id === pinnedResult.last_result.id,
            );
            pinnedResult.position = newResult;
            if (newResult >= 0) {
                pinnedResult.last_result = results[newResult];
            }
        }
    };

    onMount(() => {
        kickoff();
    });
</script>

<h1 style="grid-area: eyebrow;">Pagefind Playground</h1>

<details open class="panel" style="grid-area: top-bar;">
    <summary>Details</summary>

    <TopBar {pagefindVersion} bind:debounceSearches />
</details>

<details open class="panel" style="grid-area: search;">
    <summary>Search</summary>

    <Search {runSearch} {searchKeywords} />
</details>

<details open class="panel" style="grid-area: ranking-settings;">
    <summary>Ranking Settings</summary>

    <RankingSettings settings={rankingSettings} {updateSetting} />
</details>

<details open class="panel" style="grid-area: ranking-presets;">
    <summary>Ranking Presets</summary>

    <RankingPresets settings={rankingSettings} {updateSettings} />
</details>

<details open class="panel" style="grid-area: pinned-results;">
    <summary>Pinned Results</summary>

    <PinnedResults {pinnedResults} {toggleResultPin} />
</details>

<details open class="panel" style="grid-area: results;">
    <summary>Results</summary>

    <Results {results} {pinnedResults} {toggleResultPin} />
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
    :global(#playground) {
        width: 100%;
        height: 100%;
        display: grid;
        grid-template-areas:
            "eyebrow eyebrow"
            "top-bar top-bar"
            "search search"
            "ranking-settings ranking-presets"
            "pinned-results pinned-results"
            "results results";
        grid-template-rows: auto 1fr auto;
        grid-template-columns: 1fr 1fr;
    }

    @media (max-width: 940px) {
        :global(#playground) {
            grid-template-areas:
                "eyebrow"
                "top-bar"
                "search"
                "ranking-settings"
                "ranking-presets"
                "pinned-results"
                "results";
            grid-template-columns: 1fr;
        }
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
        container-type: inline-size;
    }

    .panel:has(summary:hover) {
        border-color: var(--hl);
        z-index: 9;
    }

    .panel[open] {
        padding: 24px 16px;
    }

    .panel summary {
        z-index: 99;
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
</style>
