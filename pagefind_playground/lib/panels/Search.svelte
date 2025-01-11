<script lang="ts">
    let { runSearch } = $props();
</script>

<label for="#search">Search</label>
<input
    type="search"
    id="search"
    placeholder="Search term..."
    oninput={runSearch}
/>

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
        color: var(--fg);
        border: solid 1px var(--fg);
        background-color: var(--bg);
        padding: 6px;
        width: 100%;
        font-size: 16px;
    }
    input:focus {
        border-color: var(--hl);
        outline: none;
    }
    input::placeholder {
        color: var(--fg);
        opacity: 0.5;
    }
</style>
