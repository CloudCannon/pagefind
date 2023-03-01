import El from "../helpers/element-builder";

const asyncSleep = async (ms = 100) => {
    return new Promise(r => setTimeout(r, ms));
};

export class FilterPills {
    constructor(opts = {}) {
        this.instance = null;
        this.wrapper = null;
        this.pillContainer = null;
        this.available = {};
        this.selected = ["All"];
        this.total = 0;
        this.filterMemo = "";

        this.filter = opts.filter;
        this.ordering = opts.ordering ?? null;
        this.alwaysShow = opts.alwaysShow ?? false;
        this.selectMultiple = opts.selectMultiple ?? false;

        if (!this.filter?.length) {
            console.error(`[Pagefind FilterPills component]: No filter option supplied, nothing to display`);
            return;
        }

        if (opts.containerElement) {
            this.initContainer(opts.containerElement);
        } else {
            console.error(`[Pagefind FilterPills component]: No selector supplied for containerElement`);
            return;
        }
    }

    initContainer(selector) {
        const container = document.querySelector(selector);
        if (!container) {
            console.error(`[Pagefind FilterPills component]: No container found for ${selector} selector`);
            return;
        }
        container.innerHTML = "";
        const id = `pagefind_modular_filter_pills_${this.filter}`;


        const wrapper = new El("div")
            .class("pagefind-modular-filter-pills-wrapper")
            .attrs({
                "role": "group",
                "aria-labelledby": id,
            });
        if (!this.alwaysShow) {
            wrapper.attrs({"data-pfmod-hidden": true});
        }
        
        new El("div")
            .id(id)
            .class("pagefind-modular-filter-pills-label")
            .attrs({
                "data-pfmod-sr-hidden": true
            })
            .text(`Filter results by ${this.filter}`)
            .addTo(wrapper);

        this.pillContainer = new El("div")
            .class("pagefind-modular-filter-pills")
            .addTo(wrapper);

        this.wrapper = wrapper.addTo(container);
    }

    update() {
        const filterMemo = this.available.map(a => a[0]).join("~");
        if (filterMemo == this.filterMemo) {
            this.updateExisting();
        } else {
            this.renderNew();
            this.filterMemo = filterMemo;
        }
    }

    pushFilters() {
        const selected = this.selected.filter(v => v !== "All");
        this.instance.triggerFilter(this.filter, selected);
    }

    pillInner(val, count) {
        if (this.total) {
            return `<span aria-label="${val}">${val} (${count})</span>`;
        } else {
            return `<span aria-label="${val}">${val}</span>`;
        }
    }

    renderNew() {
        this.available.forEach(([val, count]) => {
            new El("button")
                .class("pagefind-modular-filter-pill")
                .html(this.pillInner(val, count))
                .attrs({
                    "aria-pressed": this.selected.includes(val),
                    "type": "button",
                })
                .handle("click", () => {
                    if (val === "All") {
                        this.selected = ["All"];
                    } else if (this.selected.includes(val)) {
                        this.selected = this.selected.filter(v => v !== val);
                    } else if (this.selectMultiple) {
                        this.selected.push(val);
                    } else {
                        this.selected = [val];
                    }
                    if (!this.selected?.length) {
                        this.selected = ["All"];
                    } else if (this.selected?.length > 1) {
                        this.selected = this.selected.filter(v => v !== "All");
                    }
                    this.update();
                    this.pushFilters();
                })
                .addTo(this.pillContainer);
        });
    }

    updateExisting() {
        const pills = [...this.pillContainer.childNodes];
        this.available.forEach(([val, count], i) => {
            pills[i].innerHTML = this.pillInner(val, count);
            pills[i].setAttribute("aria-pressed", this.selected.includes(val));
        });
    }

    register(instance) {
        this.instance = instance;
        this.instance.on("filters", (filters) => {
            if (!this.pillContainer) return;

            if (this.selectMultiple) {
                filters = filters.available;
            } else {
                filters = filters.total;
            }

            let newlyAvailable = filters[this.filter];
            if (!newlyAvailable) {
                console.warn(`[Pagefind FilterPills component]: No possible values found for the ${this.filter} filter`);
                return;
            }
            this.available = Object.entries(newlyAvailable);

            if (Array.isArray(this.ordering)) {
                this.available.sort((a, b) => {
                    const apos = this.ordering.indexOf(a[0]);
                    const bpos = this.ordering.indexOf(b[0]);
                    return (apos === -1 ? Infinity : apos) - (bpos === -1 ? Infinity : bpos);
                });
            } else {
                this.available.sort((a, b) => {
                    return a[0].localeCompare(b[0]);
                });
            }
            this.available.unshift(["All", this.total]);
            this.update();
        });

        instance.on("results", (results) => {
            if (!this.pillContainer) return;
            this.total = results?.unfilteredResultCount || 0;

            if (this.available?.[0]?.[0] === "All") {
                this.available[0][1] = this.total;
            }

            if (this.total || this.alwaysShow) {
                this.wrapper.removeAttribute("data-pfmod-hidden");
            } else {
                this.wrapper.setAttribute("data-pfmod-hidden", "true");
            }
            this.update();
        });
    }
}