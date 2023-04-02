/**
 * Create a new Pagefind index that files can be added to
 */
export function createIndex(): Promise<NewIndexResponse>;

export interface NewIndexResponse {
    errors: string[],
    index?: PagefindIndex
}

/**
 * A Pagefind index that exists in the backend service
 */
export interface PagefindIndex {
    addHTMLFile: typeof addHTMLFile,
    addCustomRecord: typeof addCustomRecord,
    writeFiles: typeof writeFiles,
}

declare function addHTMLFile(filePath: string, fileContents: string): Promise<NewFileResponse>;
declare function addCustomRecord(record: CustomRecord): Promise<NewFileResponse>;

/**
 * The data required to add a custom record to Pagefind that isn't backed by an HTML file
 */
export interface CustomRecord {
    url: string,
    content: string,
    language: string,
    meta?: Record<string, string>,
    filters?: Record<string, string[]>,
    sort?: Record<string, string>
}

export interface NewFileResponse {
    errors: string[],
    file: NewFile
}

export interface NewFile {
    uniqueWords: number,
    url: string,
    meta: Record<string, string>
}

declare function writeFiles(): Promise<WriteFilesResponse>;

export interface WriteFilesResponse {
    errors: string[],
    bundleLocation: string
}
