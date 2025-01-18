import "../../pagefind_web_js/types/index";

export {};

declare global {
  type PinnedPagefindSearchResult = {
    last_result: PagefindSearchResult;
    position: number;
  };
}
