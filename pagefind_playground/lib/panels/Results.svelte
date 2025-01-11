<script lang="ts">
    let { results } = $props();
</script>

<ol>
    {#each results as result, i}
        <li class="result">
            <code>{i}: {result.url} — {result.meta.title}</code>

            <details>
                <summary>Statistics</summary>

                <dl>
                    <dt>Word count</dt>
                    <dd>{result.word_count}</dd>
                    <dt>Matched words</dt>
                    <dd>{result.locations.length}</dd>
                </dl>
            </details>

            <details>
                <summary
                    >Custom Metadata ({Object.keys(result.meta)
                        .length})</summary
                >

                <dl>
                    {#each Object.entries(result.meta) as [metaName, metaValue]}
                        <dt>{metaName}</dt>
                        <dd>{metaValue}</dd>
                    {/each}
                </dl>
            </details>

            <details>
                <summary>Anchors ({result.anchors.length})</summary>

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
                        {#each result.anchors as anchor}
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
                <summary>Raw JSON</summary>

                <pre><code>{JSON.stringify(result, null, 2)}</code></pre>
            </details>
        </li>
    {/each}
</ol>

<style>
    ol {
        list-style-type: none;
        padding: 0;
        margin: 0;
    }

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
</style>
