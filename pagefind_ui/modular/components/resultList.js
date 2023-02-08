const templateNodes = (templateResult) => {
    if (templateResult instanceof Element) {
        return [templateResult];
    } else if (Array.isArray(templateResult) && templateResult.every(r => r instanceof Element)) {
        return templateResult;
    } else if (typeof templateResult === "string" || templateResult instanceof String) {
        let wrap = document.createElement("div");
        wrap.innerHTML = templateResult;
        return [...wrap.childNodes]
    } else {
        console.error(`[Pagefind ResultList component]: Expected template function to return an HTML element or string, got ${typeof templateResult}`);
        return [];
    }
}

const placeholderTemplate = () => {
    return `<li>
    <p>......</p>
    <p>.................</p>
</li>`;
}

const resultTemplate = (result) => {
    let wrapper = document.createElement("li");
    wrapper.innerText = result?.meta?.title;
    return wrapper;
}

class Result {
    constructor(opts) {
        this.rawResult = opts.result;
        this.placeholderNodes = opts.placeholderNodes;
        this.resultFn = opts.resultFn;
        this.result = null;
        this.load();
    }

    async load() {
        if (!this.placeholderNodes?.length) return;

        this.result = await this.rawResult.data();
        const resultTemplate = this.resultFn(this.result);
        const resultNodes = templateNodes(resultTemplate);

        while (this.placeholderNodes.length > 1) {
            this.placeholderNodes.pop().remove();
        }

        this.placeholderNodes[0].replaceWith(...resultNodes);
    }
}

export class ResultList {
    constructor(opts) {
        this.containerEl = null;
        this.results = [];
        this.placeholderTemplate = opts.placeholderTemplate ?? placeholderTemplate;
        this.resultTemplate = opts.resultTemplate ?? resultTemplate;

        if (opts.containerElement) {
            this.initContainer(opts.containerElement);
        } else {
            console.error(`[Pagefind ResultList component]: No selector supplied for containerElement`);
            return;
        }
    }

    initContainer(selector) {
        const container = document.querySelector(selector);
        if (!container) {
            console.error(`[Pagefind ResultList component]: No container found for ${selector} selector`);
            return;
        }

        this.containerEl = container;
    }

    append(nodes) {
        for (const node of nodes) {
            this.containerEl.appendChild(node);
        }
    }

    register(instance) {
        instance.on("results", (results) => {
            if (!this.containerEl) return;
            this.containerEl.innerHTML = "";
            this.results = results.results.map(r => {
                let placeholderNodes = templateNodes(this.placeholderTemplate());
                this.append(placeholderNodes);
                return new Result({ result: r, placeholderNodes, resultFn: this.resultTemplate });
            })
        });
    }
}