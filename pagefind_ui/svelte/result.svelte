<script>
    export let result = { data: async () => {} };

    let data;

    const load = async (r) => {
        data = await r.data();
    };
    $: load(result);

    const placeholder = (max = 30) => {
        return ". ".repeat(Math.floor(10 + Math.random() * max));
    };
</script>

<li class="result">
    {#if data}
        {#if data.meta.image}
            <img class="thumb" src={data.meta?.image} alt={data.title} />
        {:else}
            <div class="thumb" />
        {/if}
        <div class="details">
            <p class="title"><a href={data.url}>{data.title}</a></p>
            <p class="excerpt">{@html data.excerpt}</p>
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
        gap: 10px;
        padding: 10px 0;
    }
    .thumb {
        width: 30%;
        max-width: 150px;
        aspect-ratio: 16 / 9;
        object-fit: cover;
        background-color: #efefef;
    }
    .details {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }
    .title {
        display: inline-block;
        font-weight: bold;
        font-size: 20px;
        margin-top: 0;
        margin-bottom: 0;
    }
    .title a {
        color: #034ad8;
        text-decoration: none;
    }
    .excerpt {
        display: inline-block;
        font-size: 12px;
        margin-top: 6px;
        margin-bottom: 0;
    }
    .loading {
        color: #efefef;
        background-color: #efefef;
        pointer-events: none;
    }
</style>
