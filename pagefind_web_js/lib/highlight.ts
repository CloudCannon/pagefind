// this script should be imported on the result pages to enable highlighting

import Mark from "mark.js";

// tbh not sure how to read this option

// I think it's ok to let the user decide when to run this script
// waiting for DOMContentLoaded doesn't work if it already is loaded
// Ik I could work around, but this is simpler

// TODO use browser api to read query param, make sure special chars get encoded/decoded

// the separateWordSearch of mark options treats each space separated word as a separate search
// I am not letting the user set it, because it should be handled on our side
// if pagefind ever supports exact matches, including spaces ('hello world'), then this should be passed as an entry in the pagefind-highlight query param
// see the tests for more examples

// right now, since that isn't supported, to separateWordSearch should be false


  type pagefindHighlightOptions = {
    markContext: string | HTMLElement | HTMLElement[] | NodeList | null;
    pagefindQueryParamName: string;
    // ? should this be an option?
    highlightNodeElementName: string;
    highlightNodeClassName: string;
    markOptions: Omit<Mark.MarkOptions, "separateWordSearch"> | undefined;
    addStyles: boolean;
  };

export default class PagefindHighlight {
  pagefindQueryParamName: string;
  // ? should this be an option?
  highlightNodeElementName: string;
  highlightNodeClassName: string;
  markContext: string | HTMLElement | HTMLElement[] | NodeList | null;
  markOptions: Mark.MarkOptions;
  addStyles: boolean;

  // TODO type constructor options better

  constructor(
    options: pagefindHighlightOptions = {
      markContext: null,
      pagefindQueryParamName: "pagefind-highlight",
      // ? should this be an option?
      highlightNodeElementName: "mark",
      highlightNodeClassName: "pagefind__highlight",
      markOptions: undefined,
      addStyles: true,
    }
  ) {
    const {
      pagefindQueryParamName,
      highlightNodeElementName,
      highlightNodeClassName,
      markContext,
      markOptions,
      addStyles,
    } = options;

    this.pagefindQueryParamName = pagefindQueryParamName;
    this.highlightNodeElementName = highlightNodeElementName || "mark";
    this.highlightNodeClassName = highlightNodeClassName;
    this.addStyles = addStyles;
    this.markContext = markContext;

    if (markOptions) {
      this.markOptions = markOptions;
    } else {
      this.markOptions = {
        className: this.highlightNodeClassName,
        exclude: ["*[data-pagefind-ignore]", "[data-pagefind-ignore] *"],
      };
    }
    this.markOptions.separateWordSearch = false;

    this.highlight();
  }

  // wait for the DOM to be ready
  // read the query param
  // find all occurrences of the query param in the DOM, respecting the data-pagefind attributes
  // wrap the text in a mark with a class of pagefind__highlight

  // TODO return array and get all params (to highlight multiple entitles (ex: 'hello world' and 'potato')))

  getHighlightParams(paramName: string): string[] {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.getAll(paramName);
  }

  // Inline styles might be too hard to override
  addHighlightStyles(className: string) {
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
    this.addStyles && this.addHighlightStyles(this.highlightNodeClassName);
    const markInstance = this.createMarkInstance();
    this.markText(markInstance, params);
  }
}

window.PagefindHighlight = PagefindHighlight;
