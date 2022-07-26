<script>
    export let available_filters = null;
    export const selected_filters = {};

    let initialized = false;
    let default_open = false;

    $: if (available_filters && !initialized) {
        initialized = true;
        let filters = Object.entries(available_filters);
        if (filters.length === 1) {
            let values = Object.entries(filters[0][1]);
            if (values.length <= 6) {
                // No need to hide a single filter group with only a few options
                default_open = true;
            }
        }
    }
</script>

{#if !available_filters || Object.entries(available_filters).length}
    <fieldset class="pagefind-ui__filter-panel">
        <legend class="pagefind-ui__filter-panel-label">Filters</legend>
        {#if available_filters}
            {#each Object.entries(available_filters) as [filter, values]}
                <details class="pagefind-ui__filter-block" open={default_open}>
                    <summary class="pagefind-ui__filter-name"
                        >{filter.replace(/^(\w)/, (c) =>
                            c.toLocaleUpperCase()
                        )}</summary
                    >
                    <fieldset class="pagefind-ui__filter-group">
                        <legend class="pagefind-ui__filter-group-label"
                            >{filter}</legend
                        >
                        {#each Object.entries(values) as [value, count]}
                            <div
                                class="pagefind-ui__filter-value"
                                class:pagefind-ui__filter-value--checked={selected_filters[
                                    `${filter}:${value}`
                                ]}
                            >
                                <input
                                    class="pagefind-ui__filter-checkbox"
                                    type="checkbox"
                                    id="{filter}-{value}"
                                    name={filter}
                                    bind:checked={selected_filters[
                                        `${filter}:${value}`
                                    ]}
                                    {value}
                                />
                                <label
                                    class="pagefind-ui__filter-label"
                                    for="{filter}-{value}"
                                    >{value} ({count})</label
                                >
                            </div>
                        {/each}
                    </fieldset>
                </details>
            {/each}
        {:else}
            <p class="pagefind-ui__loading">..........</p>
        {/if}
    </fieldset>
{/if}

<style>
    legend {
        position: absolute;
        clip: rect(0 0 0 0);
    }
    .pagefind-ui__filter-panel {
        min-width: min(calc(260px * var(--pagefind-ui-scale)), 100%);
        flex: 1;
        display: flex;
        flex-direction: column;
        margin-top: calc(20px * var(--pagefind-ui-scale));
    }
    .pagefind-ui__filter-group {
        border: 0;
        padding: 0;
    }
    .pagefind-ui__filter-block {
        padding: 0;
        display: block;
        border-bottom: solid calc(2px * var(--pagefind-ui-scale))
            var(--pagefind-ui-border);
        padding: calc(20px * var(--pagefind-ui-scale)) 0;
    }
    .pagefind-ui__filter-name {
        font-size: calc(16px * var(--pagefind-ui-scale));
        position: relative;
        display: flex;
        align-items: center;
        list-style: none;
        font-weight: 700;
        cursor: pointer;
        height: calc(24px * var(--pagefind-ui-scale));
    }
    .pagefind-ui__filter-name::after {
        position: absolute;
        content: "";
        right: calc(6px * var(--pagefind-ui-scale));
        top: 50%;
        width: calc(8px * var(--pagefind-ui-scale));
        height: calc(8px * var(--pagefind-ui-scale));
        border: solid calc(2px * var(--pagefind-ui-scale)) currentColor;
        border-right: 0;
        border-top: 0;
        transform: translateY(-70%) rotateZ(-45deg);
    }
    .pagefind-ui__filter-block[open] .pagefind-ui__filter-name::after {
        transform: translateY(-70%) rotateZ(-225deg);
    }
    .pagefind-ui__filter-group {
        display: flex;
        flex-direction: column;
        gap: calc(20px * var(--pagefind-ui-scale));
        padding-top: calc(30px * var(--pagefind-ui-scale));
    }
    .pagefind-ui__filter-value {
        position: relative;
        display: flex;
        align-items: center;
        gap: calc(8px * var(--pagefind-ui-scale));
    }
    .pagefind-ui__filter-value::before {
        position: absolute;
        content: "";
        top: 50%;
        left: calc(8px * var(--pagefind-ui-scale));
        width: 0px;
        height: 0px;
        border: solid 1px #fff;
        opacity: 0;
        transform: translate(
                calc(4.5px * var(--pagefind-ui-scale) * -1),
                calc(0.8px * var(--pagefind-ui-scale))
            )
            skewX(-5deg) rotateZ(-45deg);
        transform-origin: top left;
        border-top: 0;
        border-right: 0;
        pointer-events: none;
    }
    .pagefind-ui__filter-value.pagefind-ui__filter-value--checked::before {
        opacity: 1;
        width: calc(9px * var(--pagefind-ui-scale));
        height: calc(4px * var(--pagefind-ui-scale));
        transition: width 0.1s ease-out 0.1s, height 0.1s ease-in;
    }
    .pagefind-ui__filter-checkbox {
        margin: 0;
        width: calc(16px * var(--pagefind-ui-scale));
        height: calc(16px * var(--pagefind-ui-scale));
        border: solid 1px var(--pagefind-ui-border);
        appearance: none;
        -webkit-appearance: none;
        border-radius: calc(var(--pagefind-ui-border-radius) / 2);
        background-color: var(--pagefind-ui-background);
        cursor: pointer;
    }
    .pagefind-ui__filter-checkbox:checked {
        background-color: var(--pagefind-ui-primary);
        border: solid 1px var(--pagefind-ui-primary);
    }
    .pagefind-ui__filter-label {
        cursor: pointer;
        font-size: calc(16px * var(--pagefind-ui-scale));
        font-weight: 400;
    }
    .pagefind-ui__loading {
        height: calc(44px * var(--pagefind-ui-scale));
        margin: 0;
        color: var(--pagefind-ui-text);
        background-color: var(--pagefind-ui-text);
        opacity: 0.1;
        pointer-events: none;
    }
</style>
