import "pagefindWeb";

export type PagefindEntryJson = {
  version: string;
  languages: Record<string, PagefindEntryLanguage>;
  include_characters: string[];
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
  search_keywords?: string[];
};

export type PagefindSearchResponseResult = {
  /** Page hash */
  p: string;
  /** Page score */
  s: number;
  /** Matching word locations */
  l: PagefindSearchResponseResultWord[];
  /** verbose playground info */
  params?: PagefindSearchResponseResultParams;
  /** verbose playground info */
  scores?: PagefindSearchResponseResultScore[];
};

export type PagefindSearchResponseResultParams = {
  /** Document length */
  dl: number;
  /** Average page length */
  apl: number;
  /** Total pages */
  tp: number;
};

export type PagefindSearchResponseResultScore = {
  /** Word */
  w: string;
  /** Term IDF */
  idf: number;
  /** BM25's TF */
  b_tf: number;
  /** Raw TF */
  r_tf: number;
  /** Pagefind output TF */
  p_tf: number;
  /** Final term score */
  s: number;
  /** Input params */
  params: PagefindSearchResponseResultScoreParams;
};

export type PagefindSearchResponseResultScoreParams = {
  /** Weighted term frequency */
  w_tf: number;
  /** Pages containing term */
  pct: number;
  /** Length bonus */
  lb: number;
};

export type PagefindSearchResponseResultWord = {
  /** weight */
  w: number;
  /** balanced_score */
  s: number;
  /** word_location */
  l: number;
  /** verbose playground info */
  v?: PagefindSearchResponseResultVerboseWord;
};

export type PagefindSearchResponseResultVerboseWord = {
  /** word string */
  ws: string;
  /** length bonus */
  lb: number;
};
