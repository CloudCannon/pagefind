import "pagefindWeb"

export type PagefindEntryJson = {
    version: string,
    languages: Record<string, PagefindEntryLanguage>,
}

export type PagefindEntryLanguage = {
    hash: string,
    wasm?: string,
    page_count: number,
}
