<script lang="ts">
    import { onMount } from "svelte";
    import { extract_words } from "../../../pagefind_web_js/lib/excerpt";

    let {
        result,
        i,
        pinned,
        toggleResultPin,
    }: {
        result: PagefindSearchResult;
        i: number;
        pinned: boolean;
        toggleResultPin: () => void;
    } = $props();

    let resultElement: HTMLLIElement;
    let loadedData: PagefindSearchFragment | null = $state(null);
    let startedLoading: boolean = $state(false);
    let score = $derived(result.score.toFixed(8));
    let position = $derived(i === -1 ? "???" : i);
    let ghost = $derived(i === -1);
    let hydratedWords = $derived.by(() => {
        if (loadedData) {
            let words = extract_words(
                loadedData.raw_content ?? loadedData.content,
                loadedData.weighted_locations.map((wl) => wl.location),
            );
            return loadedData.weighted_locations.map((wl, i) => {
                return {
                    weighted: wl,
                    word: words[i],
                };
            });
        } else {
            return [];
        }
    });

    const reloadData = async () => {
        loadedData = await result.data();
    };
    $effect(() => {
        if (result && startedLoading) {
            reloadData();
        }
    });

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

<li class="result" class:ghost bind:this={resultElement}>
    {#if !startedLoading}
        <code>{position}: [<span class="hl">{score}</span>] unloaded</code>
    {:else if !loadedData}
        <code>{position}: [<span class="hl">{score}</span>] loading...</code>
    {:else}
        <code
            >{position}: [<span class="hl"
                >{ghost ? "Last seen: " : ""}{score}</span
            >]
            <button
                class="pinner"
                class:hl={pinned}
                onclick={toggleResultPin}
                aria-label="Pin this result">{pinned ? "★" : "☆"}</button
            >
            {loadedData.url} — {loadedData.meta.title}</code
        >

        <details>
            <summary>Statistics</summary>

            <div class="inner">
                <dl>
                    <dt>Word count</dt>
                    <dd>{loadedData.word_count}</dd>
                    <dt>Matched words</dt>
                    <dd>{loadedData.locations.length}</dd>
                </dl>
            </div>
        </details>

        <details>
            <summary>Excerpt</summary>

            <div class="inner">
                <p>{@html loadedData.excerpt}</p>
            </div>
        </details>

        <details>
            <summary>Matching Words</summary>

            <div class="inner">
                <table>
                    <thead>
                        <tr>
                            <th>Word</th>
                            <th>Indexed</th>
                            <th>Location</th>
                            <th>Weight</th>
                            <th>Bal score</th>
                            <th>Length bonus</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each hydratedWords as hword}
                            <tr>
                                <td>{hword.word}</td>
                                <td>{hword.weighted.verbose?.word_string}</td>
                                <td>{hword.weighted.location}</td>
                                <td>{hword.weighted.weight}</td>
                                <td
                                    >{hword.weighted.balanced_score.toFixed(
                                        6,
                                    )}</td
                                >
                                <td
                                    >{hword.weighted.verbose?.length_bonus?.toFixed?.(
                                        6,
                                    )}</td
                                >
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </details>

        <details>
            <summary>Term Scoring</summary>

            <div class="inner">
                <p>Page score input</p>
                <table>
                    <thead>
                        <tr>
                            <th>Page length</th>
                            <th>Avg length</th>
                            <th>Total pages</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>{result.params?.document_length}</td>
                            <td
                                >{result.params?.average_page_length.toFixed(
                                    6,
                                )}</td
                            >
                            <td>{result.params?.total_pages}</td>
                        </tr>
                    </tbody>
                </table>

                <p>Term score input</p>
                <table>
                    <thead>
                        <tr>
                            <th>Term</th>
                            <th>Weighted TF</th>
                            <th>Matching pages</th>
                            <th>Length bonus</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each result.scores ?? [] as score}
                            <tr>
                                <td>{score.search_term}</td>
                                <td
                                    >{score.params.weighted_term_frequency.toFixed(
                                        2,
                                    )}</td
                                >
                                <td
                                    >{score.params.pages_containing_term} ({(
                                        (score.params.pages_containing_term /
                                            (result.params?.total_pages || 1)) *
                                        100
                                    ).toFixed(2)}%)</td
                                >
                                <td>{score.params.length_bonus.toFixed(6)}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>

                <p>Score output</p>
                <table>
                    <thead>
                        <tr>
                            <th>Term</th>
                            <th>IDF</th>
                            <th>TF (Sat)</th>
                            <th>TF (Raw)</th>
                            <th>Final TF</th>
                            <th>Final score</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each result.scores ?? [] as score}
                            <tr>
                                <td>{score.search_term}</td>
                                <td>{score.idf.toFixed(6)}</td>
                                <td>{score.saturating_tf.toFixed(6)}</td>
                                <td>{score.raw_tf.toFixed(6)}</td>
                                <td>{score.pagefind_tf.toFixed(6)}</td>
                                <td class="hl">{score.score.toFixed(6)}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </details>

        <details>
            <summary
                >Custom Metadata ({Object.keys(loadedData.meta)
                    .length})</summary
            >

            <div class="inner">
                <dl>
                    {#each Object.entries(loadedData.meta) as [metaName, metaValue]}
                        <dt>{metaName}</dt>
                        <dd>{metaValue}</dd>
                    {/each}
                </dl>
            </div>
        </details>

        <details>
            <summary>Anchors ({loadedData.anchors.length})</summary>

            <div class="inner">
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
            </div>
        </details>

        <details>
            <summary>Raw Result JSON</summary>

            <div class="inner">
                <pre><code>{JSON.stringify(result, null, 2)}</code></pre>
            </div>
        </details>

        <details>
            <summary>Raw Data JSON</summary>

            <div class="inner">
                <pre><code>{JSON.stringify(loadedData, null, 2)}</code></pre>
            </div>
        </details>
    {/if}
</li>

<style>
    .result {
        margin-bottom: 8px;
        padding-bottom: 8px;
        border-bottom: dotted 1px var(--sub-fg);
    }

    .ghost {
        opacity: 0.6;
    }

    .pinner {
        border: none;
        background-color: transparent;
        appearance: none;
        cursor: pointer;
        font-size: 16px;
        padding: 0;
        color: var(--sub-fg);
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

    .ghost summary::before {
        content: "Last seen ";
    }

    .inner {
        max-width: 100%;
        overflow-x: scroll;
    }

    details[open] {
        border-color: var(--hl);
    }

    details:has(summary:hover) {
        border-color: var(--hl);
    }

    details[open] summary {
        color: var(--hl);
    }

    details[open] summary::after {
        content: " [-]";
    }

    table {
        border-collapse: collapse;
    }

    tbody tr:nth-child(odd) {
        background-color: transparent;
    }

    tbody tr:nth-child(even) {
        background-color: var(--sub-bg);
    }

    td,
    th {
        padding-right: 8px;
    }

    p {
        font-size: var(--fz);
    }

    .hl {
        color: var(--hl);
    }
</style>
