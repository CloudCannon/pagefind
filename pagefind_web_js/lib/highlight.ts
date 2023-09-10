// document.querySelector(".test")!.innerHTML = "Hello from the highlight script";

// this script should be imported on the result pages to enable highlighting

// tbh not sure how to read this option
const pagefindQueryParamName = "pagefind-highlight";
const highlightNodeElementName = "mark";
const highlightNodeClassName = "pagefind__highlight";

// wait for the DOM to be ready
// read the query param
// find all occurrences of the query param in the DOM, respecting the data-pagefind attributes
// wrap the text in a mark with a class of pagefind__highlight

// code from https://stackoverflow.com/a/31369978

function getElementsToHighlight() {
  // could have more than one element with [data-pagefind-body]
  // make sure it falls back correctly if no [data-pagefind-body]
  // should fall back to the root selector ig
  let pagefindBody =
    document.querySelectorAll("[data-pagefind-body]") || document.body;
}

function highlight(element, regex: RegExp) {
  let document = element.ownerDocument;

  let nodes = [],
    text = "",
    node,
    nodeIterator = document.createNodeIterator(
      element,
      NodeFilter.SHOW_TEXT,
      null,
      false
    );

  while ((node = nodeIterator.nextNode())) {
    nodes.push({
      textNode: node,
      start: text.length,
    });
    text += node.nodeValue;
  }

  if (!nodes.length) return;

  let match;
  while ((match = regex.exec(text))) {
    let matchLength = match[0].length;

    // Prevent empty matches causing infinite loops
    if (!matchLength) {
      regex.lastIndex++;
      continue;
    }

    for (let i = 0; i < nodes.length; ++i) {
      node = nodes[i];
      let nodeLength = node.textNode.nodeValue.length;

      // Skip nodes before the match
      if (node.start + nodeLength <= match.index) continue;

      // Break after the match
      if (node.start >= match.index + matchLength) break;

      // Split the start node if required
      if (node.start < match.index) {
        nodes.splice(i + 1, 0, {
          textNode: node.textNode.splitText(match.index - node.start),
          start: match.index,
        });
        continue;
      }

      // Split the end node if required
      if (node.start + nodeLength > match.index + matchLength) {
        nodes.splice(i + 1, 0, {
          textNode: node.textNode.splitText(
            match.index + matchLength - node.start
          ),
          start: match.index + matchLength,
        });
      }

      // Highlight the current node
      let highlightNode = document.createElement(highlightNodeElementName);
      highlightNode.className = highlightNodeClassName;

      node.textNode.parentNode.replaceChild(highlightNode, node.textNode);
      highlightNode.appendChild(node.textNode);
    }
  }
}

if (window) {
  window.addEventListener("DOMContentLoaded", () => {
    const query = new URLSearchParams(window.location.search).get(
      pagefindQueryParamName
    );
    if (!query) return;

    // regex to match the query param
    const queryRegex = new RegExp(query, "gi");

    highlight(getElementsToHighlight(), queryRegex);

    // add styles
    document.head.appendChild(
      document.createElement("style")
    ).textContent = `:where(.${highlightNodeClassName}) { background-color: yellow; text-color: #ccc;}`;
  });
}
