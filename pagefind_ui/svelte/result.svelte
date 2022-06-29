<script>
    export let result = { data: async () => {} };
    const skipMeta = ["title", "image"];

    let data;
    let meta = [];

    const load = async (r) => {
        data = await r.data();
        meta = Object.entries(data.meta).filter(
            ([key]) => !skipMeta.includes(key)
        );
    };
    $: load(result);

    const placeholder = (max = 30) => {
        return ". ".repeat(Math.floor(10 + Math.random() * max));
    };
</script>

<li class="result">
    {#if data}
        {#if data.meta.image}
            <img class="thumb" src={data.meta?.image} alt={data.meta?.title} />
        {:else}
            <div class="thumb" />
        {/if}
        <div class="details">
            <p class="title"><a href={data.url}>{data.meta?.title}</a></p>
            <p class="excerpt">{@html data.excerpt}</p>
            {#if meta.length}
                <ul>
                    {#each meta as [metaTitle, metaValue]}
                        <li class="meta">
                            {metaTitle.replace(/^(\w)/, (c) =>
                                c.toLocaleUpperCase()
                            )}: {metaValue}
                        </li>
                    {/each}
                </ul>
            {/if}
        </div>
    {:else}
        <div class="thumb" />
        <div class="details">
            <p class="title loading">{placeholder(30)}</p>
            <p class="excerpt loading">{placeholder(40)}</p>
        </div>
    {/if}
</li>

<style>
    .result {
        list-style-type: none;
        display: flex;
        align-items: flex-start;
        gap: min(40px, 3%);
        padding: 30px 0 40px;
        border-top: solid 2px #eee;
    }
    .result:last-of-type {
        border-bottom: solid 2px #eee;
    }
    .thumb {
        width: min(30%, calc((30% - 100px) * 100000));
        max-width: 120px;
        margin-top: 10px;
        aspect-ratio: 3 / 2;
        object-fit: cover;
        background-color: #efefef;
    }
    .details {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        margin-top: 10px;
    }
    .title {
        display: inline-block;
        font-weight: 700;
        font-size: 21px;
        margin-top: 0;
        margin-bottom: 0;
    }
    .title a {
        color: #393939;
        text-decoration: none;
    }
    .title a:hover {
        text-decoration: underline;
    }
    .excerpt {
        display: inline-block;
        font-weight: 400;
        font-size: 16px;
        margin-top: 4px;
        margin-bottom: 0;
        min-width: 250px;
    }
    .loading {
        color: #efefef;
        background-color: #efefef;
        pointer-events: none;
    }
    ul {
        list-style-type: none;
        padding: 0;
        display: flex;
        gap: 20px;
        flex-wrap: wrap;
        margin-top: 20px;
    }
    .meta {
        padding: 4px 8px;
        font-size: 14px;
        border-radius: 8px;
        background-color: #eeeeee;
    }
    :global(.pagefind-ui mark) {
        all: revert;
    }
</style>
