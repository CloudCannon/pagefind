declare var wasm_bindgen: any;
declare var pagefind_version: string;

import type * as internal from "pagefindWebInternal";

import gunzip from "./gz.js";
import { build_excerpt, calculate_excerpt_region } from "./excerpt";
import { calculate_sub_results } from "./sub_results.js";

const asyncSleep = async (ms = 100) => {
  return new Promise((r) => setTimeout(r, ms));
};

export class PagefindInstance {
  backend: any;
  decoder: TextDecoder;
  wasm: any;

  basePath: string;
  baseUrl: string;
  primary: boolean;
  indexWeight: number;
  excerptLength: number;
  mergeFilter: Object;
  ranking?: PagefindRankingWeights;
  highlightParam: string | null;

  loaded_chunks: Record<string, Promise<void>>;
  loaded_filters: Record<string, Promise<void>>;
  loaded_fragments: Record<string, Promise<PagefindSearchFragment>>;

  raw_ptr: number | null;
  searchMeta: any;
  languages: Record<string, internal.PagefindEntryLanguage> | null;
  loadedLanguage?: string;
  includeCharacters?: string[];

  version: string;
  loadedVersion?: string;

  constructor(opts: PagefindIndexOptions = {}) {
    this.version = pagefind_version;
    this.backend = wasm_bindgen;

    this.decoder = new TextDecoder("utf-8");
    this.wasm = null;

    this.basePath = opts.basePath || "/pagefind/";
    this.primary = opts.primary || false;
    if (this.primary && !opts.basePath) {
      this.initPrimary();
    }
    if (/[^\/]$/.test(this.basePath)) {
      this.basePath = `${this.basePath}/`;
    }
    if (
      window?.location?.origin &&
      this.basePath.startsWith(window.location.origin)
    ) {
      this.basePath = this.basePath.replace(window.location.origin, "");
    }

    this.baseUrl = opts.baseUrl || this.defaultBaseUrl();
    if (!/^(\/|https?:\/\/)/.test(this.baseUrl)) {
      this.baseUrl = `/${this.baseUrl}`;
    }

    this.indexWeight = opts.indexWeight ?? 1;
    this.excerptLength = opts.excerptLength ?? 30;
    this.mergeFilter = opts.mergeFilter ?? {};
    this.ranking = opts.ranking;
    this.highlightParam = opts.highlightParam ?? null;

    this.loaded_chunks = {};
    this.loaded_filters = {};
    this.loaded_fragments = {};

    this.raw_ptr = null;
    this.searchMeta = null;
    this.languages = null;
  }

  initPrimary() {
    let derivedBasePath = import.meta.url.match(/^(.*\/)pagefind.js.*$/)?.[1];
    if (derivedBasePath) {
      this.basePath = derivedBasePath;
    } else {
      console.warn(
        [
          "Pagefind couldn't determine the base of the bundle from the import path. Falling back to the default.",
          "Set a basePath option when initialising Pagefind to ignore this message.",
        ].join("\n"),
      );
    }
  }

  defaultBaseUrl() {
    let default_base = this.basePath.match(/^(.*\/)_?pagefind/)?.[1];
    return default_base || "/";
  }

  async options(options: PagefindIndexOptions) {
    const opts = [
      "basePath",
      "baseUrl",
      "indexWeight",
      "excerptLength",
      "mergeFilter",
      "highlightParam",
      "ranking",
    ];
    for (const [k, v] of Object.entries(options)) {
      if (k === "mergeFilter") {
        let filters = this.stringifyFilters(v);
        let ptr = await this.getPtr();
        this.raw_ptr = this.backend.add_synthetic_filter(ptr, filters);
      } else if (k === "ranking") {
        await this.set_ranking(options.ranking);
      } else if (opts.includes(k)) {
        if (k === "basePath" && typeof v === "string") this.basePath = v;
        if (k === "baseUrl" && typeof v === "string") this.baseUrl = v;
        if (k === "indexWeight" && typeof v === "number") this.indexWeight = v;
        if (k === "excerptLength" && typeof v === "number")
          this.excerptLength = v;
        if (k === "mergeFilter" && typeof v === "object") this.mergeFilter = v;
        if (k === "highlightParam" && typeof v === "string")
          this.highlightParam = v;
      } else {
        console.warn(
          `Unknown Pagefind option ${k}. Allowed options: [${opts.join(", ")}]`,
        );
      }
    }
  }

  async enterPlaygroundMode() {
    let ptr = await this.getPtr();
    this.raw_ptr = this.backend.enter_playground_mode(ptr);
  }

  decompress(data: Uint8Array, file = "unknown file") {
    if (this.decoder.decode(data.slice(0, 12)) === "pagefind_dcd") {
      // File is already decompressed
      return data.slice(12);
    }
    data = gunzip(data);
    if (this.decoder.decode(data.slice(0, 12)) !== "pagefind_dcd") {
      // Decompressed file does not have the correct signature
      console.error(
        `Decompressing ${file} appears to have failed: Missing signature`,
      );
      return data;
    }
    return data.slice(12);
  }

  async set_ranking(ranking?: PagefindRankingWeights) {
    if (!ranking) return;

    let rankingWeights = {
      term_similarity: ranking.termSimilarity ?? null,
      page_length: ranking.pageLength ?? null,
      term_saturation: ranking.termSaturation ?? null,
      term_frequency: ranking.termFrequency ?? null,
    };
    let ptr = await this.getPtr();
    this.raw_ptr = this.backend.set_ranking_weights(
      ptr,
      JSON.stringify(rankingWeights),
    );
  }

  async init(language: string, opts: { load_wasm: boolean }) {
    await this.loadEntry();
    let index = this.findIndex(language);
    let lang_wasm = index.wasm ? index.wasm : "unknown";
    this.loadedLanguage = language;

    let resources = [this.loadMeta(index.hash)];
    if (opts.load_wasm === true) {
      resources.push(this.loadWasm(lang_wasm));
    }
    await Promise.all(resources);
    this.raw_ptr = this.backend.init_pagefind(new Uint8Array(this.searchMeta));

    if (Object.keys(this.mergeFilter)?.length) {
      let filters = this.stringifyFilters(this.mergeFilter);
      let ptr = await this.getPtr();
      this.raw_ptr = this.backend.add_synthetic_filter(ptr, filters);
    }
    if (this.ranking) {
      await this.set_ranking(this.ranking);
    }
  }

  async loadEntry() {
    try {
      // We always load a fresh copy of the entry metadata,
      // as it ensures we don't try to load an old build's chunks,
      let entry_response = await fetch(
        `${this.basePath}pagefind-entry.json?ts=${Date.now()}`,
      );
      let entry_json =
        (await entry_response.json()) as internal.PagefindEntryJson;
      this.languages = entry_json.languages;
      this.loadedVersion = entry_json.version;
      this.includeCharacters = entry_json.include_characters ?? [];
      if (entry_json.version !== this.version) {
        if (this.primary) {
          console.warn(
            [
              "Pagefind JS version doesn't match the version in your search index.",
              `Pagefind JS: ${this.version}. Pagefind index: ${entry_json.version}`,
              "If you upgraded Pagefind recently, you likely have a cached pagefind.js file.",
              "If you encounter any search errors, try clearing your cache.",
            ].join("\n"),
          );
        } else {
          console.warn(
            [
              "Merging a Pagefind index from a different version than the main Pagefind instance.",
              `Main Pagefind JS: ${this.version}. Merged index (${this.basePath}): ${entry_json.version}`,
              "If you encounter any search errors, make sure that both sites are running the same version of Pagefind.",
            ].join("\n"),
          );
        }
      }
    } catch (e) {
      console.error(`Failed to load Pagefind metadata:\n${e?.toString()}`);
      throw new Error("Failed to load Pagefind metadata");
    }
  }

  findIndex(language: string) {
    if (this.languages) {
      let index = this.languages[language];
      if (index) return index;

      index = this.languages[language.split("-")[0]];
      if (index) return index;

      let topLang = Object.values(this.languages).sort(
        (a, b) => b.page_count - a.page_count,
      );
      if (topLang[0]) return topLang[0];
    }

    throw new Error("Pagefind Error: No language indexes found.");
  }

  async loadMeta(index: string) {
    try {
      let compressed_resp = await fetch(
        `${this.basePath}pagefind.${index}.pf_meta`,
      );
      let compressed_meta = await compressed_resp.arrayBuffer();
      this.searchMeta = this.decompress(
        new Uint8Array(compressed_meta),
        "Pagefind metadata",
      );
    } catch (e) {
      console.error(`Failed to load the meta index:\n${e?.toString()}`);
    }
  }

  async loadWasm(language: string) {
    try {
      const wasm_url = `${this.basePath}wasm.${language}.pagefind`;
      let compressed_resp = await fetch(wasm_url);
      let compressed_wasm = await compressed_resp.arrayBuffer();
      const final_wasm = this.decompress(
        new Uint8Array(compressed_wasm),
        "Pagefind WebAssembly",
      );
      if (!final_wasm) {
        throw new Error("No WASM after decompression");
      }
      this.wasm = await this.backend(final_wasm);
    } catch (e) {
      console.error(`Failed to load the Pagefind WASM:\n${e?.toString()}`);
      throw new Error(`Failed to load the Pagefind WASM:\n${e?.toString()}`);
    }
  }

  async _loadGenericChunk(url: string, method: string) {
    try {
      let compressed_resp = await fetch(url);
      let compressed_chunk = await compressed_resp.arrayBuffer();
      let chunk = this.decompress(new Uint8Array(compressed_chunk), url);

      let ptr = await this.getPtr();
      this.raw_ptr = this.backend[method](ptr, chunk);
    } catch (e) {
      console.error(`Failed to load the index chunk ${url}:\n${e?.toString()}`);
    }
  }

  async loadChunk(hash: string) {
    if (!this.loaded_chunks[hash]) {
      const url = `${this.basePath}index/${hash}.pf_index`;
      this.loaded_chunks[hash] = this._loadGenericChunk(
        url,
        "load_index_chunk",
      );
    }
    return await this.loaded_chunks[hash];
  }

  async loadFilterChunk(hash: string) {
    if (!this.loaded_filters[hash]) {
      const url = `${this.basePath}filter/${hash}.pf_filter`;
      this.loaded_filters[hash] = this._loadGenericChunk(
        url,
        "load_filter_chunk",
      );
    }
    return await this.loaded_filters[hash];
  }

  async _loadFragment(hash: string) {
    let compressed_resp = await fetch(
      `${this.basePath}fragment/${hash}.pf_fragment`,
    );
    let compressed_fragment = await compressed_resp.arrayBuffer();
    let fragment = this.decompress(
      new Uint8Array(compressed_fragment),
      `Fragment ${hash}`,
    );
    return JSON.parse(new TextDecoder().decode(fragment));
  }

  async loadFragment(
    hash: string,
    weighted_locations: PagefindWordLocation[] = [],
    search_term: string,
  ) {
    if (!this.loaded_fragments[hash]) {
      this.loaded_fragments[hash] = this._loadFragment(hash);
    }
    let fragment = (await this.loaded_fragments[
      hash
    ]) as PagefindSearchFragment & {
      raw_content: string;
      raw_url: string;
    };
    fragment.weighted_locations = weighted_locations;
    fragment.locations = weighted_locations.map((l) => l.location);

    if (!fragment.raw_content) {
      fragment.raw_content = fragment.content
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;");
      fragment.content = fragment.content.replace(/\u200B/g, "");
    }
    if (!fragment.raw_url) {
      fragment.raw_url = fragment.url;
    }
    fragment.url = this.processedUrl(fragment.raw_url, search_term);

    const excerpt_start = calculate_excerpt_region(
      weighted_locations,
      this.excerptLength,
    );
    fragment.excerpt = build_excerpt(
      fragment.raw_content,
      excerpt_start,
      this.excerptLength,
      fragment.locations,
    );

    fragment.sub_results = calculate_sub_results(fragment, this.excerptLength);

    return fragment;
  }

  fullUrl(raw: string) {
    // Avoid processing absolute URLs
    if (/^(https?:)?\/\//.test(raw)) {
      return raw;
    }
    return `${this.baseUrl}/${raw}`
      .replace(/\/+/g, "/")
      .replace(/^(https?:\/)/, "$1/");
  }

  processedUrl(url: string, search_term: string) {
    const normalized = this.fullUrl(url);
    if (this.highlightParam === null) {
      return normalized;
    }
    let individual_terms = search_term.split(/\s+/);
    try {
      // This will error is it is not a FQDN
      let processed = new URL(normalized);
      for (const term of individual_terms) {
        processed.searchParams.append(this.highlightParam, term);
      }
      return processed.toString();
    } catch (e) {
      try {
        let processed = new URL(`https://example.com${normalized}`);
        for (const term of individual_terms) {
          processed.searchParams.append(this.highlightParam, term);
        }
        return processed.toString().replace(/^https:\/\/example\.com/, "");
      } catch (e) {
        return normalized;
      }
    }
  }

  async getPtr() {
    while (this.raw_ptr === null) {
      await asyncSleep(50);
    }
    if (!this.raw_ptr) {
      console.error("Pagefind: WASM Error (No pointer)");
      throw new Error("Pagefind: WASM Error (No pointer)");
    }
    return this.raw_ptr;
  }

  stringifyFilters(obj = {}) {
    return JSON.stringify(obj);
  }

  stringifySorts(obj = {}) {
    let sorts = Object.entries(obj);
    // We currently only support one sort directive,
    // so we'll grab the first sort provided in the object.
    for (let [sort, direction] of sorts) {
      if (sorts.length > 1) {
        console.warn(
          `Pagefind was provided multiple sort options in this search, but can only operate on one. Using the ${sort} sort.`,
        );
      }
      if (direction !== "asc" && direction !== "desc") {
        console.warn(
          `Pagefind was provided a sort with unknown direction ${direction}. Supported: [asc, desc]`,
        );
      }
      return `${sort}:${direction}`;
    }

    return ``;
  }

  async filters() {
    let ptr = await this.getPtr();

    let filters = this.backend.request_all_filter_indexes(ptr) as string;
    let filter_array = JSON.parse(filters);
    if (Array.isArray(filter_array)) {
      let filter_chunks = filter_array
        .filter((v) => v)
        .map((chunk) => this.loadFilterChunk(chunk));
      await Promise.all([...filter_chunks]);
    }

    // pointer may have updated from the loadChunk calls
    ptr = await this.getPtr();

    let results = this.backend.filters(ptr) as string;
    return JSON.parse(results) as PagefindFilterCounts;
  }

  async preload(term: string, options: PagefindSearchOptions = {}) {
    await this.search(term, {
      ...options,
      preload: true,
    });
  }

  async search(
    term: string,
    options: PagefindSearchOptions = {},
  ): Promise<PagefindSearchResults | null> {
    options = {
      verbose: false,
      filters: {},
      sort: {},
      ...options,
    };
    const log = (str: string) => {
      if (options.verbose) console.log(str);
    };
    log(`Starting search on ${this.basePath}`);
    let start = Date.now();
    let ptr = await this.getPtr();
    let filter_only = term === null;
    term = term ?? "";
    let exact_search = /^\s*".+"\s*$/.test(term);
    if (exact_search) {
      log(`Running an exact search`);
    }

    let trueLanguage: string | null = null;
    try {
      trueLanguage = Intl.getCanonicalLocales(this.loadedLanguage)[0];
    } catch (err) {
      // Loaded language is not valid
    }
    const term_chunks: string[] = [];
    let segments: string[];

    // TODO: resolve type error for Intl.Segmenter
    //@ts-ignore: Property 'Segmenter' does not exist on type 'typeof Intl'
    if (trueLanguage && typeof Intl.Segmenter !== "undefined") {
      //@ts-ignore: Property 'Segmenter' does not exist on type 'typeof Intl'
      const segmenter = new Intl.Segmenter(trueLanguage, {
        granularity: "grapheme",
      });
      segments = [...segmenter.segment(term)].map(
        ({ segment }: { segment: string }) => segment,
      );
    } else {
      segments = [...term];
    }

    for (const segment of segments) {
      if (this.includeCharacters?.includes(segment)) {
        term_chunks.push(segment);
      } else if (
        !/^\p{Pd}|\p{Pe}|\p{Pf}|\p{Pi}|\p{Po}|\p{Ps}$/u.test(segment)
      ) {
        term_chunks.push(segment.toLocaleLowerCase());
      }

      /**
       * Notes:
       * Regex to match the Rust \w class if we need it:
       * /^\p{Pc}|\p{LC}|\p{Ll}|\p{Lm}|\p{Lo}|\p{Lt}|\p{Lu}|\p{Nd}|\p{Nl}|\p{No}|\s$/u
       * ES2024 regex to match emoji if we need it:
       * /\p{RGI_Emoji}/v
       */
    }

    // TODO: We could use the "word" granularity for Intl.Segmenter to handle
    // segmentation for non-whitespace-delimited languages.

    term = term_chunks
      .join("")
      .replace(/\s{2,}/g, " ")
      .trim();

    log(`Normalized search term to ${term}`);

    if (!term?.length && !filter_only) {
      return {
        results: [],
        unfilteredResultCount: 0,
        filters: {},
        totalFilters: {},
        timings: {
          preload: Date.now() - start,
          search: Date.now() - start,
          total: Date.now() - start,
        },
      };
    }

    let sort_list = this.stringifySorts(options.sort);
    log(`Stringified sort to ${sort_list}`);

    const filter_list = this.stringifyFilters(options.filters);
    log(`Stringified filters to ${filter_list}`);

    let index_resp = this.backend.request_indexes(ptr, term) as string;
    let index_array: string[] = JSON.parse(index_resp);
    let filter_resp = this.backend.request_filter_indexes(
      ptr,
      filter_list,
    ) as string;
    let filter_array: string[] = JSON.parse(filter_resp);

    let chunks = index_array
      .filter((v) => v)
      .map((chunk) => this.loadChunk(chunk));
    let filter_chunks = filter_array
      .filter((v) => v)
      .map((chunk) => this.loadFilterChunk(chunk));
    await Promise.all([...chunks, ...filter_chunks]);
    log(`Loaded necessary chunks to run search`);

    if (options.preload) {
      log(`Preload — bailing out of search operation now.`);
      return null;
    }

    // pointer may have updated from the loadChunk calls
    ptr = await this.getPtr();
    let searchStart = Date.now();
    let result = this.backend.search(
      ptr,
      term,
      filter_list,
      sort_list,
      exact_search,
    ) as string;
    log(`Got the raw search result: ${result}`);

    let {
      filtered_counts,
      total_counts,
      results,
      unfiltered_total,
      search_keywords,
    }: internal.PagefindSearchResponse = JSON.parse(result);

    let resultsInterface = results.map((result) => {
      let weighted_locations = result.l.map((l) => {
        let loc: PagefindWordLocation = {
          weight: l.w / 24.0,
          balanced_score: l.s,
          location: l.l,
        };

        if (l.v) {
          loc.verbose = {
            word_string: l.v.ws,
            length_bonus: l.v.lb,
          };
        }

        return loc;
      });
      let locations = weighted_locations.map((l) => l.location);

      let res: PagefindSearchResult = {
        id: result.p,
        score: result.s * this.indexWeight,
        words: locations,
        data: async () =>
          await this.loadFragment(result.p, weighted_locations, term),
      };

      if (result.params) {
        res.params = {
          document_length: result.params.dl,
          average_page_length: result.params.apl,
          total_pages: result.params.tp,
        };
      }

      if (result.scores) {
        res.scores = result.scores.map((r) => {
          return {
            search_term: r.w,
            idf: r.idf,
            saturating_tf: r.b_tf,
            raw_tf: r.r_tf,
            pagefind_tf: r.p_tf,
            score: r.s,
            params: {
              weighted_term_frequency: r.params.w_tf,
              pages_containing_term: r.params.pct,
              length_bonus: r.params.lb,
            },
          };
        });
      }

      return res;
    });

    const searchTime = Date.now() - searchStart;
    const realTime = Date.now() - start;

    log(
      `Found ${results.length} result${results.length == 1 ? "" : "s"} for "${term}" in ${Date.now() - searchStart}ms (${Date.now() - start}ms realtime)`,
    );
    let response: PagefindSearchResults = {
      results: resultsInterface,
      unfilteredResultCount: unfiltered_total,
      filters: filtered_counts,
      totalFilters: total_counts,
      timings: {
        preload: realTime - searchTime,
        search: searchTime,
        total: realTime,
      },
    };

    if (search_keywords) {
      response.search_keywords = search_keywords;
    }

    return response;
  }
}

export class Pagefind {
  backend: any;
  primaryLanguage: string;
  searchID: number;
  primary: PagefindInstance;
  instances: PagefindInstance[];

  constructor(options: PagefindIndexOptions = {}) {
    this.backend = wasm_bindgen;
    this.primaryLanguage = "unknown";
    this.searchID = 0;

    this.primary = new PagefindInstance({
      ...options,
      primary: true,
    });
    this.instances = [this.primary];

    this.init(options?.language);
  }

  async options(options: PagefindIndexOptions) {
    // Using .options() only affects the primary Pagefind instance.
    await this.primary.options(options);
  }

  async enterPlaygroundMode() {
    // Using .enter_playground_mode() only affects the primary Pagefind instance.
    await this.primary.enterPlaygroundMode();
  }

  async init(overrideLanguage?: string) {
    if (document?.querySelector) {
      const langCode =
        document.querySelector("html")?.getAttribute("lang") || "unknown";
      this.primaryLanguage = langCode.toLocaleLowerCase();
    }

    await this.primary.init(
      overrideLanguage ? overrideLanguage : this.primaryLanguage,
      {
        load_wasm: true,
      },
    );
  }

  async mergeIndex(indexPath: string, options: PagefindIndexOptions = {}) {
    if (this.primary.basePath.startsWith(indexPath)) {
      console.warn(
        `Skipping mergeIndex ${indexPath} that appears to be the same as the primary index (${this.primary.basePath})`,
      );
      return;
    }
    let newInstance = new PagefindInstance({
      primary: false,
      basePath: indexPath,
    });
    this.instances.push(newInstance);

    // Secondary instances rely on the primary instance having
    // loaded the webassembly, so we must wait for that to succeed.
    while (this.primary.wasm === null) {
      await asyncSleep(50);
    }

    await newInstance.init(options.language || this.primaryLanguage, {
      load_wasm: false,
    });
    delete options["language"];
    await newInstance.options(options);
  }

  mergeFilters(filters: PagefindFilterCounts[]) {
    const merged: PagefindFilterCounts = {};
    for (const searchFilter of filters) {
      for (const [filterKey, values] of Object.entries(searchFilter)) {
        if (!merged[filterKey]) {
          merged[filterKey] = values;
          continue;
        } else {
          const filter = merged[filterKey];
          for (const [valueKey, count] of Object.entries(values)) {
            filter[valueKey] = (filter[valueKey] || 0) + count;
          }
        }
      }
    }
    return merged;
  }

  async filters() {
    let filters = await Promise.all(this.instances.map((i) => i.filters()));
    return this.mergeFilters(filters);
  }

  async preload(term: string, options = {}) {
    await Promise.all(this.instances.map((i) => i.preload(term, options)));
  }

  async debouncedSearch(
    term: string,
    options?: PagefindSearchOptions,
    debounceTimeoutMs?: number,
  ): Promise<PagefindIndexesSearchResults | null> {
    const thisSearchID = ++this.searchID;
    this.preload(term, options);
    await asyncSleep(debounceTimeoutMs);

    if (thisSearchID !== this.searchID) {
      return null;
    }

    const searchResult = await this.search(term, options);
    if (thisSearchID !== this.searchID) {
      return null;
    }
    return searchResult;
  }

  async search(
    term: string,
    options: PagefindSearchOptions = {},
  ): Promise<PagefindIndexesSearchResults> {
    let search = await Promise.all(
      this.instances.map(
        (i) => i.search(term, options) as Promise<PagefindSearchResults>,
      ),
    );

    const filters = this.mergeFilters(search.map((s) => s.filters));
    const totalFilters = this.mergeFilters(search.map((s) => s.totalFilters));
    const results = search
      .map((s) => s.results)
      .flat()
      .sort((a, b) => b.score - a.score);
    const timings = search.map((s) => s.timings);
    const unfilteredResultCount = search.reduce(
      (sum, s) => sum + s.unfilteredResultCount,
      0,
    );

    let response: PagefindIndexesSearchResults = {
      results,
      unfilteredResultCount,
      filters,
      totalFilters,
      timings,
    };

    if (search[0].search_keywords) {
      response.search_keywords = search[0].search_keywords;
    }

    return response;
  }
}
