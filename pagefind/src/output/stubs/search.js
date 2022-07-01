
class Pagefind {
    constructor() {
        this.backend = wasm_bindgen;
        this.wasm = null;
        this.searchIndex = null;
        this.searchMeta = null;
        this.raw_ptr = null;
        this.loaded_chunks = {};
        this.loaded_filters = {};
        this.loaded_fragments = {};
        this.basePath = "/_pagefind/";
        this.baseUrl = "/";
        this.init();
    }

    options(options) {
        const opts = ["basePath", "baseUrl"];
        for (const [k, v] of Object.entries(options)) {
            if (opts.includes(k)) {
                this[k] = v;
            } else {
                console.warn(`Unknown Pagefind option ${k}. Allowed options: [${opts.join(', ')}]`);
            }
        }
    }

    async init() {
        try {
            this.basePath = new URL(import.meta.url).pathname.match(/^(.*\/)pagefind.js.*$/)[1];
            let default_base = this.basePath.match(/^(.*\/)_pagefind/)?.[1];
            this.baseUrl = default_base || this.baseUrl;
        } catch (e) {
            console.warn("Pagefind couldn't determine the base of the bundle from the import path. Falling back to the default.");
        }
        await Promise.all([this.loadWasm(), this.loadMeta()]);
        window.tmp_pagefind = this.backend;
        this.raw_ptr = this.backend.init_pagefind(new Uint8Array(this.searchMeta));
    }

    async loadMeta() {
        try {
            // We always load a fresh copy of the metadata,
            // as it ensures we don't try to load an old build's chunks,
            // and it's (hopefully) a small enough file to not be a worry.
            // TODO:     ^^^^^^^^^
            let compressed_meta = await fetch(`${this.basePath}pagefind.pf_meta?ts=${Date.now()}`);
            compressed_meta = await compressed_meta.arrayBuffer();
            this.searchMeta = gunzip(new Uint8Array(compressed_meta));
        } catch (e) {
            console.error(`Failed to load the meta index:\n${e.toString()}`);
        }
    }

    async loadWasm() {
        try {
            let compressed_wasm = await fetch(`${this.basePath}wasm.pagefind`);
            compressed_wasm = await compressed_wasm.arrayBuffer();
            this.wasm = await this.backend(gunzip(new Uint8Array(compressed_wasm)));
        } catch (e) {
            console.error(`Failed to load the Pagefind WASM ${url}:\n${e.toString()}`);
        }
    }

    async _loadGenericChunk(url, method) {
        try {
            let compressed_chunk = await fetch(url);
            compressed_chunk = await compressed_chunk.arrayBuffer();
            let chunk = gunzip(new Uint8Array(compressed_chunk));

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
        let fragment = gunzip(new Uint8Array(compressed_fragment));
        return JSON.parse(new TextDecoder().decode(fragment));
    }

    // TODO: Due for a rework (chunking + compression)
    // TODO: Large test "fishing" has the wrong mark
    // TODO: Large test "hades" returns some strange results
    async loadFragment(hash, excerpt = [0, 0], locations = []) {
        if (!this.loaded_fragments[hash]) {
            this.loaded_fragments[hash] = this._loadFragment(hash);
        }
        let fragment = await this.loaded_fragments[hash];

        let fragment_words = fragment.content.split(/[\r\n\s]+/g);
        for (let word of locations) {
            fragment_words[word] = `<mark>${fragment_words[word]}</mark>`;
        }
        fragment.excerpt = fragment_words.slice(excerpt[0], excerpt[0] + excerpt[1]).join(' ');
        if (!fragment.raw_url) {
            fragment.raw_url = fragment.url;
            fragment.url = this.fullUrl(fragment.raw_url);
        }
        return fragment;
    }

    fullUrl(raw) {
        return `/${this.baseUrl}/${raw}`.replace(/\/+/g, "/");
    }

    async sleep(ms = 100) {
        return new Promise(r => setTimeout(r, ms));
    }

    async getPtr() {
        while (this.raw_ptr === null) {
            await this.sleep(50);
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
            for (const valueBlock of values.split("__PF_VALUE_DELIM__")) {
                let [, value, count] = valueBlock.match(/^(.*):(\d+)$/);
                output[filter][value] = count;
            }
        }

        return output;
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

    async search(term, options) {
        options = {
            verbose: false,
            filters: {},
            ...options,
        };
        const log = str => { if (options.verbose) console.log(str) };
        let start = Date.now();
        let ptr = await this.getPtr();
        // Strip special characters to match the indexing operation
        let exact_search = /^\s*".+"\s*$/.test(term);
        term = term.toLowerCase().trim().replace(/[^\w\s]/g, "").replace(/\s{2,}/g, " ").trim();

        let filter_list = [];
        for (let [filter, values] of Object.entries(options.filters)) {
            if (Array.isArray(values)) {
                for (let value of values) {
                    filter_list.push(`${filter}:${value}`);
                }
            } else {
                filter_list.push(`${filter}:${values}`);
            }
        }

        filter_list = filter_list.join("__PF_FILTER_DELIM__");

        let chunks = this.backend.request_indexes(ptr, term).split(' ').filter(v => v).map(chunk => this.loadChunk(chunk));
        let filter_chunks = this.backend.request_filter_indexes(ptr, filter_list).split(' ').filter(v => v).map(chunk => this.loadFilterChunk(chunk));
        await Promise.all([...chunks, ...filter_chunks]);

        // pointer may have updated from the loadChunk calls
        ptr = await this.getPtr();
        let searchStart = Date.now();
        let result = this.backend.search(ptr, term, filter_list, exact_search);
        let [results, filters] = result.split(/:(.*)$/);
        let filterObj = this.parseFilters(filters);
        results = results.length ? results.split(" ") : [];

        let resultsInterface = results.map(result => {
            let [hash, excerpt, locations] = result.split('@');
            locations = locations.split(',').map(l => parseInt(l));
            excerpt = excerpt.split(',').map(l => parseInt(l));
            return {
                id: hash,
                words: locations,
                excerpt_range: excerpt,
                data: async () => await this.loadFragment(hash, excerpt, locations)
            }
        });

        log(`Found ${results.length} result${results.length == 1 ? '' : 's'} for "${term}" in ${Date.now() - searchStart}ms (${Date.now() - start}ms realtime)`);
        return {
            suggestion: "<!-- NYI -->",
            matched: "<!-- NYI -->",
            results: resultsInterface,
            filters: filterObj,
        };
    }
}

const pagefind = new Pagefind();

export const options = (options) => pagefind.options(options);
export const search = async (term, options) => await pagefind.search(term, options);
export const filters = async () => await pagefind.filters();
