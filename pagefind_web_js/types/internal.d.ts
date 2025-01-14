import "pagefindWeb";

export type PagefindEntryJson = {
  version: string;
  languages: Record<string, PagefindEntryLanguage>;
};

export type PagefindEntryLanguage = {
  hash: string;
  wasm?: string;
  page_count: number;
};

export type PagefindSearchResponse = {
  filtered_counts: PagefindFilterCounts;
  total_counts: PagefindFilterCounts;
  unfiltered_total: number;
  results: PagefindSearchResponseResult[];
};

export type PagefindSearchResponseResult = {
  /** Page hash */
  p: string;
  /** Page score */
  s: number;
  /** Matching word locations */
  l: PagefindSearchResponseResultWord[];
};

export type PagefindSearchResponseResultWord = {
  /** weight */
  w: number;
  /** balanced_score */
  s: number;
  /** word_location */
  l: number;
};
