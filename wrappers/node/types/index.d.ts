/**
 * Create a new Pagefind index that files can be added to
 */
export function createIndex(): Promise<NewIndexResponse>;

export interface NewIndexResponse {
    errors: string[],
    index?: PagefindIndex
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

export interface PagefindIndex {
    addFile(filePath: string, fileContents: string): Promise<NewFileResponse>,
    writeFiles(): Promise<WriteFilesResponse>,
}
