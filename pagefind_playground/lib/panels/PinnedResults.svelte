<script lang="ts">
    import Result from "./Result.svelte";

    let {
        pinnedResults,
        toggleResultPin,
    }: {
        pinnedResults: PinnedPagefindSearchResult[];
        toggleResultPin: (
            position: number,
            result: PagefindSearchResult,
        ) => void;
    } = $props();
</script>

<ol>
    {#each pinnedResults as result (result.last_result.id)}
        <Result
            result={result.last_result}
            i={result.position}
            pinned={true}
            toggleResultPin={() =>
                toggleResultPin(result.position, result.last_result)}
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
