
class Pagefind {
    constructor() {
        this.backend = wasm_bindgen;
        this.languages = null;
        this.primaryLanguage = "unknown";
        this.wasm = null;
        this.searchIndex = null;
        this.searchMeta = null;
        this.raw_ptr = null;
        this.loaded_chunks = {};
        this.loaded_filters = {};
        this.loaded_fragments = {};
        this.basePath = "/_pagefind/";
        this.baseUrl = "/";
        this.decoder = new TextDecoder('utf-8');
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
        await this.loadEntry();
        let index = this.findIndex();
        let lang_wasm = index.wasm ? index.wasm : "unknown";

        await Promise.all([this.loadWasm(lang_wasm), this.loadMeta(index.hash)]);
        window.tmp_pagefind = this.backend;
        this.raw_ptr = this.backend.init_pagefind(new Uint8Array(this.searchMeta));
    }

    findIndex() {
        if (document?.querySelector) {
            const langCode = document.querySelector("html").getAttribute("lang") || "unknown";
            this.primaryLanguage = langCode.toLocaleLowerCase();
        }

        if (this.languages) {
            let index = this.languages[this.primaryLanguage];
            if (index) return index;

            index = this.languages[this.primaryLanguage.split("-")[0]];
            if (index) return index;

            index = this.languages["unknown"];
            if (index) return index;

            let topLang = Object.values(this.languages).sort((a, b) => b.page_count - a.page_count);
            if (topLang[0]) return topLang[0]
        }

        throw new Error("Pagefind Error: No language indexes found.");
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

    async loadEntry() {
        try {
            // We always load a fresh copy of the entry metadata,
            // as it ensures we don't try to load an old build's chunks,
            let entry_json = await fetch(`${this.basePath}pagefind-entry.json?ts=${Date.now()}`);
            entry_json = await entry_json.json();
            this.languages = entry_json.languages;
            if (entry_json.version !== pagefind_version) {
                console.warn([
                    "Pagefind JS version doesn't match the version in your search index.",
                    `Pagefind JS: ${pagefind_version}. Pagefind index: ${entry_json.version}`,
                    "If you upgraded Pagefind recently, you likely have a cached pagefind.js file.",
                    "If you encounter any search errors, try clearing your cache."
                ].join('\n'));
            }
        } catch (e) {
            console.error(`Failed to load Pagefind metadata:\n${e.toString()}`);
        }
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

    // TODO: Due for a rework (chunking + compression)
    // TODO: Large test "fishing" has the wrong mark
    // TODO: Large test "hades" returns some strange results
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
        let exact_search = /^\s*".+"\s*$/.test(term);
        // Strip special characters to match the indexing operation
        // TODO: Maybe move regex over the wasm boundary, or otherwise work to match the Rust regex engine
        term = term.toLowerCase().trim().replace(/[\.`~!@#\$%\^&\*\(\)\{\}\[\]\\\|:;'",<>\/\?]/g, "").replace(/\s{2,}/g, " ").trim();

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
            results: resultsInterface,
            filters: filterObj,
        };
    }
}

const pagefind = new Pagefind();

export const options = (options) => pagefind.options(options);
// TODO: Add a language function that can change the language before pagefind is initialised
export const search = async (term, options) => await pagefind.search(term, options);
export const filters = async () => await pagefind.filters();
