<script lang="ts">
    let {
        runSearch,
        searchKeywords,
    }: { runSearch: (term: string) => void; searchKeywords: string[] } =
        $props();

    const handleInput = async (e: Event) => {
        if (e.target instanceof HTMLInputElement) {
            runSearch(e.target.value);
        }
    };
</script>

<label for="#search">Search</label>
<input
    type="search"
    id="search"
    placeholder="Search term..."
    oninput={handleInput}
/>

<p>
    {#if searchKeywords.length}Stemmed search keywords: <code class="kw"
            >{searchKeywords.join(", ")}</code
        >{/if}
    &nbsp;
</p>

<style>
    label {
        position: absolute;
        width: 1px;
        height: 1px;
        padding: 0;
        margin: -1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        border: 0;
    }

    input {
        box-sizing: border-box;
        color: var(--fg);
        border: solid 1px var(--fg);
        background-color: var(--bg);
        padding: 0 6px;
        height: 36px;
        width: 100%;
        font-size: 16px;
    }
    input::placeholder {
        color: var(--fg);
        opacity: 0.5;
    }

    p {
        margin: 8px 0 0 0;
    }

    .kw {
        color: var(--hl);
    }
</style>
