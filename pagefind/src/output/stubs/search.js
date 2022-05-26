
class Pagefind {
    constructor() {
        this.backend = wasm_bindgen;
        this.wasm = null;
        this.searchIndex = null;
        this.searchMeta = null;
        this.raw_ptr = null;
        this.loaded_chunks = [];
        this.loaded_filters = [];
        this.base_path = "/_pagefind/";
        this.init();
    }

    async init() {
        try {
            this.base_path = new URL(import.meta.url).pathname.match(/^(.*\/)pagefind.js.*$/)[1];
        } catch (e) {
            console.warn("Pagefind couldn't determine the base of the bundle from the import path. Falling back to the default.");
        }
        await Promise.all([this.loadWasm(), this.loadMeta()]);
        window.tmp_pagefind = this.backend;
        this.raw_ptr = this.backend.init_pagefind(new Uint8Array(this.searchMeta));
    }

    async loadMeta() {
        // We always load a fresh copy of the metadata,
        // as it ensures we don't try to load an old build's chunks,
        // and it's a small enough file to not be a worry.
        let compressed_meta = await fetch(`${this.base_path}pagefind.pf_meta?ts=${Date.now()}`);
        compressed_meta = await compressed_meta.arrayBuffer();
        this.searchMeta = gunzip(new Uint8Array(compressed_meta));
    }

    async loadWasm() {
        let compressed_wasm = await fetch(`${this.base_path}wasm.pagefind`);
        compressed_wasm = await compressed_wasm.arrayBuffer();
        this.wasm = await this.backend(gunzip(new Uint8Array(compressed_wasm)));
    }

    async loadChunk(hash) {
        if (this.loaded_chunks.includes(hash)) return;

        let compressed_chunk = await fetch(`${this.base_path}index/${hash}.pf_index`);
        compressed_chunk = await compressed_chunk.arrayBuffer();
        let chunk = gunzip(new Uint8Array(compressed_chunk));

        let ptr = await this.getPtr();
        this.raw_ptr = this.backend.load_index_chunk(ptr, chunk);
        this.loaded_chunks.push(hash);
    }

    async loadFilterChunk(hash) {
        if (this.loaded_filters.includes(hash)) return;

        let compressed_chunk = await fetch(`${this.base_path}filter/${hash}.pf_filter`);
        compressed_chunk = await compressed_chunk.arrayBuffer();
        let chunk = gunzip(new Uint8Array(compressed_chunk));

        let ptr = await this.getPtr();
        this.raw_ptr = this.backend.load_filter_chunk(ptr, chunk);
        this.loaded_filters.push(hash);
    }

    // TODO: Due for a rework (chunking)
    // TODO: Large test "fishing" has the wrong mark
    // TODO: Large test "hades" returns some strange results
    async loadFragment(hash, excerpt = [0, 0], locations = []) {
        let fragment = await fetch(`${this.base_path}fragment/${hash}.pf_fragment`);
        fragment = await fragment.json();
        let fragment_words = fragment.content.split(/[\r\n\s]+/g);
        for (let word of locations) {
            fragment_words[word] = `<mark>${fragment_words[word]}</mark>`;
        }
        fragment.excerpt = fragment_words.slice(excerpt[0], excerpt[0] + excerpt[1]).join(' ');
        return fragment;
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

    async search(term, options) {
        options = {
            verbose: false,
            filters: {},
            ...options,
        };
        const log = str => { if (options.verbose) console.log(str) };
        let start = Date.now();
        let ptr = await this.getPtr();
        term = term.toLowerCase();

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
        let results = this.backend.search(ptr, term, filter_list);
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
        return resultsInterface;
    }
}

const pagefind = new Pagefind();

export const search = async (term, options) => await pagefind.search(term, options);
