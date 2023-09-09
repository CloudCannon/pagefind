<script>
  export let show_images = true;
  export let process_result = null;
  export let result = { data: async () => {} };

  const skipMeta = ["title", "image", "image_alt", "url"];

  let data;
  let meta = [];
  let non_root_sub_results = [];
  let has_root_sub_result = false;

  const thin_sub_results = (results, limit) => {
    if (results.length <= limit) {
      return results;
    }

    const top_results = [...results]
      .sort((a, b) => b.locations.length - a.locations.length)
      .slice(0, 3)
      .map((r) => r.url);

    return results.filter((r) => top_results.includes(r.url));
  };

  const load = async (r) => {
    data = await r.data();
    data = process_result?.(data) ?? data;
    meta = Object.entries(data.meta).filter(([key]) => !skipMeta.includes(key));
    if (Array.isArray(data.sub_results)) {
      has_root_sub_result =
        data.sub_results?.[0]?.url === (data.meta?.url || data.url);
      if (has_root_sub_result) {
        non_root_sub_results = thin_sub_results(data.sub_results.slice(1), 3);
      } else {
        non_root_sub_results = thin_sub_results([...data.sub_results], 3);
      }
    }
  };
  $: load(result);

  const placeholder = (max = 30) => {
    return ". ".repeat(Math.floor(10 + Math.random() * max));
  };
</script>

<li class="pagefind-ui__result">
  {#if data}
    {#if show_images}
      <div class="pagefind-ui__result-thumb">
        {#if data.meta.image}
          <img
            class="pagefind-ui__result-image"
            src={data.meta?.image}
            alt={data.meta?.image_alt || data.meta?.title}
          />
        {/if}
      </div>
    {/if}
    <div class="pagefind-ui__result-inner">
      <p class="pagefind-ui__result-title">
        <a class="pagefind-ui__result-link" href={data.meta?.url || data.url}
          >{data.meta?.title}</a
        >
      </p>
      {#if has_root_sub_result}
        <p class="pagefind-ui__result-excerpt">{@html data.excerpt}</p>
      {/if}

      {#each non_root_sub_results as subres}
        <div class="pagefind-ui__result-nested">
          <p class="pagefind-ui__result-title">
            <a class="pagefind-ui__result-link" href={subres.url}
              >{subres.title}</a
            >
          </p>
          <p class="pagefind-ui__result-excerpt">{@html subres.excerpt}</p>
        </div>
      {/each}

      {#if meta.length}
        <ul class="pagefind-ui__result-tags">
          {#each meta as [metaTitle, metaValue]}
            <li class="pagefind-ui__result-tag">
              {metaTitle.replace(/^(\w)/, (c) => c.toLocaleUpperCase())}: {metaValue}
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  {:else}
    {#if show_images}
      <div class="pagefind-ui__result-thumb pagefind-ui__loading" />
    {/if}
    <div class="pagefind-ui__result-inner">
      <p class="pagefind-ui__result-title pagefind-ui__loading">
        {placeholder(30)}
      </p>
      <p class="pagefind-ui__result-excerpt pagefind-ui__loading">
        {placeholder(40)}
      </p>
    </div>
  {/if}
</li>

<style>
  .pagefind-ui__result {
    list-style-type: none;
    display: flex;
    align-items: flex-start;
    gap: min(calc(40px * var(--pagefind-ui-scale)), 3%);
    padding: calc(30px * var(--pagefind-ui-scale)) 0
      calc(40px * var(--pagefind-ui-scale));
    border-top: solid var(--pagefind-ui-border-width) var(--pagefind-ui-border);
  }
  .pagefind-ui__result:last-of-type {
    border-bottom: solid var(--pagefind-ui-border-width)
      var(--pagefind-ui-border);
  }
  .pagefind-ui__result-nested {
    display: flex;
    flex-direction: column;
    padding-left: calc(20px * var(--pagefind-ui-scale));
  }
  .pagefind-ui__result-nested:first-of-type {
    padding-top: calc(10px * var(--pagefind-ui-scale));
  }
  .pagefind-ui__result-nested .pagefind-ui__result-link {
    font-size: 0.9em;
    position: relative;
  }
  .pagefind-ui__result-nested .pagefind-ui__result-link::before {
    content: "⤷ ";
    position: absolute;
    top: 0;
    right: calc(100% + 0.1em);
  }
  .pagefind-ui__result-thumb {
    width: min(30%, calc((30% - (100px * var(--pagefind-ui-scale))) * 100000));
    max-width: calc(120px * var(--pagefind-ui-scale));
    margin-top: calc(10px * var(--pagefind-ui-scale));
    aspect-ratio: var(--pagefind-ui-image-box-ratio);
    position: relative;
  }
  .pagefind-ui__result-image {
    display: block;
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    font-size: 0;
    width: auto;
    height: auto;
    max-width: 100%;
    max-height: 100%;
    border-radius: var(--pagefind-ui-image-border-radius);
  }
  .pagefind-ui__result-inner {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    margin-top: calc(10px * var(--pagefind-ui-scale));
  }
  .pagefind-ui__result-title {
    display: inline-block;
    font-weight: 700;
    font-size: calc(21px * var(--pagefind-ui-scale));
    margin-top: 0;
    margin-bottom: 0;
  }
  .pagefind-ui__result-title .pagefind-ui__result-link {
    color: var(--pagefind-ui-text);
    text-decoration: none;
  }
  .pagefind-ui__result-title .pagefind-ui__result-link:hover {
    text-decoration: underline;
  }
  .pagefind-ui__result-excerpt {
    display: inline-block;
    font-weight: 400;
    font-size: calc(16px * var(--pagefind-ui-scale));
    margin-top: calc(4px * var(--pagefind-ui-scale));
    margin-bottom: 0;
    min-width: calc(250px * var(--pagefind-ui-scale));
  }
  .pagefind-ui__loading {
    color: var(--pagefind-ui-text);
    background-color: var(--pagefind-ui-text);
    border-radius: var(--pagefind-ui-border-radius);
    opacity: 0.1;
    pointer-events: none;
  }
  .pagefind-ui__result-tags {
    list-style-type: none;
    padding: 0;
    display: flex;
    gap: calc(20px * var(--pagefind-ui-scale));
    flex-wrap: wrap;
    margin-top: calc(20px * var(--pagefind-ui-scale));
  }
  .pagefind-ui__result-tag {
    padding: calc(4px * var(--pagefind-ui-scale))
      calc(8px * var(--pagefind-ui-scale));
    font-size: calc(14px * var(--pagefind-ui-scale));
    border-radius: var(--pagefind-ui-border-radius);
    background-color: var(--pagefind-ui-tag);
  }
</style>
