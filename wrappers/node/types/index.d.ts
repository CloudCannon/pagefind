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
    addHTMLFile(filePath: string, fileContents: string): Promise<NewFileResponse>,
    addCustomRecord(url: string, content: string): Promise<NewFileResponse>,
    writeFiles(): Promise<WriteFilesResponse>,
}

/**
 * The data required to add a custom record to Pagefind that isn't backed by an HTML file
 */
export interface CustomRecord {
    url: string,
    meta: Record<string, string>,
    filters: Record<string, string[]>,
    sort: Record<string, string>
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

export interface WriteFilesResponse { }