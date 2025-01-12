<script lang="ts">
    let {
        pagefindVersion,
        debounceSearches = $bindable(),
    }: { pagefindVersion: string; debounceSearches: number } = $props();
</script>

<p>Pagefind version: {pagefindVersion}</p>

<div class="row">
    <label for="debounceSearches">Debounce all searches by</label>
    <code>{debounceSearches}ms</code>
    <input
        type="range"
        min="0"
        max="500"
        step="10"
        bind:value={debounceSearches}
    />
</div>

<style>
    p {
        margin: 0 0 16px 0;
    }

    .row {
        max-width: 500px;
        display: grid;
        grid-template-columns: 150px 50px auto;
        align-items: center;
        gap: 8px;
    }

    @container (max-width: 420px) {
        .row {
            grid-template-columns: max-content auto;
        }
        input[type="range"] {
            grid-column: span 2;
        }
    }
</style>
