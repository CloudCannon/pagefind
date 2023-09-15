// this script should be imported on the result pages to enable highlighting
// after a user clicks on a result, the linked page should have this script to enable highlighting

import Mark from "mark.js";

// the separateWordSearch of mark options treats each space separated word as a separate search
// I am not letting the user set it, because it should be handled on our side
// if pagefind ever supports exact matches including spaces ('hello world'), then each sequence to be highlighted should be passed as an entry in the pagefind-highlight query param
// so if the search is "'hello world' lorem" then the query param should be "pagefind-highlight=hello%20world&pagefind-highlight=lorem"
// see the tests for more examples
// right now, since that isn't supported, to separateWordSearch should be false

type pagefindHighlightOptions = {
  markContext: string | HTMLElement | HTMLElement[] | NodeList | null;
  pagefindQueryParamName: string;
  markOptions: Omit<Mark.MarkOptions, "separateWordSearch">;
  addStyles: boolean;
};

export default class PagefindHighlight {
  pagefindQueryParamName: string;
  markContext: string | HTMLElement | HTMLElement[] | NodeList | null;
  markOptions: Mark.MarkOptions;
  addStyles: boolean;

  constructor(
    options: pagefindHighlightOptions = {
      markContext: null,
      pagefindQueryParamName: "pagefind-highlight",
      markOptions: {
        className: "pagefind__highlight",
        exclude: ["[data-pagefind-ignore]", "[data-pagefind-ignore] *"],
      },
      addStyles: true,
    }
  ) {
    const { pagefindQueryParamName, markContext, markOptions, addStyles } =
      options;

    this.pagefindQueryParamName = pagefindQueryParamName;
    this.addStyles = addStyles;
    this.markContext = markContext;
    this.markOptions = markOptions;

    // make sure these are always set
    // if the user doesn't want to exclude anything, they should pass an empty array
    // if the user doesn't want a className they should pass an empty string
    this.markOptions.className ??= "pagefind__highlight";
    this.markOptions.exclude ??= [
      "[data-pagefind-ignore]",
      "[data-pagefind-ignore] *",
    ];
    this.markOptions.separateWordSearch = false;
    this.highlight();
  }

  getHighlightParams(paramName: string): string[] {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.getAll(paramName);
  }

  // Inline styles might be too hard to override
  addHighlightStyles(className: string | undefined | null) {
    if (!className) return;
    const styleElement = document.createElement("style");
    styleElement.innerText = `:where(.${className}) { background-color: yellow; color: black; }`;
    document.head.appendChild(styleElement);
  }

  createMarkInstance() {
    if (this.markContext) {
      return new Mark(this.markContext);
    }
    const pagefindBody = document.querySelectorAll("[data-pagefind-body]");
    if (pagefindBody.length !== 0) {
      return new Mark(pagefindBody);
    } else {
      return new Mark(document.body);
    }
  }

  markText(instance: Mark, text: string[]) {
    instance.mark(text, this.markOptions);
  }

  highlight() {
    const params = this.getHighlightParams(this.pagefindQueryParamName);
    if (!params || params.length === 0) return;
    this.addStyles &&
      this.addHighlightStyles(this.markOptions.className as string);
    const markInstance = this.createMarkInstance();
    this.markText(markInstance, params);
  }
}

window.PagefindHighlight = PagefindHighlight;
