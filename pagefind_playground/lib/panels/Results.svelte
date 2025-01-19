<script lang="ts">
    import Result from "./Result.svelte";

    let {
        results,
        pinnedResults,
        toggleResultPin,
    }: {
        results: PagefindSearchResult[];
        pinnedResults: PinnedPagefindSearchResult[];
        toggleResultPin: (
            position: number,
            result: PagefindSearchResult,
        ) => void;
    } = $props();
</script>

<ol>
    {#each results as result, i (result.id)}
        <Result
            {result}
            {i}
            pinned={pinnedResults.some((r) => r.last_result.id == result.id)}
            toggleResultPin={() => toggleResultPin(i, result)}
        />
    {/each}
</ol>

<style>
    ol {
        list-style-type: none;
        padding: 0;
        margin: 0;
    }
</style>
