import sharedStyles from "./shared-styles.css.js";

const asyncSleep = async (ms = 100) => {
    return new Promise(r => setTimeout(r, ms));
};

export class PageFindInput extends HTMLElement {
    static formAssociated = true;

    static observedAttributes = [
        "debounce-timeout-ms",
        "value",
    ];

    static #template = document.createElement('template');
    static #style = new CSSStyleSheet();
    static {
        this.#template.innerHTML = `
            <form role="search"
                  aria-label="Search this site"
                  action="javascript:void(0)">
                  <label for="pagefind-input"
                         data-pfmod-sr-hidden="true">Search this site</label>
                  <input id="pagefind-input"
                         autocapitalize="none"
                         enterkeyhint="search">
                  <button data-pfmod-suppressed="true">Clear</button>
            </form>
        `;

        this.#style.replaceSync(/* css */`
            :host {
                position: relative;
            }

            :host::before {
                background-color: var(--pagefind-ui-text);
                width: calc(18px * var(--pagefind-ui-scale));
                height: calc(18px * var(--pagefind-ui-scale));
                top: calc(23px * var(--pagefind-ui-scale));
                left: calc(20px * var(--pagefind-ui-scale));
                content: "";
                position: absolute;
                display: block;
                opacity: 0.7;
                -webkit-mask-image: url("data:image/svg+xml,%3Csvg width='18' height='18' viewBox='0 0 18 18' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M12.7549 11.255H11.9649L11.6849 10.985C12.6649 9.845 13.2549 8.365 13.2549 6.755C13.2549 3.165 10.3449 0.255005 6.75488 0.255005C3.16488 0.255005 0.254883 3.165 0.254883 6.755C0.254883 10.345 3.16488 13.255 6.75488 13.255C8.36488 13.255 9.84488 12.665 10.9849 11.685L11.2549 11.965V12.755L16.2549 17.745L17.7449 16.255L12.7549 11.255ZM6.75488 11.255C4.26488 11.255 2.25488 9.245 2.25488 6.755C2.25488 4.26501 4.26488 2.255 6.75488 2.255C9.24488 2.255 11.2549 4.26501 11.2549 6.755C11.2549 9.245 9.24488 11.255 6.75488 11.255Z' fill='%23000000'/%3E%3C/svg%3E%0A");
                mask-image: url("data:image/svg+xml,%3Csvg width='18' height='18' viewBox='0 0 18 18' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M12.7549 11.255H11.9649L11.6849 10.985C12.6649 9.845 13.2549 8.365 13.2549 6.755C13.2549 3.165 10.3449 0.255005 6.75488 0.255005C3.16488 0.255005 0.254883 3.165 0.254883 6.755C0.254883 10.345 3.16488 13.255 6.75488 13.255C8.36488 13.255 9.84488 12.665 10.9849 11.685L11.2549 11.965V12.755L16.2549 17.745L17.7449 16.255L12.7549 11.255ZM6.75488 11.255C4.26488 11.255 2.25488 9.245 2.25488 6.755C2.25488 4.26501 4.26488 2.255 6.75488 2.255C9.24488 2.255 11.2549 4.26501 11.2549 6.755C11.2549 9.245 9.24488 11.255 6.75488 11.255Z' fill='%23000000'/%3E%3C/svg%3E%0A");
                -webkit-mask-size: 100%;
                mask-size: 100%;
                z-index: 9;
                pointer-events: none;
            }

            input {
                height: calc(64px * var(--pagefind-ui-scale));
                padding: 0 calc(70px * var(--pagefind-ui-scale)) 0 calc(54px * var(--pagefind-ui-scale));
                background-color: var(--pagefind-ui-background);
                border: var(--pagefind-ui-border-width) solid var(--pagefind-ui-border);
                border-radius: var(--pagefind-ui-border-radius);
                font-size: calc(21px * var(--pagefind-ui-scale));
                position: relative;
                appearance: none;
                -webkit-appearance: none;
                display: flex;
                width: 100%;
                box-sizing: border-box;
                font-weight: 700;
            }

            input::placeholder {
                opacity: 0.2;
            }

            button {
                position: absolute;
                top: calc(2px * var(--pagefind-ui-scale));
                right: calc(2px * var(--pagefind-ui-scale));
                height: calc(60px * var(--pagefind-ui-scale));
                border-radius: var(--pagefind-ui-border-radius);
                padding: 0 calc(15px * var(--pagefind-ui-scale)) 0 calc(2px * var(--pagefind-ui-scale));
                color: var(--pagefind-ui-text);
                font-size: calc(14px * var(--pagefind-ui-scale));
                cursor: pointer;
                background-color: var(--pagefind-ui-background);
                border: none;
                appearance: none;
            }

        `);
    }

    #value = '';
    get value() { return this.#input?.value ?? '' }
    set value(v) {
        this.#value = v;
        this.updateState(v);
    }

    instance = null;

    searchID = 0;

    debounceTimeoutMs = 300;

    constructor() {
        super();
        if (!this.shadowRoot)
            this.attachShadow({ mode: 'open', delegatesFocus: true })
                .append(PageFindInput.#template.content.cloneNode(true));
        this.shadowRoot.adoptedStyleSheets = [sharedStyles, PageFindInput.#style];
        this.#input = this.shadowRoot?.querySelector('input');
        this.#button = this.shadowRoot?.querySelector('button');

        if (!this.#input)
            console.error(`[<pagefind-input>]: No input element found in shadow root`);
        if (!this.#button)
            console.error(`[<pagefind-input>]: No button element found in shadow root`);

        this.addEventListener("input", async (e) => {
            if (this.instance && typeof e?.target?.value === "string") {
                this.updateState(e.target.value);

                const thisSearchID = ++this.searchID;
                await asyncSleep(this.debounceTimeoutMs);

                if (thisSearchID !== this.searchID) {
                    return null;
                }

                this.instance?.triggerSearch(e.target.value);
            }
        });
        this.#input?.addEventListener("keydown", (e) => {
            if (e.key === "Escape") {
                ++this.searchID;
                this.value = "";
                this.instance?.triggerSearch("");
                this.updateState("");
            }
            if (e.key === "Enter") {
                e.preventDefault();
            }
        });
        this.addEventListener("focus", () => {
            this.instance?.triggerLoad();
        });
        this.#button?.addEventListener("click", () => {
            this.value = "";
            this.instance.triggerSearch("");
            this.updateState("");
        });
    }

    attributeChangedCallback(name, _oldValue, newValue) {
        switch (name) {
            case "debounce-timeout-ms":
                this.#debounceTimeoutMsChanged(newValue);
                break;
            case "value":
                this.value = newValue ?? ''
                break;
        }
    }

    connectedCallback() {
        if (this.hasAttribute("debounce-timeout-ms"))
            this.#debounceTimeoutMsChanged(this.getAttribute("debounce-timeout-ms"));
    }

    #debounceTimeoutMsChanged(attrVal) {
        if (!attrVal) {
            this.debounceTimeoutMs = 300;
        } else {
            const parsed = parseInt(attrVal);
            if (!Number.isNaN(parsed))
                this.debounceTimeoutMs = parsed;
        }
    }

    updateState(term) {
        if (this.#input)
            this.#input.value = term;
        if (this.clearEl) {
            if (term && term?.length) {
                this.clearEl.removeAttribute("data-pfmod-suppressed");
            } else {
                this.clearEl.setAttribute("data-pfmod-suppressed", "true");
            }
        }
    }

    register(instance) {
        this.instance = instance;
        this.instance.on("search", (term, _filters) => {
            if (this.getRootNode().activeElement !== this) {
                this.value = term;
            }
        });
    }
}

customElements.define('pagefind-input', PageFindInput);
