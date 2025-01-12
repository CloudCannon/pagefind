<script lang="ts">
    import { onMount } from "svelte";

    let { result, i } = $props();

    let resultElement: HTMLLIElement;
    let loadedData: any = $state(null);
    let startedLoading: boolean = $state(false);
    let score = $derived(result.score.toFixed(8));

    onMount(() => {
        const observer = new IntersectionObserver(async (entries) => {
            entries.forEach(async (entry) => {
                if (entry.isIntersecting) {
                    startedLoading = true;
                    loadedData = await result.data();
                }
            });
        });

        observer.observe(resultElement);
    });
</script>

<li class="result" bind:this={resultElement}>
    {#if !startedLoading}
        <code>{i}: [<span class="hl">{score}</span>] unloaded</code>
    {:else if !loadedData}
        <code>{i}: [<span class="hl">{score}</span>] loading...</code>
    {:else}
        <code
            >{i}: [<span class="hl">{score}</span>]
            {loadedData.url} — {loadedData.meta.title}</code
        >

        <details>
            <summary>Statistics</summary>

            <dl>
                <dt>Word count</dt>
                <dd>{loadedData.word_count}</dd>
                <dt>Matched words</dt>
                <dd>{loadedData.locations.length}</dd>
            </dl>
        </details>

        <details>
            <summary>Excerpt</summary>

            <p>{@html loadedData.excerpt}</p>
        </details>

        <details>
            <summary
                >Custom Metadata ({Object.keys(loadedData.meta)
                    .length})</summary
            >

            <dl>
                {#each Object.entries(loadedData.meta) as [metaName, metaValue]}
                    <dt>{metaName}</dt>
                    <dd>{metaValue}</dd>
                {/each}
            </dl>
        </details>

        <details>
            <summary>Anchors ({loadedData.anchors.length})</summary>

            <table>
                <thead>
                    <tr>
                        <th>Node</th>
                        <th>ID attribute</th>
                        <th>Location</th>
                        <th>Text</th>
                    </tr>
                </thead>
                <tbody>
                    {#each loadedData.anchors as anchor}
                        <tr>
                            <td>{anchor.element}</td>
                            <td>{anchor.id}</td>
                            <td>{anchor.location}</td>
                            <td>"{anchor.text}"</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </details>

        <details>
            <summary>Raw Result JSON</summary>

            <pre><code>{JSON.stringify(result, null, 2)}</code></pre>
        </details>

        <details>
            <summary>Raw Data JSON</summary>

            <pre><code>{JSON.stringify(loadedData, null, 2)}</code></pre>
        </details>
    {/if}
</li>

<style>
    .result {
        margin-bottom: 8px;
        padding-bottom: 8px;
        border-bottom: dotted 1px var(--sub-fg);
    }

    details {
        border-left: solid 2px var(--sub-fg);
        padding-left: 8px;
        margin-top: 4px;
    }

    summary {
        font-size: var(--sfz);
        cursor: pointer;
        color: var(--sub-fg);
        list-style-type: none;
    }

    summary::after {
        content: " [+]";
    }

    details[open] {
        border-color: var(--hl);
    }

    details[open] summary {
        color: var(--hl);
    }

    details[open] summary::after {
        content: " [-]";
    }

    tbody tr:nth-child(odd) {
        background-color: transparent;
    }

    tbody tr:nth-child(even) {
        background-color: var(--sub-bg);
    }

    p {
        font-size: var(--fz);
    }

    .hl {
        color: var(--hl);
    }
</style>
