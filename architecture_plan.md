# Architectural Plan: Native Search Functionality for Pagefind

## 1. Introduction

The goal is to enable Pagefind to perform searches directly against a locally stored Pagefind index, without relying on a browser environment or web servers. This involves creating a new Rust crate for core search logic, implementing native file loading, and designing APIs for both Node.js and CLI usage.

## 2. Current Architecture Overview

*   **`pagefind_web` crate:** Contains the current search logic, compiled to WebAssembly. It handles loading index chunks, metadata, and filter data, performing searches, and ranking results. All file operations are designed to be driven by JavaScript calls from a browser environment (e.g., fetching data over HTTP). Key components include the `SearchIndex` struct and functions like [`init_pagefind`](pagefind_web/src/lib.rs:102), [`load_index_chunk`](pagefind_web/src/lib.rs:183), and [`search`](pagefind_web/src/lib.rs:357).
*   **`pagefind_web_js/lib/coupled_search.ts`:** This TypeScript module acts as the JavaScript interface to the WASM module. It manages fetching all necessary files (`pagefind-entry.json`, metadata, WASM, index chunks, filter chunks, fragments) via HTTP (`fetch` calls) and handles their decompression (from a custom `pagefind_dcd` format).
*   **`pagefind/src/options.rs`:** Defines the configuration structure for the Pagefind CLI, using `clap` for argument parsing and `twelf` for loading from configuration files. It produces a `SearchOptions` struct that drives the indexing process.
*   **`wrappers/node/lib/index.js`:** Provides a Node.js API for the *indexing* part of Pagefind, communicating with a Pagefind service (likely a child process). It does not currently offer a direct search API.

## 3. Proposed Architecture

We will introduce a new crate structure to separate core search logic from web-specific concerns and then build native capabilities on top of that.

### 3.1. Crate Structure

*   **`pagefind_core_search` (New Crate):**
    *   **Purpose:** This crate will house the platform-agnostic search logic. It will be responsible for parsing index files, applying filters, performing search algorithms, and ranking results.
    *   **Extraction:** Logic will be extracted from the current [`pagefind_web`](pagefind_web/src/lib.rs) crate. This includes:
        *   Structs like `SearchIndex`, `PageWord`, `IndexChunk`, `Page`, `RankingWeights`.
        *   Core search functions (modified to remove WASM-specifics and direct JS interop).
        *   Decoding logic for index chunks, metadata, and filters (excluding the `fetch` and initial decompression part, which will be handled by the calling crate).
    *   **API:** It will expose a Rust API that takes byte slices (or readers) for index files and returns search results as Rust structs/enums.
    *   **Dependencies:** `pagefind_microjson`, `pagefind_stem`.

*   **`pagefind_web` (Modified Crate):**
    *   **Purpose:** Will remain the WebAssembly interface for browser-based search.
    *   **Changes:** It will become a lighter wrapper around `pagefind_core_search`.
        *   It will depend on `pagefind_core_search`.
        *   Its `#[wasm_bindgen]` functions will primarily delegate to `pagefind_core_search`, handling WASM type conversions and managing the JavaScript-driven data loading (passing fetched byte arrays to `pagefind_core_search`).
    *   **Dependencies:** `wasm-bindgen`, `pagefind_core_search`.

*   **Path A vs. Path B for Native Implementation:**
    *   **Path A (Native Target in `pagefind_core_search`):** Add native-specific file loading logic directly into `pagefind_core_search` using conditional compilation (e.g., `#[cfg(not(target_arch = "wasm32"))]`).
        *   *Pros:* Simpler crate structure initially.
        *   *Cons:* `pagefind_core_search` would have mixed concerns (core logic + native IO). Might become less clean as native features grow.
    *   **Path B (Separate `pagefind_native_search` Crate):** Create a new crate, say `pagefind_native_search`.
        *   *Pros:* Clear separation of concerns. `pagefind_core_search` remains purely about search algorithms and data structures. `pagefind_native_search` handles all native OS interactions (file system, potentially networking if ever needed for native contexts). Easier to manage features specific to native environments.
        *   *Cons:* One additional crate to manage.
    *   **Recommendation: Path B (`pagefind_native_search` crate).** The clarity and maintainability benefits of separating native IO and environment interactions from the core search algorithms outweigh the cost of an additional crate.

*   **`pagefind_native_search` (New Crate - if Path B is chosen):**
    *   **Purpose:** Provides native search capabilities. It will handle loading the Pagefind index from the local filesystem and exposing a Rust API for searching.
    *   **Functionality:**
        *   Implement native file loading (using `std::fs::File`, `std::io::Read`) to replace the `fetch`-based loading in [`coupled_search.ts`](pagefind_web_js/lib/coupled_search.ts:141).
        *   Implement decompression of `pagefind_dcd` formatted files after reading them from disk. This logic can be shared or adapted from the current JS implementation or implemented in Rust.
        *   Provide functions to initialize a search context from a bundle path and perform searches.
    *   **Dependencies:** `pagefind_core_search`, `std`, potentially a decompression library if not implementing `gunzip` logic directly.

*   **`pagefind` (CLI Crate - Modified):**
    *   **Purpose:** The main command-line tool.
    *   **Changes:**
        *   Will depend on `pagefind_native_search`.
        *   A new CLI command/subcommand will be added to perform local searches (see CLI Interface section).
    *   **Dependencies:** `clap`, `twelf`, `pagefind_native_search`, `pagefind_processing` (current name for indexing logic, assuming it exists or will be refactored).

### 3.1.1. Mermaid Diagram: Crate Dependencies (Path B)

```mermaid
graph TD
    A[pagefind (CLI)] --> D{pagefind_native_search};
    B[pagefind_web (WASM)] --> C{pagefind_core_search};
    D --> C;
    C --> E[pagefind_microjson];
    C --> F[pagefind_stem];
    G[pagefind_node_api (NAPI)] --> D;
```

### 3.2. Native File Loading

*   The `pagefind_native_search` crate will be responsible for all file system interactions.
*   It will need to locate and read:
    *   `pagefind-entry.json`: To get metadata about languages, index hashes, and WASM file (though WASM won't be loaded for native search).
    *   `pagefind.<index_hash>.pf_meta`: The main metadata file.
    *   `index/<chunk_hash>.pf_index`: Index chunk files.
    *   `filter/<filter_hash>.pf_filter`: Filter chunk files.
    *   `fragment/<fragment_hash>.pf_fragment`: Fragment files (for retrieving full page content/details).
*   **Decompression:** The `pagefind_dcd` signature and `gunzip` logic currently in [`coupled_search.ts`](pagefind_web_js/lib/coupled_search.ts:141) (`decompress` function) will need to be replicated in Rust within `pagefind_native_search`. This could involve using a Rust crate like `flate2` for gzip decompression.
*   **Path Management:** The native search will take a `bundle_path` (directory where Pagefind output is stored) as input. All subsequent file paths will be resolved relative to this `bundle_path`.

### 3.3. Configuration Reuse

*   The primary input for native search will be the path to the Pagefind bundle directory (equivalent to `output_path` or `output_subdir` from [`pagefind/src/options.rs`](pagefind/src/options.rs)).
*   Search-time configurations relevant to native search:
    *   `force_language`: Can be passed as an option to the native search function/CLI.
    *   `include_characters`: This is primarily an indexing-time concern but might influence how search terms are processed if not already baked into the index. The `pagefind-entry.json` includes `include_characters`, so the native search can read this.
    *   `ranking_weights`: These are defined in [`pagefind_web/src/lib.rs`](pagefind_web/src/lib.rs:50) and loaded via `set_ranking_weights`. The native search API should allow passing these weights, similar to how the WASM interface does. The `pagefind-entry.json` or a separate config file within the bundle could store default ranking weights set during indexing.
*   Logging: The `Logger` from [`pagefind/src/logging.rs`](pagefind/src/logging.rs) can be reused or adapted for `pagefind_native_search` and the CLI part.

### 3.4. API Design

#### 3.4.1. Node.js API

*   **NPM Package:**
    *   A new package, e.g., `@pagefind/search` or `@pagefind/local-search`. Given the existing `@pagefind/cli` (assuming it's the name for the indexing CLI wrapper), `@pagefind/search` seems appropriate.
    *   This package will contain pre-compiled native binaries for various platforms (using NAPI-rs or a similar tool).
*   **JavaScript API:**
    ```typescript
    // In @pagefind/search
    export interface NativeSearchOptions {
        basePath: string; // Path to the Pagefind bundle directory
        language?: string; // Optional: force a specific language
        ranking?: PagefindRankingWeights; // From pagefind_web_js/types/index.d.ts
        // ... other relevant options like excerptLength
    }

    export interface NativeSearchResult {
        // Define based on existing PagefindSearchResult, adapted for native context
        id: string; // Page hash
        score: number;
        words: string[]; // Words that matched
        meta: Record<string, string>;
        excerpt: string;
        // Potentially raw content or path to fragment for more details
    }

    export interface NativeSearchResults {
        results: NativeSearchResult[];
        totalResults: number;
        filters: Record<string, Record<string, number>>; // Filter counts
        // ... other metadata like timeTaken
    }

    export class PagefindNativeSearch {
        constructor(options: NativeSearchOptions);
        search(term: string, searchOptions?: {
            filters?: Record<string, string[]>;
            sort?: Record<string, 'asc' | 'desc'>;
            exact?: boolean;
        }): Promise<NativeSearchResults>;
        preload(term: string, searchOptions?: any): Promise<void>; // Similar to web
        filters(): Promise<Record<string, Record<string, number>>>; // Similar to web
        destroy(): Promise<void>; // To free up resources if necessary
    }
    ```
    *   The implementation would involve `pagefind_native_search` exposing NAPI-compatible functions.
    *   The `PagefindNativeSearch` class would manage an instance of the Rust search engine.

#### 3.4.2. CLI Interface

*   Extend the main `pagefind` CLI tool.
*   **New Command:** `pagefind search`
    ```bash
    pagefind search --bundle <path_to_bundle_dir> --query "<search_term>" [options]
    ```
*   **Options:**
    *   `--bundle <path>`: (Required) Path to the Pagefind output directory.
    *   `--query <string>`: (Required) The search term.
    *   `--language <lang_code>`: Force a specific language.
    *   `--filters '{"category": ["tech"], "author": ["name"]}'`: JSON string for filters.
    *   `--sort '{"title": "asc"}'`: JSON string for sorting.
    *   `--ranking-term-similarity <float>`: Override ranking weights.
    *   `--ranking-page-length <float>`
    *   `--ranking-term-saturation <float>`
    *   `--ranking-term-frequency <float>`
    *   `--output-json`: Output results as JSON (default could be human-readable).
    *   `--verbose`, `--quiet`, `--silent`, `--logfile`: Reuse existing logging flags.
*   The CLI command would use the Rust API provided by `pagefind_native_search`.

### 3.5. Module Organization and Dependencies (within `pagefind_native_search`)

```
pagefind_native_search/
├── Cargo.toml
└── src/
    ├── lib.rs         // Main entry point, NAPI exports if used for Node.js
    ├── error.rs       // Custom error types
    ├── config.rs      // Structs for search configuration (bundle_path, ranking weights)
    ├── file_loader.rs // Native file loading and decompression logic
    └── searcher.rs    // Manages search context, interacts with pagefind_core_search
```

## 4. Migration Strategy

1.  **Create `pagefind_core_search`:**
    *   Initialize the new crate.
    *   Identify core, platform-agnostic search logic in [`pagefind_web/src/lib.rs`](pagefind_web/src/lib.rs). This includes data structures (`SearchIndex`, `Page`, `PageWord`, `RankingWeights`, etc.) and algorithms (term matching, scoring, filtering logic).
    *   Move this logic to `pagefind_core_search`. Refactor to remove direct `wasm_bindgen` dependencies and JS interop. The API should accept data (e.g., byte slices for chunks) and return Rust types.
    *   Update `pagefind_web` to depend on `pagefind_core_search` and act as a WASM wrapper. This ensures existing web functionality remains intact.
2.  **Create `pagefind_native_search`:**
    *   Initialize the new crate.
    *   Depend on `pagefind_core_search`.
    *   Implement `file_loader.rs` to read Pagefind bundle files from the filesystem and decompress them.
    *   Implement `searcher.rs` to orchestrate loading data via `file_loader.rs` and passing it to `pagefind_core_search` for actual searching.
    *   Define a public Rust API for this crate (e.g., `fn search(bundle_path: &Path, query: &str, options: NativeSearchOptions) -> Result<NativeSearchResults>`).
3.  **Update `pagefind` (CLI Crate):**
    *   Add `pagefind_native_search` as a dependency.
    *   Implement the new `pagefind search` CLI command, using the API from `pagefind_native_search`.
    *   Integrate configuration options.
4.  **Create Node.js Package (`@pagefind/search`):**
    *   Set up a new NPM package.
    *   Use NAPI-rs (or similar) to create native Node.js bindings for the Rust API exposed by `pagefind_native_search`.
    *   Implement the JavaScript API (`PagefindNativeSearch` class) that calls these native bindings.
    *   Set up build scripts to compile the Rust code into native modules for different platforms.
5.  **Testing:** Thoroughly test all new components: `pagefind_core_search` (unit tests), `pagefind_native_search` (integration tests with sample bundles), CLI (end-to-end tests), and Node.js API.

## 5. High-Level Component Interaction (Native Search via Node.js)

```mermaid
sequenceDiagram
    participant UserApp as User Application (Node.js)
    participant NodeAPI as @pagefind/search (JS)
    participant NAPIBridge as NAPI-rs Bindings
    participant NativeSearch as pagefind_native_search (Rust)
    participant CoreSearch as pagefind_core_search (Rust)
    participant FileSystem as Local Filesystem

    UserApp->>NodeAPI: new PagefindNativeSearch({bundlePath: "/path"})
    NodeAPI->>NAPIBridge: init_search_context("/path")
    NAPIBridge->>NativeSearch: init_search_context_rust("/path")
    NativeSearch->>FileSystem: Read pagefind-entry.json
    FileSystem-->>NativeSearch: entry_json_data
    NativeSearch->>FileSystem: Read pagefind.pf_meta
    FileSystem-->>NativeSearch: meta_data
    NativeSearch-->>NAPIBridge: context_ptr
    NAPIBridge-->>NodeAPI: nativeInstance
    NodeAPI-->>UserApp: searchInstance

    UserApp->>NodeAPI: searchInstance.search("query", {filters: ...})
    NodeAPI->>NAPIBridge: search(context_ptr, "query", filters_json)
    NAPIBridge->>NativeSearch: search_rust(context_ptr, "query", filters)
    NativeSearch->>CoreSearch: determine_required_chunks("query", filters)
    CoreSearch-->>NativeSearch: chunk_hashes_needed
    loop For each chunk_hash
        NativeSearch->>FileSystem: Read index/chunk_hash.pf_index
        FileSystem-->>NativeSearch: chunk_data (compressed)
        NativeSearch->>NativeSearch: Decompress chunk_data
        NativeSearch->>CoreSearch: load_chunk(decompressed_chunk_data)
    end
    NativeSearch->>CoreSearch: perform_search("query", filters)
    CoreSearch-->>NativeSearch: search_results_rust
    NativeSearch-->>NAPIBridge: search_results_json
    NAPIBridge-->>NodeAPI: results_object
    NodeAPI-->>UserApp: Promise<NativeSearchResults>