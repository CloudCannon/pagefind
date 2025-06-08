import child_process from 'child_process';
import path from 'path';
import { fileURLToPath } from 'url';
import { resolveBinaryPath } from './resolveBinary.js';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

/**
 * Service class that manages communication with the pagefind-search binary
 */
export class SearchService {
    constructor(config) {
        this.config = config;
        this.bundlePath = path.resolve(config.bundlePath);
        this.language = config.language;
        this.verbose = config.verbose || false;
        
        // Additional configuration options
        this.configFile = config.configFile;
        this.defaultLimit = config.defaultLimit;
        this.outputFormat = config.outputFormat || 'json';
        this.preloadChunks = config.preloadChunks;
        this.generateExcerpts = config.generateExcerpts;
        this.excerptLength = config.excerptLength;
        this.excerptContext = config.excerptContext;
        this.loadFragments = config.loadFragments;
        this.rankingWeights = config.rankingWeights;
    }

    /**
     * Initialize the search service
     */
    async init() {
        // Verify bundle path exists
        try {
            const { existsSync } = await import('fs');
            if (!existsSync(this.bundlePath)) {
                return { error: `Bundle path does not exist: ${this.bundlePath}` };
            }
            
            // Check for pagefind-entry.json
            const entryPath = path.join(this.bundlePath, 'pagefind-entry.json');
            if (!existsSync(entryPath)) {
                return { error: `No pagefind-entry.json found in bundle path: ${this.bundlePath}` };
            }
            
            return { success: true };
        } catch (error) {
            return { error: error.message };
        }
    }

    /**
     * Execute a command with the pagefind-search binary
     */
    async executeCommand(args) {
        return new Promise((resolve) => {
            // Find the binary path
            const binaryPath = resolveBinaryPath();
            
            const fullArgs = [
                ...args,
                '--bundle', this.bundlePath,
                '--output', 'json'
            ];
            
            // Add configuration file if specified
            if (this.configFile) {
                fullArgs.push('--config', this.configFile);
            }
            
            if (this.language) {
                fullArgs.push('--language', this.language);
            }
            
            if (this.verbose) {
                fullArgs.push('--verbose');
            }
            
            // Add ranking weights if specified
            if (this.rankingWeights) {
                if (this.rankingWeights.termSimilarity !== undefined) {
                    fullArgs.push('--ranking-term-similarity', this.rankingWeights.termSimilarity.toString());
                }
                if (this.rankingWeights.pageLength !== undefined) {
                    fullArgs.push('--ranking-page-length', this.rankingWeights.pageLength.toString());
                }
                if (this.rankingWeights.termSaturation !== undefined) {
                    fullArgs.push('--ranking-term-saturation', this.rankingWeights.termSaturation.toString());
                }
                if (this.rankingWeights.termFrequency !== undefined) {
                    fullArgs.push('--ranking-term-frequency', this.rankingWeights.termFrequency.toString());
                }
            }

            const proc = child_process.spawn(binaryPath, fullArgs, {
                windowsHide: true,
                stdio: ['pipe', 'pipe', 'pipe'],
            });

            let stdout = '';
            let stderr = '';

            proc.stdout.on('data', (data) => {
                stdout += data.toString();
            });

            proc.stderr.on('data', (data) => {
                stderr += data.toString();
            });

            proc.on('error', (error) => {
                resolve({ error: `Failed to execute command: ${error.message}` });
            });

            proc.on('close', (code) => {
                if (code !== 0) {
                    resolve({ error: stderr || `Process exited with code ${code}` });
                    return;
                }

                try {
                    const result = JSON.parse(stdout);
                    resolve(result);
                } catch (error) {
                    resolve({ error: `Failed to parse output: ${error.message}` });
                }
            });
        });
    }

    /**
     * Perform a search
     */
    async search(query, options = {}) {
        const args = ['search', '--query', query];
        
        if (options.filters && Object.keys(options.filters).length > 0) {
            args.push('--filters', JSON.stringify(options.filters));
        }
        
        if (options.sort) {
            const sortJson = JSON.stringify([options.sort.by, options.sort.direction]);
            args.push('--sort', sortJson);
        }
        
        // Use provided limit or fall back to default limit from config
        const limit = options.limit || this.defaultLimit;
        if (limit) {
            args.push('--limit', limit.toString());
        }

        return await this.executeCommand(args);
    }

    /**
     * Preload chunks for a query
     */
    async preload(query) {
        // The CLI doesn't have a preload command yet, but we can simulate it
        // by doing a search with limit 0
        const args = ['search', '--query', query, '--limit', '0'];
        const result = await this.executeCommand(args);
        
        if (result.error) {
            return result;
        }
        
        return { success: true };
    }

    /**
     * Get all available filters
     */
    async getFilters() {
        const args = ['filters'];
        return await this.executeCommand(args);
    }

    /**
     * Load a fragment
     */
    async loadFragment(pageHash) {
        // The native search binary doesn't expose fragment loading directly via CLI
        // For now, we'll read the fragment file directly
        try {
            const { readFileSync } = await import('fs');
            const fragmentPath = path.join(this.bundlePath, 'fragment', `${pageHash}.pf_fragment`);
            
            // Try to read the file
            let content;
            try {
                content = readFileSync(fragmentPath);
            } catch (error) {
                // Try with .gz extension if the direct read fails
                const gzPath = fragmentPath + '.gz';
                const gzContent = readFileSync(gzPath);
                
                // Decompress if needed
                const { gunzipSync } = await import('zlib');
                content = gunzipSync(gzContent);
            }
            
            // Check for pagefind_dcd magic bytes and decompress if needed
            const MAGIC_BYTES = Buffer.from('pagefind_dcd');
            if (content.subarray(0, MAGIC_BYTES.length).equals(MAGIC_BYTES)) {
                const { gunzipSync } = await import('zlib');
                content = gunzipSync(content.subarray(MAGIC_BYTES.length));
            }
            
            const fragment = JSON.parse(content.toString());
            return { fragment };
        } catch (error) {
            return { error: `Failed to load fragment: ${error.message}` };
        }
    }

    /**
     * Destroy the service
     */
    async destroy() {
        // Nothing to clean up for subprocess-based approach
    }
}