<script lang="ts">
    import type * as internal from "../../../pagefind_web_js/types/internal.d.ts";

    let {
        pagefindVersion,
        loadedPagefindVersion,
        loadedPagefindLanguage,
        availablePagefindLanguages,
        loadLanguage,
        debounceSearches = $bindable(),
    }: {
        pagefindVersion: string;
        loadedPagefindVersion: string;
        loadedPagefindLanguage: string;
        availablePagefindLanguages: Record<
            string,
            internal.PagefindEntryLanguage
        >;
        loadLanguage: (lang: string) => void;
        debounceSearches: number;
    } = $props();
</script>

<table>
    <thead>
        <tr>
            <th>Playground ver.</th>
            <th>Bundle ver.</th>
            <th>Loaded language</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>{pagefindVersion}</td>
            <td>{loadedPagefindVersion}</td>
            <td>{loadedPagefindLanguage}</td>
        </tr>
    </tbody>
</table>

<p>Available languages:</p>

<table>
    <thead>
        <tr>
            <th>Action</th>
            <th>Language</th>
            <th>Hash</th>
            <th>Wasm</th>
            <th>Page count</th>
        </tr>
    </thead>
    <tbody>
        {#each Object.entries(availablePagefindLanguages) as [lang, detail]}
            <tr>
                <td
                    ><button
                        disabled={loadedPagefindLanguage === lang}
                        onclick={() => loadLanguage(lang)}>Load</button
                    ></td
                >
                <td>{lang}</td>
                <td>{detail.hash}</td>
                <td>{detail.wasm}</td>
                <td>{detail.page_count}</td>
            </tr>
        {/each}
    </tbody>
</table>

<hr />

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
        margin: 0 0 4px 0;
    }

    table {
        border-collapse: collapse;
        margin-bottom: 16px;
    }

    button {
        color: var(--fg);
        border: solid 1px var(--fg);
        background-color: var(--bg);
        padding: 0 12px;
        height: 24px;
        cursor: pointer;
    }
    button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    tbody tr:nth-child(odd) {
        background-color: transparent;
    }

    tbody tr:nth-child(even) {
        background-color: var(--sub-bg);
    }

    th {
        text-align: left;
    }

    td,
    th {
        padding-right: 12px;
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
