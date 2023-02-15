export default class El {
    constructor(tagname) {
        this.element = document.createElement(tagname);
    }

    class(s) {
        this.element.classList.add(s);
        return this;
    }

    attrs(obj) {
        for (const [k,v] of Object.entries(obj)) {
            this.element.setAttribute(k, v);
        }
        return this;
    }

    text(t) {
        this.element.innerText = t;
        return this;
    }

    html(t) {
        this.element.innerHTML = t;
        return this;
    }

    handle(e, f) {
        this.element.addEventListener(e, f);
        return this;
    }

    addTo(el) {
        if (el instanceof El) {
            el.element.appendChild(this.element);
        } else {
            el.appendChild(this.element);
        }
        return this.element;
    }
}