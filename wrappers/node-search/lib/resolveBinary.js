import { existsSync } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

/**
 * Resolve the path to the pagefind-search binary
 * @returns {string} The path to the binary
 */
export function resolveBinaryPath() {
    // In development, use the system binary
    if (process.env.NODE_ENV === 'development' || process.env.PAGEFIND_SEARCH_BINARY) {
        return process.env.PAGEFIND_SEARCH_BINARY || 'pagefind-search';
    }
    
    // Platform-specific binary names
    const platform = process.platform;
    const arch = process.arch;
    
    let binaryName = 'pagefind-search';
    if (platform === 'win32') {
        binaryName = 'pagefind-search.exe';
    }
    
    // Try to find the binary in common locations
    const possiblePaths = [
        // Adjacent to this file
        path.join(__dirname, '..', 'bin', binaryName),
        // In node_modules/.bin
        path.join(__dirname, '..', '..', '.bin', binaryName),
        // Platform-specific package
        path.join(__dirname, '..', '..', `@pagefind/${platform}-${arch}`, 'bin', binaryName),
    ];
    
    for (const path of possiblePaths) {
        if (existsSync(path)) {
            return path;
        }
    }
    
    // Fall back to hoping it's in PATH
    return binaryName;
}