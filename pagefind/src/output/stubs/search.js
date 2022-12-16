
const asyncSleep = async (ms = 100) => {
    return new Promise(r => setTimeout(r, ms));
};

class PagefindInstance {
    constructor(opts = {}) {
        this.backend = wasm_bindgen;
        this.decoder = new TextDecoder('utf-8');
        this.wasm = null;

        this.basePath = opts.basePath || "/_pagefind/";
        this.primary = opts.primary || false;
        if (this.primary) {
            this.initPrimary();
        }
        if (/[^\/]$/.test(this.basePath)) {
            this.basePath = `${this.basePath}/`;
        }
        if (window?.location?.origin && this.basePath.startsWith(window.location.origin)) {
            this.basePath = this.basePath.replace(window.location.origin, '');
        }

        this.baseUrl = opts.baseUrl || this.defaultBasePath();
        if (!/^(\/|https?:\/\/)/.test(this.baseUrl)) {
            this.baseUrl = `/${this.baseUrl}`;
        }

        this.indexWeight = opts.indexWeight ?? 1;
        this.mergeFilter = opts.mergeFilter ?? {};

        this.loaded_chunks = {};
        this.loaded_filters = {};
        this.loaded_fragments = {};

        this.raw_ptr = null;
        this.searchMeta = null;
        this.languages = null;
    }

    initPrimary() {
        try {
            this.basePath = import.meta.url.match(/^(.*\/)pagefind.js.*$/)[1];
        } catch (e) {
            console.warn("Pagefind couldn't determine the base of the bundle from the import path. Falling back to the default.");
        }
    }

    defaultBasePath() {
        let default_base = this.basePath.match(/^(.*\/)_pagefind/)?.[1];
        return default_base || "/";
    }

    async options(options) {
        const opts = ["basePath", "baseUrl", "indexWeight", "mergeFilter"];
        for (const [k, v] of Object.entries(options)) {
            if (k === "mergeFilter") {
                let filters = this.stringifyFilters(v);
                let ptr = await this.getPtr();
                this.raw_ptr = this.backend.add_synthetic_filter(ptr, filters);
            } else if (opts.includes(k)) {
                this[k] = v;
            } else {
                console.warn(`Unknown Pagefind option ${k}. Allowed options: [${opts.join(', ')}]`);
            }
        }
    }

    decompress(data, file = "unknown file") {
        if (this.decoder.decode(data.slice(0, 12)) === "pagefind_dcd") {
            // File is already decompressed
            return data.slice(12);
        }
        data = gunzip(data);
        if (this.decoder.decode(data.slice(0, 12)) !== "pagefind_dcd") {
            // Decompressed file does not have the correct signature
            console.error(`Decompressing ${file} appears to have failed: Missing signature`);
            return data;
        }
        return data.slice(12);
    }

    async init(language, opts) {
        await this.loadEntry();
        let index = this.findIndex(language);
        let lang_wasm = index.wasm ? index.wasm : "unknown";

        let resources = [this.loadMeta(index.hash)];
        if (opts.load_wasm === true) {
            resources.push(this.loadWasm(lang_wasm));
        }
        await Promise.all(resources);
        this.raw_ptr = this.backend.init_pagefind(new Uint8Array(this.searchMeta));
    }

    async loadEntry() {
        try {
            // We always load a fresh copy of the entry metadata,
            // as it ensures we don't try to load an old build's chunks,
            let entry_json = await fetch(`${this.basePath}pagefind-entry.json?ts=${Date.now()}`);
            entry_json = await entry_json.json();
            this.languages = entry_json.languages;
            if (entry_json.version !== pagefind_version) {
                if (this.primary) {
                    console.warn([
                        "Pagefind JS version doesn't match the version in your search index.",
                        `Pagefind JS: ${pagefind_version}. Pagefind index: ${entry_json.version}`,
                        "If you upgraded Pagefind recently, you likely have a cached pagefind.js file.",
                        "If you encounter any search errors, try clearing your cache."
                    ].join('\n'));
                } else {
                    console.warn([
                        "Merging a Pagefind index from a different version than the main Pagefind instance.",
                        `Main Pagefind JS: ${pagefind_version}. Merged index (${this.basePath}): ${entry_json.version}`,
                        "If you encounter any search errors, make sure that both sites are running the same version of Pagefind."
                    ].join('\n'));
                }
            }
        } catch (e) {
            console.error(`Failed to load Pagefind metadata:\n${e.toString()}`);
            throw new Error("Failed to load Pagefind metadata");
        }
    }

    findIndex(language) {
        if (this.languages) {
            let index = this.languages[language];
            if (index) return index;

            index = this.languages[language.split("-")[0]];
            if (index) return index;

            let topLang = Object.values(this.languages).sort((a, b) => b.page_count - a.page_count);
            if (topLang[0]) return topLang[0]
        }

        throw new Error("Pagefind Error: No language indexes found.");
    }

    async loadMeta(index) {
        try {
            let compressed_meta = await fetch(`${this.basePath}pagefind.${index}.pf_meta`);
            compressed_meta = await compressed_meta.arrayBuffer();
            this.searchMeta = this.decompress(new Uint8Array(compressed_meta), "Pagefind metadata");
        } catch (e) {
            console.error(`Failed to load the meta index:\n${e.toString()}`);
        }
    }

    async loadWasm(language) {
        try {
            const wasm_url = `${this.basePath}wasm.${language}.pagefind`;
            let compressed_wasm = await fetch(wasm_url);
            compressed_wasm = await compressed_wasm.arrayBuffer();
            const final_wasm = this.decompress(new Uint8Array(compressed_wasm), "Pagefind WebAssembly");
            this.wasm = await this.backend(final_wasm);
        } catch (e) {
            console.error(`Failed to load the Pagefind WASM:\n${e.toString()}`);
        }
    }

    async _loadGenericChunk(url, method) {
        try {
            let compressed_chunk = await fetch(url);
            compressed_chunk = await compressed_chunk.arrayBuffer();
            let chunk = this.decompress(new Uint8Array(compressed_chunk), url);

            let ptr = await this.getPtr();
            this.raw_ptr = this.backend[method](ptr, chunk);
        } catch (e) {
            console.error(`Failed to load the index chunk ${url}:\n${e.toString()}`);
        }
    }

    async loadChunk(hash) {
        if (!this.loaded_chunks[hash]) {
            const url = `${this.basePath}index/${hash}.pf_index`;
            this.loaded_chunks[hash] = this._loadGenericChunk(url, "load_index_chunk");
        }
        return await this.loaded_chunks[hash];
    }

    async loadFilterChunk(hash) {
        if (!this.loaded_filters[hash]) {
            const url = `${this.basePath}filter/${hash}.pf_filter`;
            this.loaded_filters[hash] = this._loadGenericChunk(url, "load_filter_chunk");
        }
        return await this.loaded_filters[hash];
    }

    async _loadFragment(hash) {
        let compressed_fragment = await fetch(`${this.basePath}fragment/${hash}.pf_fragment`);
        compressed_fragment = await compressed_fragment.arrayBuffer();
        let fragment = this.decompress(new Uint8Array(compressed_fragment), `Fragment ${hash}`);
        return JSON.parse(new TextDecoder().decode(fragment));
    }

    async loadFragment(hash, excerpt = [0, 0], locations = []) {
        if (!this.loaded_fragments[hash]) {
            this.loaded_fragments[hash] = this._loadFragment(hash);
        }
        let fragment = await this.loaded_fragments[hash];

        if (!fragment.raw_content) {
            fragment.raw_content = fragment.content;
            fragment.content = fragment.content.replace(/\u200B/g, '');
        }

        let is_zws_delimited = fragment.raw_content.includes('\u200B');
        let fragment_words = [];
        if (is_zws_delimited) {
            // If segmentation was run on the backend, count words by ZWS boundaries
            fragment_words = fragment.raw_content.split('\u200B');
        } else {
            fragment_words = fragment.raw_content.split(/[\r\n\s]+/g);
        }
        for (let word of locations) {
            fragment_words[word] = `<mark>${fragment_words[word]}</mark>`;
        }
        fragment.excerpt = fragment_words.slice(excerpt[0], excerpt[0] + excerpt[1]).join(is_zws_delimited ? '' : ' ').trim();
        if (!fragment.raw_url) {
            fragment.raw_url = fragment.url;
            fragment.url = this.fullUrl(fragment.raw_url);
        }
        return fragment;
    }

    fullUrl(raw) {
        return `${this.baseUrl}/${raw}`.replace(/(?<!https?:)\/+/g, "/");
    }

    async getPtr() {
        while (this.raw_ptr === null) {
            await asyncSleep(50);
        }
        if (!this.raw_ptr) {
            console.error("Pagefind: WASM Error (No pointer)");
            throw new Error("Pagefind: WASM Error (No pointer)")
        }
        return this.raw_ptr;
    }

    parseFilters(str) {
        let output = {};
        if (!str) return output;
        for (const block of str.split("__PF_FILTER_DELIM__")) {
            let [filter, values] = block.split(/:(.*)$/);
            output[filter] = {};
            if (values) {
                for (const valueBlock of values.split("__PF_VALUE_DELIM__")) {
                    if (valueBlock) {
                        let [, value, count] = valueBlock.match(/^(.*):(\d+)$/);
                        output[filter][value] = parseInt(count) ?? count;
                    }
                }
            }
        }

        return output;
    }

    stringifyFilters(obj = {}) {
        let filter_list = [];
        for (let [filter, values] of Object.entries(obj)) {
            if (Array.isArray(values)) {
                for (let value of values) {
                    filter_list.push(`${filter}:${value}`);
                }
            } else {
                filter_list.push(`${filter}:${values}`);
            }
        }

        return filter_list.join("__PF_FILTER_DELIM__");
    }

    stringifySorts(obj = {}) {
        let sorts = Object.entries(obj);
        // We currently only support one sort directive,
        // so we'll grab the first sort provided in the object.
        for (let [sort, direction] of sorts) {
            if (sorts.length > 1) {
                console.warn(`Pagefind was provided multiple sort options in this search, but can only operate on one. Using the ${sort} sort.`);
            }
            if (direction !== "asc" && direction !== "desc") {
                console.warn(`Pagefind was provided a sort with unknown direction ${direction}. Supported: [asc, desc]`);
            }
            return `${sort}:${direction}`;
        }

        return ``;
    }

    async filters() {
        let ptr = await this.getPtr();

        let filter_chunks = this.backend.request_all_filter_indexes(ptr).split(' ').filter(v => v).map(chunk => this.loadFilterChunk(chunk));
        await Promise.all([...filter_chunks]);

        // pointer may have updated from the loadChunk calls
        ptr = await this.getPtr();

        let results = this.backend.filters(ptr);
        return this.parseFilters(results);
    }

    async preload(term, options = {}) {
        options.preload = true;
        await this.search(term, options);
    }

    async search(term, options = {}) {
        options = {
            verbose: false,
            filters: {},
            sort: {},
            ...options,
        };
        const log = str => { if (options.verbose) console.log(str) };
        log(`Starting search on ${this.basePath}`);
        let start = Date.now();
        let ptr = await this.getPtr();
        let filter_only = term === null;
        term = term ?? "";
        let exact_search = /^\s*".+"\s*$/.test(term);
        if (exact_search) {
            log(`Running an exact search`);
        }
        // Strip special characters to match the indexing operation
        // TODO: Maybe move regex over the wasm boundary, or otherwise work to match the Rust regex engine
        term = term.toLowerCase().trim().replace(/[\.`~!@#\$%\^&\*\(\)\{\}\[\]\\\|:;'",<>\/\?\-]/g, "").replace(/\s{2,}/g, " ").trim();
        log(`Normalized search term to ${term}`);

        if (!term?.length && !filter_only) {
            return {
                results: [],
                filters: {},
                timings: {
                    preload: Date.now() - start,
                    search: Date.now() - start,
                    total: Date.now() - start
                }
            };
        }

        let sort_list = this.stringifySorts(options.sort);
        log(`Stringified sort to ${sort_list}`);

        const filter_list = this.stringifyFilters(options.filters);
        log(`Stringified filters to ${filter_list}`);

        let chunks = this.backend.request_indexes(ptr, term).split(' ').filter(v => v).map(chunk => this.loadChunk(chunk));
        let filter_chunks = this.backend.request_filter_indexes(ptr, filter_list).split(' ').filter(v => v).map(chunk => this.loadFilterChunk(chunk));
        await Promise.all([...chunks, ...filter_chunks]);
        log(`Loaded necessary chunks to run search`);

        if (options.preload) {
            log(`Preload — bailing out of search operation now.`);
            return;
        }

        // pointer may have updated from the loadChunk calls
        ptr = await this.getPtr();
        let searchStart = Date.now();
        let result = this.backend.search(ptr, term, filter_list, sort_list, exact_search);
        log(`Got the raw search result: ${result}`);
        let [results, filters] = result.split(/:(.*)$/);
        let filterObj = this.parseFilters(filters);
        log(`Remaining filters: ${JSON.stringify(result)}`);
        results = results.length ? results.split(" ") : [];

        let resultsInterface = results.map(result => {
            let [hash, score, excerpt, locations] = result.split('@');
            log(`Processing result: \n  hash:${hash}\n  score:${score}\n  excerpt:${excerpt}\n  locations:${locations}`);
            locations = locations.split(',').map(l => parseInt(l));
            excerpt = excerpt.split(',').map(l => parseInt(l));
            return {
                id: hash,
                score: parseFloat(score) * this.indexWeight,
                words: locations,
                excerpt_range: excerpt,
                data: async () => await this.loadFragment(hash, excerpt, locations)
            }
        });

        const searchTime = Date.now() - searchStart;
        const realTime = Date.now() - start;

        log(`Found ${results.length} result${results.length == 1 ? '' : 's'} for "${term}" in ${Date.now() - searchStart}ms (${Date.now() - start}ms realtime)`);
        return {
            results: resultsInterface,
            filters: filterObj,
            timings: {
                preload: realTime - searchTime,
                search: searchTime,
                total: realTime
            }
        };
    }
}

class Pagefind {
    constructor() {
        this.backend = wasm_bindgen;
        this.primaryLanguage = "unknown";

        this.primary = new PagefindInstance({
            primary: true
        });
        this.instances = [this.primary];

        this.init();
    }

    async options(options) {
        // Using .options() only affects the primary Pagefind instance.
        await this.primary.options(options);
    }

    async init() {
        if (document?.querySelector) {
            const langCode = document.querySelector("html").getAttribute("lang") || "unknown";
            this.primaryLanguage = langCode.toLocaleLowerCase();
        }

        await this.primary.init(this.primaryLanguage, {
            load_wasm: true
        });
    }

    async mergeIndex(indexPath, options = {}) {
        if (this.primary.basePath.startsWith(indexPath)) {
            console.warn(`Skipping mergeIndex ${indexPath} that appears to be the same as the primary index (${this.primary.basePath})`);
            return;
        }
        let newInstance = new PagefindInstance({
            primary: false,
            basePath: indexPath
        });
        this.instances.push(newInstance);

        // Secondary instances rely on the primary instance having
        // loaded the webassembly, so we must wait for that to succeed.
        while (this.primary.wasm === null) {
            await asyncSleep(50);
        }

        await newInstance.init(options.language || this.primaryLanguage, {
            load_wasm: false
        });
        delete options["language"];
        await newInstance.options(options);
    }

    mergeFilters(filters) {
        const merged = {};
        for (const searchFilter of filters) {
            for (const [filterKey, values] of Object.entries(searchFilter)) {
                if (!merged[filterKey]) {
                    merged[filterKey] = values;
                    continue;
                } else {
                    const filter = merged[filterKey];
                    for (const [valueKey, count] of Object.entries(filter)) {
                        filter[valueKey] = (filter[valueKey] || 0) + count;
                    }
                }
            }
        }
        return merged;
    }

    async filters() {
        let filters = await Promise.all(this.instances.map(i => i.filters()));
        return this.mergeFilters(filters);
    }

    async preload(term, options = {}) {
        await Promise.all(this.instances.map(i => i.preload(term, options)));
    }

    async search(term, options = {}) {
        let search = await Promise.all(this.instances.map(i => i.search(term, options)));

        const filters = this.mergeFilters(search.map(s => s.filters));
        const results = search.map(s => s.results).flat().sort((a, b) => b.score - a.score);
        const timings = search.map(s => s.timings);

        return { results, filters, timings };
    }
}

const pagefind = new Pagefind();

export const mergeIndex = async (indexPath, options) => await pagefind.mergeIndex(indexPath, options);
export const options = async (options) => await pagefind.options(options);
// TODO: Add a language function that can change the language before pagefind is initialised
export const search = async (term, options) => await pagefind.search(term, options);
export const preload = async (term, options) => await pagefind.preload(term, options);
export const filters = async () => await pagefind.filters();
