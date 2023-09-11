// this script should be imported on the result pages to enable highlighting

import Mark from "mark.js";

// tbh not sure how to read this option

// I think it's ok to let the user decide when to run this script
// waiting for DOMContentLoaded doesn't work if it already is loaded
// Ik I could work around, but this is simpler

export default class PagefindHighlight {
  pagefindQueryParamName: string;
  // ? should this be an option?
  highlightNodeElementName: string;
  highlightNodeClassName: string;
  markContext: string | HTMLElement | HTMLElement[] | NodeList | null;
  markOptions: Mark.MarkOptions;
  addStyles: boolean;

  constructor(
    options = {
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
        exclude: ["[data-pagefind-ignore]"],
      };
    }

    this.highlight();
  }

  // wait for the DOM to be ready
  // read the query param
  // find all occurrences of the query param in the DOM, respecting the data-pagefind attributes
  // wrap the text in a mark with a class of pagefind__highlight

  getHighlightParam(paramName: string): string {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get(paramName) || "";
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

  markText(instance: Mark, text: string) {
    instance.mark(text, this.markOptions);
  }

  highlight() {
    const param = this.getHighlightParam(this.pagefindQueryParamName);
    if (!param) return;
    this.addStyles && this.addHighlightStyles(this.highlightNodeClassName);
    const markInstance = this.createMarkInstance();
    this.markText(markInstance, param);
  }
}

window.PagefindHighlight = PagefindHighlight;
