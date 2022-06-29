<script>
    export let available_filters = null;
    export const selected_filters = {};
</script>

<fieldset class="pagefind-filter-block">
    <legend>Filters</legend>
    {#if available_filters}
        {#each Object.entries(available_filters) as [filter, values]}
            <details>
                <summary
                    >{filter.replace(/^(\w)/, (c) =>
                        c.toLocaleUpperCase()
                    )}</summary
                >
                <fieldset class="pagefind-filter">
                    <legend>{filter}</legend>
                    {#each Object.entries(values) as [value, count]}
                        <div class="pagefind-filter-input">
                            <input
                                type="checkbox"
                                id="{filter}-{value}"
                                name={filter}
                                bind:checked={selected_filters[
                                    `${filter}:${value}`
                                ]}
                                {value}
                            />
                            <label for="{filter}-{value}"
                                >{value} ({count})</label
                            >
                        </div>
                    {/each}
                </fieldset>
            </details>
        {/each}
    {:else}
        <p class="loading">..........</p>
    {/if}
</fieldset>

<style>
    .pagefind-filter-block {
        flex: 1;
        min-width: 250px;
    }
    fieldset {
        border: 0;
        padding: 0;
    }
    legend {
        position: absolute;
        clip: rect(0 0 0 0);
    }
    details {
        padding: 0;
        display: block;
        max-height: 300px;
        border-top: 1px solid #cfcfcf;
        border-bottom: 1px solid #cfcfcf;
    }
    details + details {
        margin-top: -1px;
    }
    summary {
        position: relative;
        display: flex;
        align-items: center;
        height: 44px;
        list-style: none;
        font-weight: 700;
        font-size: 16px;
        cursor: pointer;
    }
    summary::after {
        position: absolute;
        content: "";
        right: 6px;
        top: 50%;
        width: 8px;
        height: 8px;
        border: solid 2px currentColor;
        border-right: 0;
        border-top: 0;
        transform: translateY(-70%) rotateZ(-45deg);
    }

    .pagefind-filter {
        display: flex;
        flex-direction: column;
        gap: 8px;
        padding: 0 12px 20px;
    }
    .pagefind-filter-input {
        position: relative;
        display: flex;
        align-items: center;
        gap: 8px;
    }
    .pagefind-filter-input::before {
        position: absolute;
        content: "";
        top: 50%;
        left: 8px;
        width: 9px;
        height: 4px;
        border: solid 1px #fff;
        transform: translate(-50%, -70%) skewX(-5deg) rotateZ(-45deg);
        border-top: 0;
        border-right: 0;
        pointer-events: none;
    }
    input[type="checkbox"] {
        margin: 0;
        width: 16px;
        height: 16px;
        border: solid 1px #cfcfcf;
        appearance: none;
        -webkit-appearance: none;
        border-radius: 4px;
        background-color: #fff;
        cursor: pointer;
    }
    input[type="checkbox"]:checked {
        background-color: #034ad8;
        border: solid 1px #034ad8;
    }
    label {
        cursor: pointer;
        font-size: 16px;
    }

    .loading {
        height: 44px;
        margin: 0;
        color: #efefef;
        background-color: #efefef;
        pointer-events: none;
    }
</style>
