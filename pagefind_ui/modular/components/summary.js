export class Summary {
    constructor(opts = {}) {
        this.containerEl = null;
        this.defaultMessage = opts.defaultMessage ?? "";
        this.term = "";

        if (opts.containerElement) {
            this.initContainer(opts.containerElement);
        } else {
            console.error(`[Pagefind Summary component]: No selector supplied for containerElement`);
            return;
        }
    }

    initContainer(selector) {
        const container = document.querySelector(selector);
        if (!container) {
            console.error(`[Pagefind Summary component]: No container found for ${selector} selector`);
            return;
        }

        this.containerEl = container;
        this.containerEl.innerText = this.defaultMessage;
    }

    register(instance) {
        instance.on("search", (term, _filters) => {
            this.term = term;
        });

        instance.on("results", (results) => {
            if (!this.containerEl || !results) return;
            if (!this.term) {
                this.containerEl.innerText = this.defaultMessage;
                return;
            }
            let count = results?.results?.length ?? 0;
            this.containerEl.innerText = `${count} result${count === 1 ? '' : 's'} for ${this.term}`;
        });

        instance.on("loading", () => {
            if (!this.containerEl) return;
            this.containerEl.innerText = `Searching for ${this.term}...`;
        });
    }
}