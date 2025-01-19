import { Pagefind } from "./coupled_search.js";

let pagefind: Pagefind | undefined = undefined;
let initial_options: PagefindIndexOptions | undefined = undefined;

const init_pagefind = () => {
  if (!pagefind) {
    pagefind = new Pagefind(initial_options ?? {});
  }
};

export const options = async (new_options: PagefindIndexOptions) => {
  if (pagefind) {
    await pagefind.options(new_options);
  } else {
    initial_options = new_options;
  }
};
export const init = async () => {
  init_pagefind();
};
export const destroy = async () => {
  pagefind = undefined;
  initial_options = undefined;
};

export const mergeIndex = async (
  indexPath: string,
  options: PagefindIndexOptions,
) => {
  init_pagefind();
  return await pagefind!.mergeIndex(indexPath, options);
};
export const search = async (term: string, options: PagefindSearchOptions) => {
  init_pagefind();
  return await pagefind!.search(term, options);
};
export const debouncedSearch = async (
  term: string,
  options: PagefindSearchOptions,
  debounceTimeoutMs: number = 300,
) => {
  init_pagefind();
  return await pagefind!.debouncedSearch(term, options, debounceTimeoutMs);
};
export const preload = async (term: string, options: PagefindSearchOptions) => {
  init_pagefind();
  return await pagefind!.preload(term, options);
};
export const filters = async () => {
  init_pagefind();
  return await pagefind!.filters();
};
