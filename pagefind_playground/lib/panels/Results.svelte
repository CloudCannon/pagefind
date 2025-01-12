<script lang="ts">
    import Result from "./Result.svelte";

    let { results } = $props();
</script>

<ol>
    {#each results as result, i (result.id)}
        <Result {result} {i} />
    {/each}
</ol>

<style>
    ol {
        list-style-type: none;
        padding: 0;
        margin: 0;
    }
</style>
