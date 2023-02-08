export class Input {
    constructor(opts) {
        this.inputEl = null;
        this.instance = null;

        if (opts.inputElement) {
            if (opts.containerElement) {
                console.warn(`[Pagefind Input component]: inputElement and containerElement both supplied. Ignoring the container option.`);
                return;
            }

            this.initExisting(opts.inputElement);
        } else if (opts.containerElement) {
            this.initContainer(opts.containerElement);
        } else {
            console.error(`[Pagefind Input component]: No selector supplied for containerElement or inputElement`);
            return;
        }

        this.inputEl.addEventListener("input", (e) => {
            if (this.instance && typeof e?.target?.value === "string") {
                this.instance.triggerSearch(e.target.value);
            }
        });
    }

    initContainer(selector) {
        const container = document.querySelector(selector);
        if (!container) {
            console.error(`[Pagefind Input component]: No container found for ${selector} selector`);
            return;
        }
        if (container.tagName === "INPUT") {
            console.warn(`[Pagefind Input component]: Encountered input element for ${selector} when a container was expected`);
            console.warn(`[Pagefind Input component]: Treating containerElement option as inputElement and proceeding`);
            this.initExisting(selector);
        } else {
            this.inputEl = document.createElement("input");
            container.innerHTML = "";
            container.appendChild(this.inputEl);
        }
    }

    initExisting(selector) {
        const el = document.querySelector(selector);
        if (!el) {
            console.error(`[Pagefind Input component]: No input element found for ${selector} selector`);
            return;
        }
        if (el.tagName !== "INPUT") {
            console.error(`[Pagefind Input component]: Expected ${selector} to be an <input> element`);
            return;
        }
        this.inputEl = el;
    }

    register(instance) {
        this.instance = instance;
        this.instance.on("search", (term, _filters) => {
            if (this.inputEl && document.activeElement !== this.inputEl) {
                this.inputEl.value = term;
            }
        });
    }
}