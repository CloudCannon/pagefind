import El from "../helpers/element-builder";

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
    const placeholder = (max = 30) => {
        return ". ".repeat(Math.floor(10 + Math.random() * max));
    };
    return `<li class="pagefind-modular-list-result">
    <div class="pagefind-modular-list-thumb" data-pfmod-loading></div>
    <div class="pagefind-modular-list-inner">
        <p class="pagefind-modular-list-title" data-pfmod-loading>${placeholder(30)}</p>
        <p class="pagefind-modular-list-excerpt" data-pfmod-loading>${placeholder(40)}</p>
    </div>
</li>`;
}

const resultTemplate = (result) => {
    let wrapper = new El("li").class("pagefind-modular-list-result");

    let thumb = new El("div").class("pagefind-modular-list-thumb").addTo(wrapper);
    if (result?.meta?.image) {
        new El("img").class("pagefind-modular-list-image").attrs({
            src: result.meta.image,
            alt: result.meta.image_alt || result.meta.title
        }).addTo(thumb);
    }

    let inner = new El("div").class("pagefind-modular-list-inner").addTo(wrapper);
    let title = new El("p").class("pagefind-modular-list-title").addTo(inner);
    new El("a").class("pagefind-modular-list-link").text(result.meta?.title).attrs({
        href: result.meta?.url || result.url
    }).addTo(title);

    new El("p").class("pagefind-modular-list-excerpt").html(result.excerpt).addTo(inner);

    return wrapper.element;
}

const nearestScrollParent = (el) => {
    if (!(el instanceof HTMLElement)) return null;
    const overflowY = window.getComputedStyle(el).overflowY;
    const isScrollable = overflowY !== 'visible' && overflowY !== 'hidden';

    if (isScrollable) {
        return el;
    } else {
        return nearestScrollParent(el.parentNode);
    }
}

class Result {
    constructor(opts = {}) {
        this.rawResult = opts.result;
        this.placeholderNodes = opts.placeholderNodes;
        this.resultFn = opts.resultFn;
        this.intersectionEl = opts.intersectionEl;
        this.result = null;
        this.waitForIntersection();
    }

    waitForIntersection() {
        if (!this.placeholderNodes?.length) return;

        let options = {
            root: this.intersectionEl,
            rootMargin: "0px",
            threshold: 0.01,
        };
        
        let observer = new IntersectionObserver((entries, observer) => {
            if (this.result !== null) return;
            if (entries?.[0]?.isIntersecting) {
                this.load();
                observer.disconnect();
            }
        }, options);

        observer.observe(this.placeholderNodes[0]);
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
        this.intersectionEl = document.body;
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
            this.intersectionEl = nearestScrollParent(this.containerEl);
            this.results = results.results.map(r => {
                let placeholderNodes = templateNodes(this.placeholderTemplate());
                this.append(placeholderNodes);
                return new Result({ result: r, placeholderNodes, resultFn: this.resultTemplate, intersectionEl: this.intersectionEl });
            })
        });

        instance.on("loading", () => {
            if (!this.containerEl) return;
            this.containerEl.innerHTML = "";
        });
    }
}