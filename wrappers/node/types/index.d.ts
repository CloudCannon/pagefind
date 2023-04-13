/**
 * Create a new Pagefind index that files can be added to
 */
export function createIndex(): Promise<NewIndexResponse>;

export interface NewIndexResponse {
    errors: string[],
    index?: PagefindIndex
}

/**
 * A Pagefind index that exists in the backend service before being built
 */
export interface PagefindIndex {
    addHTMLFile: typeof addHTMLFile,
    addCustomRecord: typeof addCustomRecord,
    writeFiles: typeof writeFiles,
    getFiles: typeof getFiles,
}

/**
 * Index an HTML file that isn't on disk
 */
declare function addHTMLFile(file: HTMLFile): Promise<NewFileResponse>;
/**
 * Index a custom record that isn't backed by an HTML file
 */
declare function addCustomRecord(record: CustomRecord): Promise<NewFileResponse>;

/**
 * The data required for Pagefind to index an HTML file that isn't on disk
 * @example
 * {
 *   path: "about/index.html",
 *   content: "<html lang='en'><body><h1>Meet the team</h1></body></html>"
 * }
 */
export interface HTMLFile {
    /** 
     * The relative path to the HTML file if it were to exist on disk.
     * Pagefind will compute the result URL from this path.
     * @example "about/index.html"
     */
    path: string,
    /** The source HTML content of the file to be parsed */
    content: string
}

/**
 * The data required for Pagefind to index a custom record that isn't backed by an HTML file
 * @example
 * {
 *   url: "/about/",
 *   content: "Meet the team",
 *   language: "en"
 * }
 */
export interface CustomRecord {
    /** The output URL of this record. Pagefind will not alter this */
    url: string,
    /** The raw content of this record */
    content: string,
    /** What language is this record written in. Multiple languages will be split into separate indexes */
    language: string,
    /** The metadata to attach to this record. Supplying a `title` is highly recommended */
    meta?: Record<string, string>,
    /** The filters to attach to this record */
    filters?: Record<string, string[]>,
    /** The sort keys to attach to this record */
    sort?: Record<string, string>
}

export interface NewFileResponse {
    errors: string[],
    file: NewFile
}

/**
 * Details about a new file that has been successfully added to the Pagefind index
 */
export interface NewFile {
    uniqueWords: number,
    url: string,
    meta: Record<string, string>
}

/**
 * Write the index files to the cwd
 */
declare function writeFiles(): Promise<WriteFilesResponse>;

export interface WriteFilesResponse {
    errors: string[],
    bundleLocation: string
}

/**
 * Get an in-memory copy of the built index files
 */
declare function getFiles(): Promise<GetFilesResponse>;

export interface GetFilesResponse {
    errors: string[],
    files: IndexFile[]
}

export interface IndexFile {
    path: string,
    content: Buffer
}