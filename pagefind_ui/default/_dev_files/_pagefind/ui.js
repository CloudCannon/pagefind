(() => {
  var __defProp = Object.defineProperty;
  var __export = (target, all) => {
    for (var name in all)
      __defProp(target, name, { get: all[name], enumerable: true });
  };

  // node_modules/svelte/internal/index.mjs
  function noop() {
  }
  function run(fn) {
    return fn();
  }
  function blank_object() {
    return /* @__PURE__ */ Object.create(null);
  }
  function run_all(fns) {
    fns.forEach(run);
  }
  function is_function(thing) {
    return typeof thing === "function";
  }
  function safe_not_equal(a, b) {
    return a != a ? b == b : a !== b || (a && typeof a === "object" || typeof a === "function");
  }
  var src_url_equal_anchor;
  function src_url_equal(element_src, url) {
    if (!src_url_equal_anchor) {
      src_url_equal_anchor = document.createElement("a");
    }
    src_url_equal_anchor.href = url;
    return element_src === src_url_equal_anchor.href;
  }
  function is_empty(obj) {
    return Object.keys(obj).length === 0;
  }
  var is_hydrating = false;
  function start_hydrating() {
    is_hydrating = true;
  }
  function end_hydrating() {
    is_hydrating = false;
  }
  function append(target, node) {
    target.appendChild(node);
  }
  function insert(target, node, anchor) {
    target.insertBefore(node, anchor || null);
  }
  function detach(node) {
    if (node.parentNode) {
      node.parentNode.removeChild(node);
    }
  }
  function destroy_each(iterations, detaching) {
    for (let i = 0; i < iterations.length; i += 1) {
      if (iterations[i])
        iterations[i].d(detaching);
    }
  }
  function element(name) {
    return document.createElement(name);
  }
  function svg_element(name) {
    return document.createElementNS("http://www.w3.org/2000/svg", name);
  }
  function text(data) {
    return document.createTextNode(data);
  }
  function space() {
    return text(" ");
  }
  function empty() {
    return text("");
  }
  function listen(node, event, handler, options) {
    node.addEventListener(event, handler, options);
    return () => node.removeEventListener(event, handler, options);
  }
  function attr(node, attribute, value) {
    if (value == null)
      node.removeAttribute(attribute);
    else if (node.getAttribute(attribute) !== value)
      node.setAttribute(attribute, value);
  }
  function children(element2) {
    return Array.from(element2.childNodes);
  }
  function set_data(text2, data) {
    data = "" + data;
    if (text2.wholeText !== data)
      text2.data = data;
  }
  function set_input_value(input, value) {
    input.value = value == null ? "" : value;
  }
  function toggle_class(element2, name, toggle) {
    element2.classList[toggle ? "add" : "remove"](name);
  }
  var HtmlTag = class {
    constructor(is_svg = false) {
      this.is_svg = false;
      this.is_svg = is_svg;
      this.e = this.n = null;
    }
    c(html) {
      this.h(html);
    }
    m(html, target, anchor = null) {
      if (!this.e) {
        if (this.is_svg)
          this.e = svg_element(target.nodeName);
        else
          this.e = element(target.nodeName);
        this.t = target;
        this.c(html);
      }
      this.i(anchor);
    }
    h(html) {
      this.e.innerHTML = html;
      this.n = Array.from(this.e.childNodes);
    }
    i(anchor) {
      for (let i = 0; i < this.n.length; i += 1) {
        insert(this.t, this.n[i], anchor);
      }
    }
    p(html) {
      this.d();
      this.h(html);
      this.i(this.a);
    }
    d() {
      this.n.forEach(detach);
    }
  };
  var current_component;
  function set_current_component(component) {
    current_component = component;
  }
  function get_current_component() {
    if (!current_component)
      throw new Error("Function called outside component initialization");
    return current_component;
  }
  function onMount(fn) {
    get_current_component().$$.on_mount.push(fn);
  }
  var dirty_components = [];
  var binding_callbacks = [];
  var render_callbacks = [];
  var flush_callbacks = [];
  var resolved_promise = Promise.resolve();
  var update_scheduled = false;
  function schedule_update() {
    if (!update_scheduled) {
      update_scheduled = true;
      resolved_promise.then(flush);
    }
  }
  function add_render_callback(fn) {
    render_callbacks.push(fn);
  }
  function add_flush_callback(fn) {
    flush_callbacks.push(fn);
  }
  var seen_callbacks = /* @__PURE__ */ new Set();
  var flushidx = 0;
  function flush() {
    if (flushidx !== 0) {
      return;
    }
    const saved_component = current_component;
    do {
      try {
        while (flushidx < dirty_components.length) {
          const component = dirty_components[flushidx];
          flushidx++;
          set_current_component(component);
          update(component.$$);
        }
      } catch (e) {
        dirty_components.length = 0;
        flushidx = 0;
        throw e;
      }
      set_current_component(null);
      dirty_components.length = 0;
      flushidx = 0;
      while (binding_callbacks.length)
        binding_callbacks.pop()();
      for (let i = 0; i < render_callbacks.length; i += 1) {
        const callback = render_callbacks[i];
        if (!seen_callbacks.has(callback)) {
          seen_callbacks.add(callback);
          callback();
        }
      }
      render_callbacks.length = 0;
    } while (dirty_components.length);
    while (flush_callbacks.length) {
      flush_callbacks.pop()();
    }
    update_scheduled = false;
    seen_callbacks.clear();
    set_current_component(saved_component);
  }
  function update($$) {
    if ($$.fragment !== null) {
      $$.update();
      run_all($$.before_update);
      const dirty = $$.dirty;
      $$.dirty = [-1];
      $$.fragment && $$.fragment.p($$.ctx, dirty);
      $$.after_update.forEach(add_render_callback);
    }
  }
  var outroing = /* @__PURE__ */ new Set();
  var outros;
  function group_outros() {
    outros = {
      r: 0,
      c: [],
      p: outros
      // parent group
    };
  }
  function check_outros() {
    if (!outros.r) {
      run_all(outros.c);
    }
    outros = outros.p;
  }
  function transition_in(block, local) {
    if (block && block.i) {
      outroing.delete(block);
      block.i(local);
    }
  }
  function transition_out(block, local, detach2, callback) {
    if (block && block.o) {
      if (outroing.has(block))
        return;
      outroing.add(block);
      outros.c.push(() => {
        outroing.delete(block);
        if (callback) {
          if (detach2)
            block.d(1);
          callback();
        }
      });
      block.o(local);
    } else if (callback) {
      callback();
    }
  }
  var globals = typeof window !== "undefined" ? window : typeof globalThis !== "undefined" ? globalThis : global;
  function outro_and_destroy_block(block, lookup) {
    transition_out(block, 1, 1, () => {
      lookup.delete(block.key);
    });
  }
  function update_keyed_each(old_blocks, dirty, get_key, dynamic, ctx, list, lookup, node, destroy, create_each_block4, next, get_context) {
    let o = old_blocks.length;
    let n = list.length;
    let i = o;
    const old_indexes = {};
    while (i--)
      old_indexes[old_blocks[i].key] = i;
    const new_blocks = [];
    const new_lookup = /* @__PURE__ */ new Map();
    const deltas = /* @__PURE__ */ new Map();
    i = n;
    while (i--) {
      const child_ctx = get_context(ctx, list, i);
      const key = get_key(child_ctx);
      let block = lookup.get(key);
      if (!block) {
        block = create_each_block4(key, child_ctx);
        block.c();
      } else if (dynamic) {
        block.p(child_ctx, dirty);
      }
      new_lookup.set(key, new_blocks[i] = block);
      if (key in old_indexes)
        deltas.set(key, Math.abs(i - old_indexes[key]));
    }
    const will_move = /* @__PURE__ */ new Set();
    const did_move = /* @__PURE__ */ new Set();
    function insert2(block) {
      transition_in(block, 1);
      block.m(node, next);
      lookup.set(block.key, block);
      next = block.first;
      n--;
    }
    while (o && n) {
      const new_block = new_blocks[n - 1];
      const old_block = old_blocks[o - 1];
      const new_key = new_block.key;
      const old_key = old_block.key;
      if (new_block === old_block) {
        next = new_block.first;
        o--;
        n--;
      } else if (!new_lookup.has(old_key)) {
        destroy(old_block, lookup);
        o--;
      } else if (!lookup.has(new_key) || will_move.has(new_key)) {
        insert2(new_block);
      } else if (did_move.has(old_key)) {
        o--;
      } else if (deltas.get(new_key) > deltas.get(old_key)) {
        did_move.add(new_key);
        insert2(new_block);
      } else {
        will_move.add(old_key);
        o--;
      }
    }
    while (o--) {
      const old_block = old_blocks[o];
      if (!new_lookup.has(old_block.key))
        destroy(old_block, lookup);
    }
    while (n)
      insert2(new_blocks[n - 1]);
    return new_blocks;
  }
  function bind(component, name, callback) {
    const index = component.$$.props[name];
    if (index !== void 0) {
      component.$$.bound[index] = callback;
      callback(component.$$.ctx[index]);
    }
  }
  function create_component(block) {
    block && block.c();
  }
  function mount_component(component, target, anchor, customElement) {
    const { fragment, after_update } = component.$$;
    fragment && fragment.m(target, anchor);
    if (!customElement) {
      add_render_callback(() => {
        const new_on_destroy = component.$$.on_mount.map(run).filter(is_function);
        if (component.$$.on_destroy) {
          component.$$.on_destroy.push(...new_on_destroy);
        } else {
          run_all(new_on_destroy);
        }
        component.$$.on_mount = [];
      });
    }
    after_update.forEach(add_render_callback);
  }
  function destroy_component(component, detaching) {
    const $$ = component.$$;
    if ($$.fragment !== null) {
      run_all($$.on_destroy);
      $$.fragment && $$.fragment.d(detaching);
      $$.on_destroy = $$.fragment = null;
      $$.ctx = [];
    }
  }
  function make_dirty(component, i) {
    if (component.$$.dirty[0] === -1) {
      dirty_components.push(component);
      schedule_update();
      component.$$.dirty.fill(0);
    }
    component.$$.dirty[i / 31 | 0] |= 1 << i % 31;
  }
  function init(component, options, instance4, create_fragment4, not_equal, props, append_styles, dirty = [-1]) {
    const parent_component = current_component;
    set_current_component(component);
    const $$ = component.$$ = {
      fragment: null,
      ctx: [],
      // state
      props,
      update: noop,
      not_equal,
      bound: blank_object(),
      // lifecycle
      on_mount: [],
      on_destroy: [],
      on_disconnect: [],
      before_update: [],
      after_update: [],
      context: new Map(options.context || (parent_component ? parent_component.$$.context : [])),
      // everything else
      callbacks: blank_object(),
      dirty,
      skip_bound: false,
      root: options.target || parent_component.$$.root
    };
    append_styles && append_styles($$.root);
    let ready = false;
    $$.ctx = instance4 ? instance4(component, options.props || {}, (i, ret, ...rest) => {
      const value = rest.length ? rest[0] : ret;
      if ($$.ctx && not_equal($$.ctx[i], $$.ctx[i] = value)) {
        if (!$$.skip_bound && $$.bound[i])
          $$.bound[i](value);
        if (ready)
          make_dirty(component, i);
      }
      return ret;
    }) : [];
    $$.update();
    ready = true;
    run_all($$.before_update);
    $$.fragment = create_fragment4 ? create_fragment4($$.ctx) : false;
    if (options.target) {
      if (options.hydrate) {
        start_hydrating();
        const nodes = children(options.target);
        $$.fragment && $$.fragment.l(nodes);
        nodes.forEach(detach);
      } else {
        $$.fragment && $$.fragment.c();
      }
      if (options.intro)
        transition_in(component.$$.fragment);
      mount_component(component, options.target, options.anchor, options.customElement);
      end_hydrating();
      flush();
    }
    set_current_component(parent_component);
  }
  var SvelteElement;
  if (typeof HTMLElement === "function") {
    SvelteElement = class extends HTMLElement {
      constructor() {
        super();
        this.attachShadow({ mode: "open" });
      }
      connectedCallback() {
        const { on_mount } = this.$$;
        this.$$.on_disconnect = on_mount.map(run).filter(is_function);
        for (const key in this.$$.slotted) {
          this.appendChild(this.$$.slotted[key]);
        }
      }
      attributeChangedCallback(attr2, _oldValue, newValue) {
        this[attr2] = newValue;
      }
      disconnectedCallback() {
        run_all(this.$$.on_disconnect);
      }
      $destroy() {
        destroy_component(this, 1);
        this.$destroy = noop;
      }
      $on(type, callback) {
        if (!is_function(callback)) {
          return noop;
        }
        const callbacks = this.$$.callbacks[type] || (this.$$.callbacks[type] = []);
        callbacks.push(callback);
        return () => {
          const index = callbacks.indexOf(callback);
          if (index !== -1)
            callbacks.splice(index, 1);
        };
      }
      $set($$props) {
        if (this.$$set && !is_empty($$props)) {
          this.$$.skip_bound = true;
          this.$$set($$props);
          this.$$.skip_bound = false;
        }
      }
    };
  }
  var SvelteComponent = class {
    $destroy() {
      destroy_component(this, 1);
      this.$destroy = noop;
    }
    $on(type, callback) {
      if (!is_function(callback)) {
        return noop;
      }
      const callbacks = this.$$.callbacks[type] || (this.$$.callbacks[type] = []);
      callbacks.push(callback);
      return () => {
        const index = callbacks.indexOf(callback);
        if (index !== -1)
          callbacks.splice(index, 1);
      };
    }
    $set($$props) {
      if (this.$$set && !is_empty($$props)) {
        this.$$.skip_bound = true;
        this.$$set($$props);
        this.$$.skip_bound = false;
      }
    }
  };

  // node_modules/is-alphabetical/index.js
  function isAlphabetical(character) {
    const code = typeof character === "string" ? character.charCodeAt(0) : character;
    return code >= 97 && code <= 122 || code >= 65 && code <= 90;
  }

  // node_modules/is-decimal/index.js
  function isDecimal(character) {
    const code = typeof character === "string" ? character.charCodeAt(0) : character;
    return code >= 48 && code <= 57;
  }

  // node_modules/is-alphanumerical/index.js
  function isAlphanumerical(character) {
    return isAlphabetical(character) || isDecimal(character);
  }

  // node_modules/bcp-47/lib/regular.js
  var regular = [
    "art-lojban",
    "cel-gaulish",
    "no-bok",
    "no-nyn",
    "zh-guoyu",
    "zh-hakka",
    "zh-min",
    "zh-min-nan",
    "zh-xiang"
  ];

  // node_modules/bcp-47/lib/normal.js
  var normal = {
    "en-gb-oed": "en-GB-oxendict",
    "i-ami": "ami",
    "i-bnn": "bnn",
    "i-default": null,
    "i-enochian": null,
    "i-hak": "hak",
    "i-klingon": "tlh",
    "i-lux": "lb",
    "i-mingo": null,
    "i-navajo": "nv",
    "i-pwn": "pwn",
    "i-tao": "tao",
    "i-tay": "tay",
    "i-tsu": "tsu",
    "sgn-be-fr": "sfb",
    "sgn-be-nl": "vgt",
    "sgn-ch-de": "sgg",
    "art-lojban": "jbo",
    "cel-gaulish": null,
    "no-bok": "nb",
    "no-nyn": "nn",
    "zh-guoyu": "cmn",
    "zh-hakka": "hak",
    "zh-min": null,
    "zh-min-nan": "nan",
    "zh-xiang": "hsn"
  };

  // node_modules/bcp-47/lib/parse.js
  var own = {}.hasOwnProperty;
  function parse(tag, options = {}) {
    const result = empty2();
    const source = String(tag);
    const value = source.toLowerCase();
    let index = 0;
    if (tag === null || tag === void 0) {
      throw new Error("Expected string, got `" + tag + "`");
    }
    if (own.call(normal, value)) {
      const replacement = normal[value];
      if ((options.normalize === void 0 || options.normalize === null || options.normalize) && typeof replacement === "string") {
        return parse(replacement);
      }
      result[regular.includes(value) ? "regular" : "irregular"] = source;
      return result;
    }
    while (isAlphabetical(value.charCodeAt(index)) && index < 9)
      index++;
    if (index > 1 && index < 9) {
      result.language = source.slice(0, index);
      if (index < 4) {
        let groups = 0;
        while (value.charCodeAt(index) === 45 && isAlphabetical(value.charCodeAt(index + 1)) && isAlphabetical(value.charCodeAt(index + 2)) && isAlphabetical(value.charCodeAt(index + 3)) && !isAlphabetical(value.charCodeAt(index + 4))) {
          if (groups > 2) {
            return fail(
              index,
              3,
              "Too many extended language subtags, expected at most 3 subtags"
            );
          }
          result.extendedLanguageSubtags.push(source.slice(index + 1, index + 4));
          index += 4;
          groups++;
        }
      }
      if (value.charCodeAt(index) === 45 && isAlphabetical(value.charCodeAt(index + 1)) && isAlphabetical(value.charCodeAt(index + 2)) && isAlphabetical(value.charCodeAt(index + 3)) && isAlphabetical(value.charCodeAt(index + 4)) && !isAlphabetical(value.charCodeAt(index + 5))) {
        result.script = source.slice(index + 1, index + 5);
        index += 5;
      }
      if (value.charCodeAt(index) === 45) {
        if (isAlphabetical(value.charCodeAt(index + 1)) && isAlphabetical(value.charCodeAt(index + 2)) && !isAlphabetical(value.charCodeAt(index + 3))) {
          result.region = source.slice(index + 1, index + 3);
          index += 3;
        } else if (isDecimal(value.charCodeAt(index + 1)) && isDecimal(value.charCodeAt(index + 2)) && isDecimal(value.charCodeAt(index + 3)) && !isDecimal(value.charCodeAt(index + 4))) {
          result.region = source.slice(index + 1, index + 4);
          index += 4;
        }
      }
      while (value.charCodeAt(index) === 45) {
        const start = index + 1;
        let offset = start;
        while (isAlphanumerical(value.charCodeAt(offset))) {
          if (offset - start > 7) {
            return fail(
              offset,
              1,
              "Too long variant, expected at most 8 characters"
            );
          }
          offset++;
        }
        if (
          // Long variant.
          offset - start > 4 || // Short variant.
          offset - start > 3 && isDecimal(value.charCodeAt(start))
        ) {
          result.variants.push(source.slice(start, offset));
          index = offset;
        } else {
          break;
        }
      }
      while (value.charCodeAt(index) === 45) {
        if (value.charCodeAt(index + 1) === 120 || !isAlphanumerical(value.charCodeAt(index + 1)) || value.charCodeAt(index + 2) !== 45 || !isAlphanumerical(value.charCodeAt(index + 3))) {
          break;
        }
        let offset = index + 2;
        let groups = 0;
        while (value.charCodeAt(offset) === 45 && isAlphanumerical(value.charCodeAt(offset + 1)) && isAlphanumerical(value.charCodeAt(offset + 2))) {
          const start = offset + 1;
          offset = start + 2;
          groups++;
          while (isAlphanumerical(value.charCodeAt(offset))) {
            if (offset - start > 7) {
              return fail(
                offset,
                2,
                "Too long extension, expected at most 8 characters"
              );
            }
            offset++;
          }
        }
        if (!groups) {
          return fail(
            offset,
            4,
            "Empty extension, extensions must have at least 2 characters of content"
          );
        }
        result.extensions.push({
          singleton: source.charAt(index + 1),
          extensions: source.slice(index + 3, offset).split("-")
        });
        index = offset;
      }
    } else {
      index = 0;
    }
    if (index === 0 && value.charCodeAt(index) === 120 || value.charCodeAt(index) === 45 && value.charCodeAt(index + 1) === 120) {
      index = index ? index + 2 : 1;
      let offset = index;
      while (value.charCodeAt(offset) === 45 && isAlphanumerical(value.charCodeAt(offset + 1))) {
        const start = index + 1;
        offset = start;
        while (isAlphanumerical(value.charCodeAt(offset))) {
          if (offset - start > 7) {
            return fail(
              offset,
              5,
              "Too long private-use area, expected at most 8 characters"
            );
          }
          offset++;
        }
        result.privateuse.push(source.slice(index + 1, offset));
        index = offset;
      }
    }
    if (index !== source.length) {
      return fail(index, 6, "Found superfluous content after tag");
    }
    return result;
    function fail(offset, code, reason) {
      if (options.warning)
        options.warning(reason, code, offset);
      return options.forgiving ? result : empty2();
    }
  }
  function empty2() {
    return {
      language: null,
      extendedLanguageSubtags: [],
      script: null,
      region: null,
      variants: [],
      extensions: [],
      privateuse: [],
      irregular: null,
      regular: null
    };
  }

  // svelte/result.svelte
  function get_each_context(ctx, list, i) {
    const child_ctx = ctx.slice();
    child_ctx[8] = list[i][0];
    child_ctx[9] = list[i][1];
    return child_ctx;
  }
  function create_else_block(ctx) {
    let t0;
    let div;
    let p0;
    let t2;
    let p1;
    let if_block = (
      /*show_images*/
      ctx[0] && create_if_block_4(ctx)
    );
    return {
      c() {
        if (if_block)
          if_block.c();
        t0 = space();
        div = element("div");
        p0 = element("p");
        p0.textContent = `${/*placeholder*/
        ctx[3](30)}`;
        t2 = space();
        p1 = element("p");
        p1.textContent = `${/*placeholder*/
        ctx[3](40)}`;
        attr(p0, "class", "pagefind-ui__result-title pagefind-ui__loading svelte-j9e30");
        attr(p1, "class", "pagefind-ui__result-excerpt pagefind-ui__loading svelte-j9e30");
        attr(div, "class", "pagefind-ui__result-inner svelte-j9e30");
      },
      m(target, anchor) {
        if (if_block)
          if_block.m(target, anchor);
        insert(target, t0, anchor);
        insert(target, div, anchor);
        append(div, p0);
        append(div, t2);
        append(div, p1);
      },
      p(ctx2, dirty) {
        if (
          /*show_images*/
          ctx2[0]
        ) {
          if (if_block) {
          } else {
            if_block = create_if_block_4(ctx2);
            if_block.c();
            if_block.m(t0.parentNode, t0);
          }
        } else if (if_block) {
          if_block.d(1);
          if_block = null;
        }
      },
      d(detaching) {
        if (if_block)
          if_block.d(detaching);
        if (detaching)
          detach(t0);
        if (detaching)
          detach(div);
      }
    };
  }
  function create_if_block(ctx) {
    let t0;
    let div;
    let p0;
    let a;
    let raw0_value = (
      /*data*/
      ctx[1].meta?.title + ""
    );
    let a_href_value;
    let t1;
    let p1;
    let raw1_value = (
      /*data*/
      ctx[1].excerpt + ""
    );
    let t2;
    let if_block0 = (
      /*show_images*/
      ctx[0] && create_if_block_2(ctx)
    );
    let if_block1 = (
      /*meta*/
      ctx[2].length && create_if_block_1(ctx)
    );
    return {
      c() {
        if (if_block0)
          if_block0.c();
        t0 = space();
        div = element("div");
        p0 = element("p");
        a = element("a");
        t1 = space();
        p1 = element("p");
        t2 = space();
        if (if_block1)
          if_block1.c();
        attr(a, "class", "pagefind-ui__result-link svelte-j9e30");
        attr(a, "href", a_href_value = /*data*/
        ctx[1].meta?.url || /*data*/
        ctx[1].url);
        attr(p0, "class", "pagefind-ui__result-title svelte-j9e30");
        attr(p1, "class", "pagefind-ui__result-excerpt svelte-j9e30");
        attr(div, "class", "pagefind-ui__result-inner svelte-j9e30");
      },
      m(target, anchor) {
        if (if_block0)
          if_block0.m(target, anchor);
        insert(target, t0, anchor);
        insert(target, div, anchor);
        append(div, p0);
        append(p0, a);
        a.innerHTML = raw0_value;
        append(div, t1);
        append(div, p1);
        p1.innerHTML = raw1_value;
        append(div, t2);
        if (if_block1)
          if_block1.m(div, null);
      },
      p(ctx2, dirty) {
        if (
          /*show_images*/
          ctx2[0]
        ) {
          if (if_block0) {
            if_block0.p(ctx2, dirty);
          } else {
            if_block0 = create_if_block_2(ctx2);
            if_block0.c();
            if_block0.m(t0.parentNode, t0);
          }
        } else if (if_block0) {
          if_block0.d(1);
          if_block0 = null;
        }
        if (dirty & /*data*/
        2 && raw0_value !== (raw0_value = /*data*/
        ctx2[1].meta?.title + ""))
          a.innerHTML = raw0_value;
        ;
        if (dirty & /*data*/
        2 && a_href_value !== (a_href_value = /*data*/
        ctx2[1].meta?.url || /*data*/
        ctx2[1].url)) {
          attr(a, "href", a_href_value);
        }
        if (dirty & /*data*/
        2 && raw1_value !== (raw1_value = /*data*/
        ctx2[1].excerpt + ""))
          p1.innerHTML = raw1_value;
        ;
        if (
          /*meta*/
          ctx2[2].length
        ) {
          if (if_block1) {
            if_block1.p(ctx2, dirty);
          } else {
            if_block1 = create_if_block_1(ctx2);
            if_block1.c();
            if_block1.m(div, null);
          }
        } else if (if_block1) {
          if_block1.d(1);
          if_block1 = null;
        }
      },
      d(detaching) {
        if (if_block0)
          if_block0.d(detaching);
        if (detaching)
          detach(t0);
        if (detaching)
          detach(div);
        if (if_block1)
          if_block1.d();
      }
    };
  }
  function create_if_block_4(ctx) {
    let div;
    return {
      c() {
        div = element("div");
        attr(div, "class", "pagefind-ui__result-thumb pagefind-ui__loading svelte-j9e30");
      },
      m(target, anchor) {
        insert(target, div, anchor);
      },
      d(detaching) {
        if (detaching)
          detach(div);
      }
    };
  }
  function create_if_block_2(ctx) {
    let div;
    let if_block = (
      /*data*/
      ctx[1].meta.image && create_if_block_3(ctx)
    );
    return {
      c() {
        div = element("div");
        if (if_block)
          if_block.c();
        attr(div, "class", "pagefind-ui__result-thumb svelte-j9e30");
      },
      m(target, anchor) {
        insert(target, div, anchor);
        if (if_block)
          if_block.m(div, null);
      },
      p(ctx2, dirty) {
        if (
          /*data*/
          ctx2[1].meta.image
        ) {
          if (if_block) {
            if_block.p(ctx2, dirty);
          } else {
            if_block = create_if_block_3(ctx2);
            if_block.c();
            if_block.m(div, null);
          }
        } else if (if_block) {
          if_block.d(1);
          if_block = null;
        }
      },
      d(detaching) {
        if (detaching)
          detach(div);
        if (if_block)
          if_block.d();
      }
    };
  }
  function create_if_block_3(ctx) {
    let img;
    let img_src_value;
    let img_alt_value;
    return {
      c() {
        img = element("img");
        attr(img, "class", "pagefind-ui__result-image svelte-j9e30");
        if (!src_url_equal(img.src, img_src_value = /*data*/
        ctx[1].meta?.image))
          attr(img, "src", img_src_value);
        attr(img, "alt", img_alt_value = /*data*/
        ctx[1].meta?.image_alt || /*data*/
        ctx[1].meta?.title);
      },
      m(target, anchor) {
        insert(target, img, anchor);
      },
      p(ctx2, dirty) {
        if (dirty & /*data*/
        2 && !src_url_equal(img.src, img_src_value = /*data*/
        ctx2[1].meta?.image)) {
          attr(img, "src", img_src_value);
        }
        if (dirty & /*data*/
        2 && img_alt_value !== (img_alt_value = /*data*/
        ctx2[1].meta?.image_alt || /*data*/
        ctx2[1].meta?.title)) {
          attr(img, "alt", img_alt_value);
        }
      },
      d(detaching) {
        if (detaching)
          detach(img);
      }
    };
  }
  function create_if_block_1(ctx) {
    let ul;
    let each_value = (
      /*meta*/
      ctx[2]
    );
    let each_blocks = [];
    for (let i = 0; i < each_value.length; i += 1) {
      each_blocks[i] = create_each_block(get_each_context(ctx, each_value, i));
    }
    return {
      c() {
        ul = element("ul");
        for (let i = 0; i < each_blocks.length; i += 1) {
          each_blocks[i].c();
        }
        attr(ul, "class", "pagefind-ui__result-tags svelte-j9e30");
      },
      m(target, anchor) {
        insert(target, ul, anchor);
        for (let i = 0; i < each_blocks.length; i += 1) {
          each_blocks[i].m(ul, null);
        }
      },
      p(ctx2, dirty) {
        if (dirty & /*meta*/
        4) {
          each_value = /*meta*/
          ctx2[2];
          let i;
          for (i = 0; i < each_value.length; i += 1) {
            const child_ctx = get_each_context(ctx2, each_value, i);
            if (each_blocks[i]) {
              each_blocks[i].p(child_ctx, dirty);
            } else {
              each_blocks[i] = create_each_block(child_ctx);
              each_blocks[i].c();
              each_blocks[i].m(ul, null);
            }
          }
          for (; i < each_blocks.length; i += 1) {
            each_blocks[i].d(1);
          }
          each_blocks.length = each_value.length;
        }
      },
      d(detaching) {
        if (detaching)
          detach(ul);
        destroy_each(each_blocks, detaching);
      }
    };
  }
  function create_each_block(ctx) {
    let li;
    let html_tag;
    let raw0_value = (
      /*metaTitle*/
      ctx[8].replace(/^(\w)/, func) + ""
    );
    let t0;
    let html_tag_1;
    let raw1_value = (
      /*metaValue*/
      ctx[9] + ""
    );
    let t1;
    return {
      c() {
        li = element("li");
        html_tag = new HtmlTag(false);
        t0 = text(": ");
        html_tag_1 = new HtmlTag(false);
        t1 = space();
        html_tag.a = t0;
        html_tag_1.a = t1;
        attr(li, "class", "pagefind-ui__result-tag svelte-j9e30");
      },
      m(target, anchor) {
        insert(target, li, anchor);
        html_tag.m(raw0_value, li);
        append(li, t0);
        html_tag_1.m(raw1_value, li);
        append(li, t1);
      },
      p(ctx2, dirty) {
        if (dirty & /*meta*/
        4 && raw0_value !== (raw0_value = /*metaTitle*/
        ctx2[8].replace(/^(\w)/, func) + ""))
          html_tag.p(raw0_value);
        if (dirty & /*meta*/
        4 && raw1_value !== (raw1_value = /*metaValue*/
        ctx2[9] + ""))
          html_tag_1.p(raw1_value);
      },
      d(detaching) {
        if (detaching)
          detach(li);
      }
    };
  }
  function create_fragment(ctx) {
    let li;
    function select_block_type(ctx2, dirty) {
      if (
        /*data*/
        ctx2[1]
      )
        return create_if_block;
      return create_else_block;
    }
    let current_block_type = select_block_type(ctx, -1);
    let if_block = current_block_type(ctx);
    return {
      c() {
        li = element("li");
        if_block.c();
        attr(li, "class", "pagefind-ui__result svelte-j9e30");
      },
      m(target, anchor) {
        insert(target, li, anchor);
        if_block.m(li, null);
      },
      p(ctx2, [dirty]) {
        if (current_block_type === (current_block_type = select_block_type(ctx2, dirty)) && if_block) {
          if_block.p(ctx2, dirty);
        } else {
          if_block.d(1);
          if_block = current_block_type(ctx2);
          if (if_block) {
            if_block.c();
            if_block.m(li, null);
          }
        }
      },
      i: noop,
      o: noop,
      d(detaching) {
        if (detaching)
          detach(li);
        if_block.d();
      }
    };
  }
  var func = (c) => c.toLocaleUpperCase();
  function instance($$self, $$props, $$invalidate) {
    let { show_images = true } = $$props;
    let { process_result = null } = $$props;
    let { result = {
      data: async () => {
      }
    } } = $$props;
    const skipMeta = ["title", "image", "image_alt", "url"];
    let data;
    let meta = [];
    const load = async (r) => {
      $$invalidate(1, data = await r.data());
      $$invalidate(1, data = process_result?.(data) ?? data);
      $$invalidate(2, meta = Object.entries(data.meta).filter(([key]) => !skipMeta.includes(key)));
    };
    const placeholder = (max = 30) => {
      return ". ".repeat(Math.floor(10 + Math.random() * max));
    };
    $$self.$$set = ($$props2) => {
      if ("show_images" in $$props2)
        $$invalidate(0, show_images = $$props2.show_images);
      if ("process_result" in $$props2)
        $$invalidate(4, process_result = $$props2.process_result);
      if ("result" in $$props2)
        $$invalidate(5, result = $$props2.result);
    };
    $$self.$$.update = () => {
      if ($$self.$$.dirty & /*result*/
      32) {
        $:
          load(result);
      }
    };
    return [show_images, data, meta, placeholder, process_result, result];
  }
  var Result = class extends SvelteComponent {
    constructor(options) {
      super();
      init(this, options, instance, create_fragment, safe_not_equal, {
        show_images: 0,
        process_result: 4,
        result: 5
      });
    }
  };
  var result_default = Result;

  // svelte/filters.svelte
  function get_each_context2(ctx, list, i) {
    const child_ctx = ctx.slice();
    child_ctx[7] = list[i][0];
    child_ctx[8] = list[i][1];
    child_ctx[9] = list;
    child_ctx[10] = i;
    return child_ctx;
  }
  function get_each_context_1(ctx, list, i) {
    const child_ctx = ctx.slice();
    child_ctx[11] = list[i][0];
    child_ctx[12] = list[i][1];
    child_ctx[13] = list;
    child_ctx[14] = i;
    return child_ctx;
  }
  function create_if_block2(ctx) {
    let fieldset;
    let legend;
    let t0_value = (
      /*translate*/
      ctx[3]("filters_label") + ""
    );
    let t0;
    let t1;
    let each_value = Object.entries(
      /*available_filters*/
      ctx[1]
    );
    let each_blocks = [];
    for (let i = 0; i < each_value.length; i += 1) {
      each_blocks[i] = create_each_block2(get_each_context2(ctx, each_value, i));
    }
    return {
      c() {
        fieldset = element("fieldset");
        legend = element("legend");
        t0 = text(t0_value);
        t1 = space();
        for (let i = 0; i < each_blocks.length; i += 1) {
          each_blocks[i].c();
        }
        attr(legend, "class", "pagefind-ui__filter-panel-label svelte-1v2r7ls");
        attr(fieldset, "class", "pagefind-ui__filter-panel svelte-1v2r7ls");
      },
      m(target, anchor) {
        insert(target, fieldset, anchor);
        append(fieldset, legend);
        append(legend, t0);
        append(fieldset, t1);
        for (let i = 0; i < each_blocks.length; i += 1) {
          each_blocks[i].m(fieldset, null);
        }
      },
      p(ctx2, dirty) {
        if (dirty & /*translate*/
        8 && t0_value !== (t0_value = /*translate*/
        ctx2[3]("filters_label") + ""))
          set_data(t0, t0_value);
        if (dirty & /*default_open, Object, available_filters, selected_filters, show_empty_filters*/
        23) {
          each_value = Object.entries(
            /*available_filters*/
            ctx2[1]
          );
          let i;
          for (i = 0; i < each_value.length; i += 1) {
            const child_ctx = get_each_context2(ctx2, each_value, i);
            if (each_blocks[i]) {
              each_blocks[i].p(child_ctx, dirty);
            } else {
              each_blocks[i] = create_each_block2(child_ctx);
              each_blocks[i].c();
              each_blocks[i].m(fieldset, null);
            }
          }
          for (; i < each_blocks.length; i += 1) {
            each_blocks[i].d(1);
          }
          each_blocks.length = each_value.length;
        }
      },
      d(detaching) {
        if (detaching)
          detach(fieldset);
        destroy_each(each_blocks, detaching);
      }
    };
  }
  function create_if_block_12(ctx) {
    let div;
    let input;
    let input_id_value;
    let input_name_value;
    let input_value_value;
    let t0;
    let label;
    let html_tag;
    let raw_value = (
      /*value*/
      ctx[11] + ""
    );
    let t1;
    let t2_value = (
      /*count*/
      ctx[12] + ""
    );
    let t2;
    let t3;
    let label_for_value;
    let t4;
    let mounted;
    let dispose;
    function input_change_handler() {
      ctx[6].call(
        input,
        /*filter*/
        ctx[7],
        /*value*/
        ctx[11]
      );
    }
    return {
      c() {
        div = element("div");
        input = element("input");
        t0 = space();
        label = element("label");
        html_tag = new HtmlTag(false);
        t1 = text(" (");
        t2 = text(t2_value);
        t3 = text(")");
        t4 = space();
        attr(input, "class", "pagefind-ui__filter-checkbox svelte-1v2r7ls");
        attr(input, "type", "checkbox");
        attr(input, "id", input_id_value = /*filter*/
        ctx[7] + "-" + /*value*/
        ctx[11]);
        attr(input, "name", input_name_value = /*filter*/
        ctx[7]);
        input.__value = input_value_value = /*value*/
        ctx[11];
        input.value = input.__value;
        html_tag.a = t1;
        attr(label, "class", "pagefind-ui__filter-label svelte-1v2r7ls");
        attr(label, "for", label_for_value = /*filter*/
        ctx[7] + "-" + /*value*/
        ctx[11]);
        attr(div, "class", "pagefind-ui__filter-value svelte-1v2r7ls");
        toggle_class(
          div,
          "pagefind-ui__filter-value--checked",
          /*selected_filters*/
          ctx[0][`${/*filter*/
          ctx[7]}:${/*value*/
          ctx[11]}`]
        );
      },
      m(target, anchor) {
        insert(target, div, anchor);
        append(div, input);
        input.checked = /*selected_filters*/
        ctx[0][`${/*filter*/
        ctx[7]}:${/*value*/
        ctx[11]}`];
        append(div, t0);
        append(div, label);
        html_tag.m(raw_value, label);
        append(label, t1);
        append(label, t2);
        append(label, t3);
        append(div, t4);
        if (!mounted) {
          dispose = listen(input, "change", input_change_handler);
          mounted = true;
        }
      },
      p(new_ctx, dirty) {
        ctx = new_ctx;
        if (dirty & /*available_filters*/
        2 && input_id_value !== (input_id_value = /*filter*/
        ctx[7] + "-" + /*value*/
        ctx[11])) {
          attr(input, "id", input_id_value);
        }
        if (dirty & /*available_filters*/
        2 && input_name_value !== (input_name_value = /*filter*/
        ctx[7])) {
          attr(input, "name", input_name_value);
        }
        if (dirty & /*available_filters*/
        2 && input_value_value !== (input_value_value = /*value*/
        ctx[11])) {
          input.__value = input_value_value;
          input.value = input.__value;
        }
        if (dirty & /*selected_filters, Object, available_filters*/
        3) {
          input.checked = /*selected_filters*/
          ctx[0][`${/*filter*/
          ctx[7]}:${/*value*/
          ctx[11]}`];
        }
        if (dirty & /*available_filters*/
        2 && raw_value !== (raw_value = /*value*/
        ctx[11] + ""))
          html_tag.p(raw_value);
        if (dirty & /*available_filters*/
        2 && t2_value !== (t2_value = /*count*/
        ctx[12] + ""))
          set_data(t2, t2_value);
        if (dirty & /*available_filters*/
        2 && label_for_value !== (label_for_value = /*filter*/
        ctx[7] + "-" + /*value*/
        ctx[11])) {
          attr(label, "for", label_for_value);
        }
        if (dirty & /*selected_filters, Object, available_filters*/
        3) {
          toggle_class(
            div,
            "pagefind-ui__filter-value--checked",
            /*selected_filters*/
            ctx[0][`${/*filter*/
            ctx[7]}:${/*value*/
            ctx[11]}`]
          );
        }
      },
      d(detaching) {
        if (detaching)
          detach(div);
        mounted = false;
        dispose();
      }
    };
  }
  function create_each_block_1(ctx) {
    let if_block_anchor;
    let if_block = (
      /*show_empty_filters*/
      (ctx[2] || /*count*/
      ctx[12] || /*selected_filters*/
      ctx[0][`${/*filter*/
      ctx[7]}:${/*value*/
      ctx[11]}`]) && create_if_block_12(ctx)
    );
    return {
      c() {
        if (if_block)
          if_block.c();
        if_block_anchor = empty();
      },
      m(target, anchor) {
        if (if_block)
          if_block.m(target, anchor);
        insert(target, if_block_anchor, anchor);
      },
      p(ctx2, dirty) {
        if (
          /*show_empty_filters*/
          ctx2[2] || /*count*/
          ctx2[12] || /*selected_filters*/
          ctx2[0][`${/*filter*/
          ctx2[7]}:${/*value*/
          ctx2[11]}`]
        ) {
          if (if_block) {
            if_block.p(ctx2, dirty);
          } else {
            if_block = create_if_block_12(ctx2);
            if_block.c();
            if_block.m(if_block_anchor.parentNode, if_block_anchor);
          }
        } else if (if_block) {
          if_block.d(1);
          if_block = null;
        }
      },
      d(detaching) {
        if (if_block)
          if_block.d(detaching);
        if (detaching)
          detach(if_block_anchor);
      }
    };
  }
  function create_each_block2(ctx) {
    let details;
    let summary;
    let raw0_value = (
      /*filter*/
      ctx[7].replace(/^(\w)/, func2) + ""
    );
    let t0;
    let fieldset;
    let legend;
    let raw1_value = (
      /*filter*/
      ctx[7] + ""
    );
    let t1;
    let t2;
    let each_value_1 = Object.entries(
      /*values*/
      ctx[8] || {}
    );
    let each_blocks = [];
    for (let i = 0; i < each_value_1.length; i += 1) {
      each_blocks[i] = create_each_block_1(get_each_context_1(ctx, each_value_1, i));
    }
    return {
      c() {
        details = element("details");
        summary = element("summary");
        t0 = space();
        fieldset = element("fieldset");
        legend = element("legend");
        t1 = space();
        for (let i = 0; i < each_blocks.length; i += 1) {
          each_blocks[i].c();
        }
        t2 = space();
        attr(summary, "class", "pagefind-ui__filter-name svelte-1v2r7ls");
        attr(legend, "class", "pagefind-ui__filter-group-label svelte-1v2r7ls");
        attr(fieldset, "class", "pagefind-ui__filter-group svelte-1v2r7ls");
        attr(details, "class", "pagefind-ui__filter-block svelte-1v2r7ls");
        details.open = /*default_open*/
        ctx[4];
      },
      m(target, anchor) {
        insert(target, details, anchor);
        append(details, summary);
        summary.innerHTML = raw0_value;
        append(details, t0);
        append(details, fieldset);
        append(fieldset, legend);
        legend.innerHTML = raw1_value;
        append(fieldset, t1);
        for (let i = 0; i < each_blocks.length; i += 1) {
          each_blocks[i].m(fieldset, null);
        }
        append(details, t2);
      },
      p(ctx2, dirty) {
        if (dirty & /*available_filters*/
        2 && raw0_value !== (raw0_value = /*filter*/
        ctx2[7].replace(/^(\w)/, func2) + ""))
          summary.innerHTML = raw0_value;
        ;
        if (dirty & /*available_filters*/
        2 && raw1_value !== (raw1_value = /*filter*/
        ctx2[7] + ""))
          legend.innerHTML = raw1_value;
        ;
        if (dirty & /*selected_filters, Object, available_filters, show_empty_filters*/
        7) {
          each_value_1 = Object.entries(
            /*values*/
            ctx2[8] || {}
          );
          let i;
          for (i = 0; i < each_value_1.length; i += 1) {
            const child_ctx = get_each_context_1(ctx2, each_value_1, i);
            if (each_blocks[i]) {
              each_blocks[i].p(child_ctx, dirty);
            } else {
              each_blocks[i] = create_each_block_1(child_ctx);
              each_blocks[i].c();
              each_blocks[i].m(fieldset, null);
            }
          }
          for (; i < each_blocks.length; i += 1) {
            each_blocks[i].d(1);
          }
          each_blocks.length = each_value_1.length;
        }
        if (dirty & /*default_open*/
        16) {
          details.open = /*default_open*/
          ctx2[4];
        }
      },
      d(detaching) {
        if (detaching)
          detach(details);
        destroy_each(each_blocks, detaching);
      }
    };
  }
  function create_fragment2(ctx) {
    let show_if = (
      /*available_filters*/
      ctx[1] && Object.entries(
        /*available_filters*/
        ctx[1]
      ).length
    );
    let if_block_anchor;
    let if_block = show_if && create_if_block2(ctx);
    return {
      c() {
        if (if_block)
          if_block.c();
        if_block_anchor = empty();
      },
      m(target, anchor) {
        if (if_block)
          if_block.m(target, anchor);
        insert(target, if_block_anchor, anchor);
      },
      p(ctx2, [dirty]) {
        if (dirty & /*available_filters*/
        2)
          show_if = /*available_filters*/
          ctx2[1] && Object.entries(
            /*available_filters*/
            ctx2[1]
          ).length;
        if (show_if) {
          if (if_block) {
            if_block.p(ctx2, dirty);
          } else {
            if_block = create_if_block2(ctx2);
            if_block.c();
            if_block.m(if_block_anchor.parentNode, if_block_anchor);
          }
        } else if (if_block) {
          if_block.d(1);
          if_block = null;
        }
      },
      i: noop,
      o: noop,
      d(detaching) {
        if (if_block)
          if_block.d(detaching);
        if (detaching)
          detach(if_block_anchor);
      }
    };
  }
  var func2 = (c) => c.toLocaleUpperCase();
  function instance2($$self, $$props, $$invalidate) {
    let { available_filters = null } = $$props;
    let { show_empty_filters = true } = $$props;
    let { translate = () => "" } = $$props;
    const selected_filters = {};
    let initialized = false;
    let default_open = false;
    function input_change_handler(filter, value) {
      selected_filters[`${filter}:${value}`] = this.checked;
      $$invalidate(0, selected_filters);
    }
    $$self.$$set = ($$props2) => {
      if ("available_filters" in $$props2)
        $$invalidate(1, available_filters = $$props2.available_filters);
      if ("show_empty_filters" in $$props2)
        $$invalidate(2, show_empty_filters = $$props2.show_empty_filters);
      if ("translate" in $$props2)
        $$invalidate(3, translate = $$props2.translate);
    };
    $$self.$$.update = () => {
      if ($$self.$$.dirty & /*available_filters, initialized*/
      34) {
        $:
          if (available_filters && !initialized) {
            $$invalidate(5, initialized = true);
            let filters = Object.entries(available_filters || {});
            if (filters.length === 1) {
              let values = Object.entries(filters[0][1]);
              if (values?.length <= 6) {
                $$invalidate(4, default_open = true);
              }
            }
          }
      }
    };
    return [
      selected_filters,
      available_filters,
      show_empty_filters,
      translate,
      default_open,
      initialized,
      input_change_handler
    ];
  }
  var Filters = class extends SvelteComponent {
    constructor(options) {
      super();
      init(this, options, instance2, create_fragment2, safe_not_equal, {
        available_filters: 1,
        show_empty_filters: 2,
        translate: 3,
        selected_filters: 0
      });
    }
    get selected_filters() {
      return this.$$.ctx[0];
    }
  };
  var filters_default = Filters;

  // translations/af.json
  var af_exports = {};
  __export(af_exports, {
    comments: () => comments,
    default: () => af_default,
    direction: () => direction,
    strings: () => strings,
    thanks_to: () => thanks_to
  });
  var thanks_to = "Jan Claasen";
  var comments = "";
  var direction = "ltr";
  var strings = {
    placeholder: "Soek",
    clear_search: "Opruim",
    load_more: "Laai nog resultate",
    search_label: "Soek hierdie webwerf",
    filters_label: "Filters",
    zero_results: "Geen resultate vir [SEARCH_TERM]",
    many_results: "[COUNT] resultate vir [SEARCH_TERM]",
    one_result: "[COUNT] resultate vir [SEARCH_TERM]",
    alt_search: "Geen resultate vir [SEARCH_TERM]. Toon resultate vir [DIFFERENT_TERM] in plaas daarvan",
    search_suggestion: "Geen resultate vir [SEARCH_TERM]. Probeer eerder een van die volgende terme:",
    searching: "Soek vir [SEARCH_TERM]"
  };
  var af_default = {
    thanks_to,
    comments,
    direction,
    strings
  };

  // translations/ca.json
  var ca_exports = {};
  __export(ca_exports, {
    comments: () => comments2,
    default: () => ca_default,
    direction: () => direction2,
    strings: () => strings2,
    thanks_to: () => thanks_to2
  });
  var thanks_to2 = "Pablo Villaverde <https://github.com/pvillaverde>";
  var comments2 = "";
  var direction2 = "ltr";
  var strings2 = {
    placeholder: "Cerca",
    clear_search: "Netejar",
    load_more: "Veure m\xE9es resultats",
    search_label: "Cerca en aquest lloc",
    filters_label: "Filtres",
    zero_results: "No es van trobar resultats per [SEARCH_TERM]",
    many_results: "[COUNT] resultats trobats per [SEARCH_TERM]",
    one_result: "[COUNT] resultat trobat per [SEARCH_TERM]",
    alt_search: "No es van trobar resultats per [SEARCH_TERM]. Mostrant al seu lloc resultats per [DIFFERENT_TERM]",
    search_suggestion: "No es van trobar resultats per [SEARCH_TERM]. Proveu una de les cerques seg\xFCents:",
    searching: "Cercant [SEARCH_TERM]..."
  };
  var ca_default = {
    thanks_to: thanks_to2,
    comments: comments2,
    direction: direction2,
    strings: strings2
  };

  // translations/de.json
  var de_exports = {};
  __export(de_exports, {
    comments: () => comments3,
    default: () => de_default,
    direction: () => direction3,
    strings: () => strings3,
    thanks_to: () => thanks_to3
  });
  var thanks_to3 = "Jan Claasen";
  var comments3 = "";
  var direction3 = "ltr";
  var strings3 = {
    placeholder: "Suche",
    clear_search: "L\xF6schen",
    load_more: "Mehr Ergebnisse laden",
    search_label: "Suche diese Seite",
    filters_label: "Filter",
    zero_results: "Keine Ergebnisse f\xFCr [SEARCH_TERM]",
    many_results: "[COUNT] Ergebnisse f\xFCr [SEARCH_TERM]",
    one_result: "[COUNT] Ergebnis f\xFCr [SEARCH_TERM]",
    alt_search: "Keine Ergebnisse f\xFCr [SEARCH_TERM]. Stattdessen werden Ergebnisse f\xFCr [DIFFERENT_TERM] angezeigt",
    search_suggestion: "Keine Ergebnisse f\xFCr [SEARCH_TERM]. Versuchen Sie eine der folgenden Suchen:",
    searching: "Suche f\xFCr [SEARCH_TERM]"
  };
  var de_default = {
    thanks_to: thanks_to3,
    comments: comments3,
    direction: direction3,
    strings: strings3
  };

  // translations/en.json
  var en_exports = {};
  __export(en_exports, {
    comments: () => comments4,
    default: () => en_default,
    direction: () => direction4,
    strings: () => strings4,
    thanks_to: () => thanks_to4
  });
  var thanks_to4 = "Liam Bigelow <liam@cloudcannon.com>";
  var comments4 = "";
  var direction4 = "ltr";
  var strings4 = {
    placeholder: "Search",
    clear_search: "Clear",
    load_more: "Load more results",
    search_label: "Search this site",
    filters_label: "Filters",
    zero_results: "No results for [SEARCH_TERM]",
    many_results: "[COUNT] results for [SEARCH_TERM]",
    one_result: "[COUNT] result for [SEARCH_TERM]",
    alt_search: "No results for [SEARCH_TERM]. Showing results for [DIFFERENT_TERM] instead",
    search_suggestion: "No results for [SEARCH_TERM]. Try one of the following searches:",
    searching: "Searching for [SEARCH_TERM]..."
  };
  var en_default = {
    thanks_to: thanks_to4,
    comments: comments4,
    direction: direction4,
    strings: strings4
  };

  // translations/es.json
  var es_exports = {};
  __export(es_exports, {
    comments: () => comments5,
    default: () => es_default,
    direction: () => direction5,
    strings: () => strings5,
    thanks_to: () => thanks_to5
  });
  var thanks_to5 = "Pablo Villaverde <https://github.com/pvillaverde>";
  var comments5 = "";
  var direction5 = "ltr";
  var strings5 = {
    placeholder: "Buscar",
    clear_search: "Limpiar",
    load_more: "Ver m\xE1s resultados",
    search_label: "Buscar en este sitio",
    filters_label: "Filtros",
    zero_results: "No se encontraron resultados para [SEARCH_TERM]",
    many_results: "[COUNT] resultados encontrados para [SEARCH_TERM]",
    one_result: "[COUNT] resultado encontrado para [SEARCH_TERM]",
    alt_search: "No se encontraron resultados para [SEARCH_TERM]. Mostrando en su lugar resultados para [DIFFERENT_TERM]",
    search_suggestion: "No se encontraron resultados para [SEARCH_TERM]. Prueba una de las siguientes b\xFAsquedas:",
    searching: "Buscando [SEARCH_TERM]..."
  };
  var es_default = {
    thanks_to: thanks_to5,
    comments: comments5,
    direction: direction5,
    strings: strings5
  };

  // translations/fr.json
  var fr_exports = {};
  __export(fr_exports, {
    comments: () => comments6,
    default: () => fr_default,
    direction: () => direction6,
    strings: () => strings6,
    thanks_to: () => thanks_to6
  });
  var thanks_to6 = "Nicolas Friedli <nicolas@theologique.ch>";
  var comments6 = "";
  var direction6 = "ltr";
  var strings6 = {
    placeholder: "Rechercher",
    clear_search: "Nettoyer",
    load_more: "Charger plus de r\xE9sultats",
    search_label: "Recherche sur ce site",
    filters_label: "Filtres",
    zero_results: "Pas de r\xE9sultat pour [SEARCH_TERM]",
    many_results: "[COUNT] r\xE9sultats pour [SEARCH_TERM]",
    one_result: "[COUNT] r\xE9sultat pour [SEARCH_TERM]",
    alt_search: "Pas de r\xE9sultat pour [SEARCH_TERM]. Montre les r\xE9sultats pour [DIFFERENT_TERM] \xE0 la place",
    search_suggestion: "Pas de r\xE9sultat pour [SEARCH_TERM]. Essayer une des recherches suivantes:",
    searching: "Recherche [SEARCH_TERM]..."
  };
  var fr_default = {
    thanks_to: thanks_to6,
    comments: comments6,
    direction: direction6,
    strings: strings6
  };

  // translations/gl.json
  var gl_exports = {};
  __export(gl_exports, {
    comments: () => comments7,
    default: () => gl_default,
    direction: () => direction7,
    strings: () => strings7,
    thanks_to: () => thanks_to7
  });
  var thanks_to7 = "Pablo Villaverde <https://github.com/pvillaverde>";
  var comments7 = "";
  var direction7 = "ltr";
  var strings7 = {
    placeholder: "Buscar",
    clear_search: "Limpar",
    load_more: "Ver m\xE1is resultados",
    search_label: "Buscar neste sitio",
    filters_label: "Filtros",
    zero_results: "Non se atoparon resultados para [SEARCH_TERM]",
    many_results: "[COUNT] resultados atopados para [SEARCH_TERM]",
    one_result: "[COUNT] resultado atopado para [SEARCH_TERM]",
    alt_search: "Non se atoparon resultados para [SEARCH_TERM]. Amosando no seu lugar resultados para [DIFFERENT_TERM]",
    search_suggestion: "Non se atoparon resultados para [SEARCH_TERM]. Probe unha das seguintes pesquisas:",
    searching: "Buscando [SEARCH_TERM]..."
  };
  var gl_default = {
    thanks_to: thanks_to7,
    comments: comments7,
    direction: direction7,
    strings: strings7
  };

  // translations/ja.json
  var ja_exports = {};
  __export(ja_exports, {
    comments: () => comments8,
    default: () => ja_default,
    direction: () => direction8,
    strings: () => strings8,
    thanks_to: () => thanks_to8
  });
  var thanks_to8 = "Tate";
  var comments8 = "";
  var direction8 = "ltr";
  var strings8 = {
    placeholder: "\u691C\u7D22",
    clear_search: "\u6D88\u3059",
    load_more: "\u3082\u3063\u3068\u8AAD\u307F\u8FBC\u3080",
    search_label: "\u3053\u306E\u30B5\u30A4\u30C8\u3092\u691C\u7D22",
    filters_label: "\u30D5\u30A3\u30EB\u30BF",
    zero_results: "[SEARCH_TERM]\u306E\u691C\u7D22\u306B\u4E00\u81F4\u3059\u308B\u4EF6\u306F\u3042\u308A\u307E\u305B\u3093\u3067\u3057\u305F",
    many_results: "[SEARCH_TERM]\u306E[COUNT]\u4EF6\u306E\u691C\u7D22\u7D50\u679C",
    one_result: "[SEARCH_TERM]\u306E[COUNT]\u4EF6\u306E\u691C\u7D22\u7D50\u679C",
    alt_search: "[SEARCH_TERM]\u306E\u691C\u7D22\u306B\u4E00\u81F4\u3059\u308B\u4EF6\u306F\u3042\u308A\u307E\u305B\u3093\u3067\u3057\u305F\u3002[DIFFERENT_TERM]\u306E\u691C\u7D22\u7D50\u679C\u3092\u8868\u793A\u3057\u3066\u3044\u307E\u3059",
    search_suggestion: "[SEARCH_TERM]\u306E\u691C\u7D22\u306B\u4E00\u81F4\u3059\u308B\u4EF6\u306F\u3042\u308A\u307E\u305B\u3093\u3067\u3057\u305F\u3002\u6B21\u306E\u3044\u305A\u308C\u304B\u306E\u691C\u7D22\u3092\u8A66\u3057\u3066\u304F\u3060\u3055\u3044",
    searching: "[SEARCH_TERM]\u3092\u691C\u7D22\u3057\u3066\u3044\u307E\u3059"
  };
  var ja_default = {
    thanks_to: thanks_to8,
    comments: comments8,
    direction: direction8,
    strings: strings8
  };

  // translations/nl.json
  var nl_exports = {};
  __export(nl_exports, {
    comments: () => comments9,
    default: () => nl_default,
    direction: () => direction9,
    strings: () => strings9,
    thanks_to: () => thanks_to9
  });
  var thanks_to9 = "Paul van Brouwershaven";
  var comments9 = "";
  var direction9 = "ltr";
  var strings9 = {
    placeholder: "Zoeken",
    clear_search: "Reset",
    load_more: "Meer resultaten laden",
    search_label: "Doorzoek deze site",
    filters_label: "Filters",
    zero_results: "Geen resultaten voor [SEARCH_TERM]",
    many_results: "[COUNT] resultaten voor [SEARCH_TERM]",
    one_result: "[COUNT] resultaat voor [SEARCH_TERM]",
    alt_search: "Geen resultaten voor [SEARCH_TERM]. In plaats daarvan worden resultaten voor [DIFFERENT_TERM] weergegeven",
    search_suggestion: "Geen resultaten voor [SEARCH_TERM]. Probeer een van de volgende zoekopdrachten:",
    searching: "Zoeken naar [SEARCH_TERM]..."
  };
  var nl_default = {
    thanks_to: thanks_to9,
    comments: comments9,
    direction: direction9,
    strings: strings9
  };

  // translations/no.json
  var no_exports = {};
  __export(no_exports, {
    comments: () => comments10,
    default: () => no_default,
    direction: () => direction10,
    strings: () => strings10,
    thanks_to: () => thanks_to10
  });
  var thanks_to10 = "Christopher Wingate";
  var comments10 = "";
  var direction10 = "ltr";
  var strings10 = {
    placeholder: "S\xF8k",
    clear_search: "Fjern",
    load_more: "Last flere resultater",
    search_label: "S\xF8k p\xE5 denne siden",
    filters_label: "Filtre",
    zero_results: "Ingen resultater for [SEARCH_TERM]",
    many_results: "[COUNT] resultater for [SEARCH_TERM]",
    one_result: "[COUNT] resultat for [SEARCH_TERM]",
    alt_search: "Ingen resultater for [SEARCH_TERM]. Viser resultater for [DIFFERENT_TERM] i stedet",
    search_suggestion: "Ingen resultater for [SEARCH_TERM]. Pr\xF8v en av disse s\xF8keordene i stedet:",
    searching: "S\xF8ker etter [SEARCH_TERM]"
  };
  var no_default = {
    thanks_to: thanks_to10,
    comments: comments10,
    direction: direction10,
    strings: strings10
  };

  // translations/pt.json
  var pt_exports = {};
  __export(pt_exports, {
    comments: () => comments11,
    default: () => pt_default,
    direction: () => direction11,
    strings: () => strings11,
    thanks_to: () => thanks_to11
  });
  var thanks_to11 = "Jonatah";
  var comments11 = "";
  var direction11 = "ltr";
  var strings11 = {
    placeholder: "Pesquisar",
    clear_search: "Limpar",
    load_more: "Ver mais resultados",
    search_label: "Pesquisar",
    filters_label: "Filtros",
    zero_results: "Nenhum resultado encontrado para [SEARCH_TERM]",
    many_results: "[COUNT] resultados encontrados para [SEARCH_TERM]",
    one_result: "[COUNT] resultado encontrado para [SEARCH_TERM]",
    alt_search: "Nenhum resultado encontrado para [SEARCH_TERM]. Exibindo resultados para [DIFFERENT_TERM]",
    search_suggestion: "Nenhum resultado encontrado para [SEARCH_TERM]. Tente uma das seguintes pesquisas:",
    searching: "Pesquisando por [SEARCH_TERM]..."
  };
  var pt_default = {
    thanks_to: thanks_to11,
    comments: comments11,
    direction: direction11,
    strings: strings11
  };

  // translations/ru.json
  var ru_exports = {};
  __export(ru_exports, {
    comments: () => comments12,
    default: () => ru_default,
    direction: () => direction12,
    strings: () => strings12,
    thanks_to: () => thanks_to12
  });
  var thanks_to12 = "Aleksandr Gordeev";
  var comments12 = "";
  var direction12 = "ltr";
  var strings12 = {
    placeholder: "\u041F\u043E\u0438\u0441\u043A",
    clear_search: "\u041E\u0447\u0438\u0441\u0442\u0438\u0442\u044C \u043F\u043E\u043B\u0435",
    load_more: "\u0417\u0430\u0433\u0440\u0443\u0437\u0438\u0442\u044C \u0435\u0449\u0435",
    search_label: "\u041F\u043E\u0438\u0441\u043A \u043F\u043E \u0441\u0430\u0439\u0442\u0443",
    filters_label: "\u0424\u0438\u043B\u044C\u0442\u0440\u044B",
    zero_results: "\u041D\u0438\u0447\u0435\u0433\u043E \u043D\u0435 \u043D\u0430\u0439\u0434\u0435\u043D\u043E \u043F\u043E \u0437\u0430\u043F\u0440\u043E\u0441\u0443: [SEARCH_TERM]",
    many_results: "[COUNT] \u0440\u0435\u0437\u0443\u043B\u044C\u0442\u0430\u0442\u043E\u0432 \u043F\u043E \u0437\u0430\u043F\u0440\u043E\u0441\u0443: [SEARCH_TERM]",
    one_result: "[COUNT] \u0440\u0435\u0437\u0443\u043B\u044C\u0442\u0430\u0442 \u043F\u043E \u0437\u0430\u043F\u0440\u043E\u0441\u0443: [SEARCH_TERM]",
    alt_search: "\u041D\u0438\u0447\u0435\u0433\u043E \u043D\u0435 \u043D\u0430\u0439\u0434\u0435\u043D\u043E \u043F\u043E \u0437\u0430\u043F\u0440\u043E\u0441\u0443: [SEARCH_TERM]. \u041F\u043E\u043A\u0430\u0437\u0430\u043D\u044B \u0440\u0435\u0437\u0443\u043B\u044C\u0442\u0430\u0442\u044B \u043F\u043E \u0437\u0430\u043F\u0440\u043E\u0441\u0443: [DIFFERENT_TERM]",
    search_suggestion: "\u041D\u0438\u0447\u0435\u0433\u043E \u043D\u0435 \u043D\u0430\u0439\u0434\u0435\u043D\u043E \u043F\u043E \u0437\u0430\u043F\u0440\u043E\u0441\u0443: [SEARCH_TERM]. \u041F\u043E\u043F\u0440\u043E\u0431\u0443\u0439\u0442\u0435 \u043E\u0434\u0438\u043D \u0438\u0437 \u0441\u043B\u0435\u0434\u0443\u044E\u0449\u0438\u0445 \u0432\u0430\u0440\u0438\u0430\u043D\u0442\u043E\u0432",
    searching: "\u041F\u043E\u0438\u0441\u043A \u043F\u043E \u0437\u0430\u043F\u0440\u043E\u0441\u0443: [SEARCH_TERM]"
  };
  var ru_default = {
    thanks_to: thanks_to12,
    comments: comments12,
    direction: direction12,
    strings: strings12
  };

  // translations/sv.json
  var sv_exports = {};
  __export(sv_exports, {
    comments: () => comments13,
    default: () => sv_default,
    direction: () => direction13,
    strings: () => strings13,
    thanks_to: () => thanks_to13
  });
  var thanks_to13 = "Montazar Al-Jaber <montazar@nanawee.tech>";
  var comments13 = "";
  var direction13 = "ltr";
  var strings13 = {
    placeholder: "S\xF6k",
    clear_search: "Rensa",
    load_more: "Visa fler tr\xE4ffar",
    search_label: "S\xF6k p\xE5 denna sida",
    filters_label: "Filter",
    zero_results: "[SEARCH_TERM] gav inga tr\xE4ffar",
    many_results: "[SEARCH_TERM] gav [COUNT] tr\xE4ffar",
    one_result: "[SEARCH_TERM] gav [COUNT] tr\xE4ff",
    alt_search: "[SEARCH_TERM] gav inga tr\xE4ffar. Visar resultat f\xF6r [DIFFERENT_TERM] ist\xE4llet",
    search_suggestion: "[SEARCH_TERM] gav inga tr\xE4ffar. F\xF6rs\xF6k igen med en av f\xF6ljande s\xF6kord:",
    searching: "S\xF6ker efter [SEARCH_TERM]..."
  };
  var sv_default = {
    thanks_to: thanks_to13,
    comments: comments13,
    direction: direction13,
    strings: strings13
  };

  // translations/zh-cn.json
  var zh_cn_exports = {};
  __export(zh_cn_exports, {
    comments: () => comments14,
    default: () => zh_cn_default,
    direction: () => direction14,
    strings: () => strings14,
    thanks_to: () => thanks_to14
  });
  var thanks_to14 = "Amber Song";
  var comments14 = "";
  var direction14 = "ltr";
  var strings14 = {
    placeholder: "\u641C\u7D22",
    clear_search: "\u6E05\u9664",
    load_more: "\u52A0\u8F7D\u66F4\u591A\u7ED3\u679C",
    search_label: "\u7AD9\u5185\u641C\u7D22",
    filters_label: "\u7B5B\u9009",
    zero_results: "\u672A\u627E\u5230 [SEARCH_TERM] \u7684\u76F8\u5173\u7ED3\u679C",
    many_results: "\u627E\u5230 [COUNT] \u4E2A [SEARCH_TERM] \u7684\u76F8\u5173\u7ED3\u679C",
    one_result: "\u627E\u5230 [COUNT] \u4E2A [SEARCH_TERM] \u7684\u76F8\u5173\u7ED3\u679C",
    alt_search: "\u672A\u627E\u5230 [SEARCH_TERM] \u7684\u76F8\u5173\u7ED3\u679C\u3002\u6539\u4E3A\u663E\u793A [DIFFERENT_TERM] \u7684\u76F8\u5173\u7ED3\u679C",
    search_suggestion: "\u672A\u627E\u5230 [SEARCH_TERM] \u7684\u76F8\u5173\u7ED3\u679C\u3002\u8BF7\u5C1D\u8BD5\u4EE5\u4E0B\u641C\u7D22\u3002",
    searching: "\u6B63\u5728\u641C\u7D22 [SEARCH_TERM]..."
  };
  var zh_cn_default = {
    thanks_to: thanks_to14,
    comments: comments14,
    direction: direction14,
    strings: strings14
  };

  // translations/zh-tw.json
  var zh_tw_exports = {};
  __export(zh_tw_exports, {
    comments: () => comments15,
    default: () => zh_tw_default,
    direction: () => direction15,
    strings: () => strings15,
    thanks_to: () => thanks_to15
  });
  var thanks_to15 = "Amber Song";
  var comments15 = "";
  var direction15 = "ltr";
  var strings15 = {
    placeholder: "\u641C\u7D22",
    clear_search: "\u6E05\u9664",
    load_more: "\u52A0\u8F09\u66F4\u591A\u7D50\u679C",
    search_label: "\u7AD9\u5167\u641C\u7D22",
    filters_label: "\u7BE9\u9078",
    zero_results: "\u672A\u627E\u5230 [SEARCH_TERM] \u7684\u76F8\u95DC\u7D50\u679C",
    many_results: "\u627E\u5230 [COUNT] \u500B [SEARCH_TERM] \u7684\u76F8\u95DC\u7D50\u679C",
    one_result: "\u627E\u5230 [COUNT] \u500B [SEARCH_TERM] \u7684\u76F8\u95DC\u7D50\u679C",
    alt_search: "\u672A\u627E\u5230 [SEARCH_TERM] \u7684\u76F8\u95DC\u7D50\u679C\u3002\u6539\u70BA\u986F\u793A [DIFFERENT_TERM] \u7684\u76F8\u95DC\u7D50\u679C",
    search_suggestion: "\u672A\u627E\u5230 [SEARCH_TERM] \u7684\u76F8\u95DC\u7D50\u679C\u3002\u8ACB\u5617\u8A66\u4EE5\u4E0B\u641C\u7D22\u3002",
    searching: "\u6B63\u5728\u641C\u7D22 [SEARCH_TERM]..."
  };
  var zh_tw_default = {
    thanks_to: thanks_to15,
    comments: comments15,
    direction: direction15,
    strings: strings15
  };

  // translations/zh.json
  var zh_exports = {};
  __export(zh_exports, {
    comments: () => comments16,
    default: () => zh_default,
    direction: () => direction16,
    strings: () => strings16,
    thanks_to: () => thanks_to16
  });
  var thanks_to16 = "Amber Song";
  var comments16 = "";
  var direction16 = "ltr";
  var strings16 = {
    placeholder: "\u641C\u7D22",
    clear_search: "\u6E05\u9664",
    load_more: "\u52A0\u8F7D\u66F4\u591A\u7ED3\u679C",
    search_label: "\u7AD9\u5185\u641C\u7D22",
    filters_label: "\u7B5B\u9009",
    zero_results: "\u672A\u627E\u5230 [SEARCH_TERM] \u7684\u76F8\u5173\u7ED3\u679C",
    many_results: "\u627E\u5230 [COUNT] \u4E2A [SEARCH_TERM] \u7684\u76F8\u5173\u7ED3\u679C",
    one_result: "\u627E\u5230 [COUNT] \u4E2A [SEARCH_TERM] \u7684\u76F8\u5173\u7ED3\u679C",
    alt_search: "\u672A\u627E\u5230 [SEARCH_TERM] \u7684\u76F8\u5173\u7ED3\u679C\u3002\u6539\u4E3A\u663E\u793A [DIFFERENT_TERM] \u7684\u76F8\u5173\u7ED3\u679C",
    search_suggestion: "\u672A\u627E\u5230 [SEARCH_TERM] \u7684\u76F8\u5173\u7ED3\u679C\u3002\u8BF7\u5C1D\u8BD5\u4EE5\u4E0B\u641C\u7D22\u3002",
    searching: "\u6B63\u5728\u641C\u7D22 [SEARCH_TERM]..."
  };
  var zh_default = {
    thanks_to: thanks_to16,
    comments: comments16,
    direction: direction16,
    strings: strings16
  };

  // import-glob:../translations/*.json
  var modules = [af_exports, ca_exports, de_exports, en_exports, es_exports, fr_exports, gl_exports, ja_exports, nl_exports, no_exports, pt_exports, ru_exports, sv_exports, zh_cn_exports, zh_tw_exports, zh_exports];
  var __default = modules;
  var filenames = ["../translations/af.json", "../translations/ca.json", "../translations/de.json", "../translations/en.json", "../translations/es.json", "../translations/fr.json", "../translations/gl.json", "../translations/ja.json", "../translations/nl.json", "../translations/no.json", "../translations/pt.json", "../translations/ru.json", "../translations/sv.json", "../translations/zh-cn.json", "../translations/zh-tw.json", "../translations/zh.json"];

  // svelte/ui.svelte
  function get_each_context3(ctx, list, i) {
    const child_ctx = ctx.slice();
    child_ctx[37] = list[i];
    return child_ctx;
  }
  function create_if_block_6(ctx) {
    let filters;
    let updating_selected_filters;
    let current;
    function filters_selected_filters_binding(value) {
      ctx[24](value);
    }
    let filters_props = {
      show_empty_filters: (
        /*show_empty_filters*/
        ctx[3]
      ),
      available_filters: (
        /*available_filters*/
        ctx[13]
      ),
      translate: (
        /*translate*/
        ctx[14]
      )
    };
    if (
      /*selected_filters*/
      ctx[6] !== void 0
    ) {
      filters_props.selected_filters = /*selected_filters*/
      ctx[6];
    }
    filters = new filters_default({ props: filters_props });
    binding_callbacks.push(() => bind(filters, "selected_filters", filters_selected_filters_binding));
    return {
      c() {
        create_component(filters.$$.fragment);
      },
      m(target, anchor) {
        mount_component(filters, target, anchor);
        current = true;
      },
      p(ctx2, dirty) {
        const filters_changes = {};
        if (dirty[0] & /*show_empty_filters*/
        8)
          filters_changes.show_empty_filters = /*show_empty_filters*/
          ctx2[3];
        if (dirty[0] & /*available_filters*/
        8192)
          filters_changes.available_filters = /*available_filters*/
          ctx2[13];
        if (!updating_selected_filters && dirty[0] & /*selected_filters*/
        64) {
          updating_selected_filters = true;
          filters_changes.selected_filters = /*selected_filters*/
          ctx2[6];
          add_flush_callback(() => updating_selected_filters = false);
        }
        filters.$set(filters_changes);
      },
      i(local) {
        if (current)
          return;
        transition_in(filters.$$.fragment, local);
        current = true;
      },
      o(local) {
        transition_out(filters.$$.fragment, local);
        current = false;
      },
      d(detaching) {
        destroy_component(filters, detaching);
      }
    };
  }
  function create_if_block3(ctx) {
    let div;
    let current_block_type_index;
    let if_block;
    let current;
    const if_block_creators = [create_if_block_13, create_else_block2];
    const if_blocks = [];
    function select_block_type(ctx2, dirty) {
      if (
        /*loading*/
        ctx2[9]
      )
        return 0;
      return 1;
    }
    current_block_type_index = select_block_type(ctx, [-1, -1]);
    if_block = if_blocks[current_block_type_index] = if_block_creators[current_block_type_index](ctx);
    return {
      c() {
        div = element("div");
        if_block.c();
        attr(div, "class", "pagefind-ui__results-area svelte-142hkwb");
      },
      m(target, anchor) {
        insert(target, div, anchor);
        if_blocks[current_block_type_index].m(div, null);
        current = true;
      },
      p(ctx2, dirty) {
        let previous_block_index = current_block_type_index;
        current_block_type_index = select_block_type(ctx2, dirty);
        if (current_block_type_index === previous_block_index) {
          if_blocks[current_block_type_index].p(ctx2, dirty);
        } else {
          group_outros();
          transition_out(if_blocks[previous_block_index], 1, 1, () => {
            if_blocks[previous_block_index] = null;
          });
          check_outros();
          if_block = if_blocks[current_block_type_index];
          if (!if_block) {
            if_block = if_blocks[current_block_type_index] = if_block_creators[current_block_type_index](ctx2);
            if_block.c();
          } else {
            if_block.p(ctx2, dirty);
          }
          transition_in(if_block, 1);
          if_block.m(div, null);
        }
      },
      i(local) {
        if (current)
          return;
        transition_in(if_block);
        current = true;
      },
      o(local) {
        transition_out(if_block);
        current = false;
      },
      d(detaching) {
        if (detaching)
          detach(div);
        if_blocks[current_block_type_index].d();
      }
    };
  }
  function create_else_block2(ctx) {
    let p;
    let t0;
    let ol;
    let each_blocks = [];
    let each_1_lookup = /* @__PURE__ */ new Map();
    let t1;
    let if_block1_anchor;
    let current;
    function select_block_type_1(ctx2, dirty) {
      if (
        /*searchResult*/
        ctx2[8].results.length === 0
      )
        return create_if_block_42;
      if (
        /*searchResult*/
        ctx2[8].results.length === 1
      )
        return create_if_block_5;
      return create_else_block_1;
    }
    let current_block_type = select_block_type_1(ctx, [-1, -1]);
    let if_block0 = current_block_type(ctx);
    let each_value = (
      /*searchResult*/
      ctx[8].results.slice(
        0,
        /*show*/
        ctx[12]
      )
    );
    const get_key = (ctx2) => (
      /*result*/
      ctx2[37].id
    );
    for (let i = 0; i < each_value.length; i += 1) {
      let child_ctx = get_each_context3(ctx, each_value, i);
      let key = get_key(child_ctx);
      each_1_lookup.set(key, each_blocks[i] = create_each_block3(key, child_ctx));
    }
    let if_block1 = (
      /*searchResult*/
      ctx[8].results.length > /*show*/
      ctx[12] && create_if_block_32(ctx)
    );
    return {
      c() {
        p = element("p");
        if_block0.c();
        t0 = space();
        ol = element("ol");
        for (let i = 0; i < each_blocks.length; i += 1) {
          each_blocks[i].c();
        }
        t1 = space();
        if (if_block1)
          if_block1.c();
        if_block1_anchor = empty();
        attr(p, "class", "pagefind-ui__message svelte-142hkwb");
        attr(ol, "class", "pagefind-ui__results svelte-142hkwb");
      },
      m(target, anchor) {
        insert(target, p, anchor);
        if_block0.m(p, null);
        insert(target, t0, anchor);
        insert(target, ol, anchor);
        for (let i = 0; i < each_blocks.length; i += 1) {
          each_blocks[i].m(ol, null);
        }
        insert(target, t1, anchor);
        if (if_block1)
          if_block1.m(target, anchor);
        insert(target, if_block1_anchor, anchor);
        current = true;
      },
      p(ctx2, dirty) {
        if (current_block_type === (current_block_type = select_block_type_1(ctx2, dirty)) && if_block0) {
          if_block0.p(ctx2, dirty);
        } else {
          if_block0.d(1);
          if_block0 = current_block_type(ctx2);
          if (if_block0) {
            if_block0.c();
            if_block0.m(p, null);
          }
        }
        if (dirty[0] & /*show_images, process_result, searchResult, show*/
        4358) {
          each_value = /*searchResult*/
          ctx2[8].results.slice(
            0,
            /*show*/
            ctx2[12]
          );
          group_outros();
          each_blocks = update_keyed_each(each_blocks, dirty, get_key, 1, ctx2, each_value, each_1_lookup, ol, outro_and_destroy_block, create_each_block3, null, get_each_context3);
          check_outros();
        }
        if (
          /*searchResult*/
          ctx2[8].results.length > /*show*/
          ctx2[12]
        ) {
          if (if_block1) {
            if_block1.p(ctx2, dirty);
          } else {
            if_block1 = create_if_block_32(ctx2);
            if_block1.c();
            if_block1.m(if_block1_anchor.parentNode, if_block1_anchor);
          }
        } else if (if_block1) {
          if_block1.d(1);
          if_block1 = null;
        }
      },
      i(local) {
        if (current)
          return;
        for (let i = 0; i < each_value.length; i += 1) {
          transition_in(each_blocks[i]);
        }
        current = true;
      },
      o(local) {
        for (let i = 0; i < each_blocks.length; i += 1) {
          transition_out(each_blocks[i]);
        }
        current = false;
      },
      d(detaching) {
        if (detaching)
          detach(p);
        if_block0.d();
        if (detaching)
          detach(t0);
        if (detaching)
          detach(ol);
        for (let i = 0; i < each_blocks.length; i += 1) {
          each_blocks[i].d();
        }
        if (detaching)
          detach(t1);
        if (if_block1)
          if_block1.d(detaching);
        if (detaching)
          detach(if_block1_anchor);
      }
    };
  }
  function create_if_block_13(ctx) {
    let if_block_anchor;
    let if_block = (
      /*search_term*/
      ctx[11] && create_if_block_22(ctx)
    );
    return {
      c() {
        if (if_block)
          if_block.c();
        if_block_anchor = empty();
      },
      m(target, anchor) {
        if (if_block)
          if_block.m(target, anchor);
        insert(target, if_block_anchor, anchor);
      },
      p(ctx2, dirty) {
        if (
          /*search_term*/
          ctx2[11]
        ) {
          if (if_block) {
            if_block.p(ctx2, dirty);
          } else {
            if_block = create_if_block_22(ctx2);
            if_block.c();
            if_block.m(if_block_anchor.parentNode, if_block_anchor);
          }
        } else if (if_block) {
          if_block.d(1);
          if_block = null;
        }
      },
      i: noop,
      o: noop,
      d(detaching) {
        if (if_block)
          if_block.d(detaching);
        if (detaching)
          detach(if_block_anchor);
      }
    };
  }
  function create_else_block_1(ctx) {
    let t_value = (
      /*translate*/
      ctx[14]("many_results").replace(
        /\[SEARCH_TERM\]/,
        /*search_term*/
        ctx[11]
      ).replace(/\[COUNT\]/, new Intl.NumberFormat(
        /*translations*/
        ctx[4].language
      ).format(
        /*searchResult*/
        ctx[8].results.length
      )) + ""
    );
    let t;
    return {
      c() {
        t = text(t_value);
      },
      m(target, anchor) {
        insert(target, t, anchor);
      },
      p(ctx2, dirty) {
        if (dirty[0] & /*search_term, translations, searchResult*/
        2320 && t_value !== (t_value = /*translate*/
        ctx2[14]("many_results").replace(
          /\[SEARCH_TERM\]/,
          /*search_term*/
          ctx2[11]
        ).replace(/\[COUNT\]/, new Intl.NumberFormat(
          /*translations*/
          ctx2[4].language
        ).format(
          /*searchResult*/
          ctx2[8].results.length
        )) + ""))
          set_data(t, t_value);
      },
      d(detaching) {
        if (detaching)
          detach(t);
      }
    };
  }
  function create_if_block_5(ctx) {
    let t_value = (
      /*translate*/
      ctx[14]("one_result").replace(
        /\[SEARCH_TERM\]/,
        /*search_term*/
        ctx[11]
      ).replace(/\[COUNT\]/, new Intl.NumberFormat(
        /*translations*/
        ctx[4].language
      ).format(1)) + ""
    );
    let t;
    return {
      c() {
        t = text(t_value);
      },
      m(target, anchor) {
        insert(target, t, anchor);
      },
      p(ctx2, dirty) {
        if (dirty[0] & /*search_term, translations*/
        2064 && t_value !== (t_value = /*translate*/
        ctx2[14]("one_result").replace(
          /\[SEARCH_TERM\]/,
          /*search_term*/
          ctx2[11]
        ).replace(/\[COUNT\]/, new Intl.NumberFormat(
          /*translations*/
          ctx2[4].language
        ).format(1)) + ""))
          set_data(t, t_value);
      },
      d(detaching) {
        if (detaching)
          detach(t);
      }
    };
  }
  function create_if_block_42(ctx) {
    let t_value = (
      /*translate*/
      ctx[14]("zero_results").replace(
        /\[SEARCH_TERM\]/,
        /*search_term*/
        ctx[11]
      ) + ""
    );
    let t;
    return {
      c() {
        t = text(t_value);
      },
      m(target, anchor) {
        insert(target, t, anchor);
      },
      p(ctx2, dirty) {
        if (dirty[0] & /*search_term*/
        2048 && t_value !== (t_value = /*translate*/
        ctx2[14]("zero_results").replace(
          /\[SEARCH_TERM\]/,
          /*search_term*/
          ctx2[11]
        ) + ""))
          set_data(t, t_value);
      },
      d(detaching) {
        if (detaching)
          detach(t);
      }
    };
  }
  function create_each_block3(key_1, ctx) {
    let first;
    let result;
    let current;
    result = new result_default({
      props: {
        show_images: (
          /*show_images*/
          ctx[1]
        ),
        process_result: (
          /*process_result*/
          ctx[2]
        ),
        result: (
          /*result*/
          ctx[37]
        )
      }
    });
    return {
      key: key_1,
      first: null,
      c() {
        first = empty();
        create_component(result.$$.fragment);
        this.first = first;
      },
      m(target, anchor) {
        insert(target, first, anchor);
        mount_component(result, target, anchor);
        current = true;
      },
      p(new_ctx, dirty) {
        ctx = new_ctx;
        const result_changes = {};
        if (dirty[0] & /*show_images*/
        2)
          result_changes.show_images = /*show_images*/
          ctx[1];
        if (dirty[0] & /*process_result*/
        4)
          result_changes.process_result = /*process_result*/
          ctx[2];
        if (dirty[0] & /*searchResult, show*/
        4352)
          result_changes.result = /*result*/
          ctx[37];
        result.$set(result_changes);
      },
      i(local) {
        if (current)
          return;
        transition_in(result.$$.fragment, local);
        current = true;
      },
      o(local) {
        transition_out(result.$$.fragment, local);
        current = false;
      },
      d(detaching) {
        if (detaching)
          detach(first);
        destroy_component(result, detaching);
      }
    };
  }
  function create_if_block_32(ctx) {
    let button;
    let mounted;
    let dispose;
    return {
      c() {
        button = element("button");
        button.textContent = `${/*translate*/
        ctx[14]("load_more")}`;
        attr(button, "type", "button");
        attr(button, "class", "pagefind-ui__button svelte-142hkwb");
      },
      m(target, anchor) {
        insert(target, button, anchor);
        if (!mounted) {
          dispose = listen(
            button,
            "click",
            /*showMore*/
            ctx[16]
          );
          mounted = true;
        }
      },
      p: noop,
      d(detaching) {
        if (detaching)
          detach(button);
        mounted = false;
        dispose();
      }
    };
  }
  function create_if_block_22(ctx) {
    let p;
    let t_value = (
      /*translate*/
      ctx[14]("searching").replace(
        /\[SEARCH_TERM\]/,
        /*search_term*/
        ctx[11]
      ) + ""
    );
    let t;
    return {
      c() {
        p = element("p");
        t = text(t_value);
        attr(p, "class", "pagefind-ui__message svelte-142hkwb");
      },
      m(target, anchor) {
        insert(target, p, anchor);
        append(p, t);
      },
      p(ctx2, dirty) {
        if (dirty[0] & /*search_term*/
        2048 && t_value !== (t_value = /*translate*/
        ctx2[14]("searching").replace(
          /\[SEARCH_TERM\]/,
          /*search_term*/
          ctx2[11]
        ) + ""))
          set_data(t, t_value);
      },
      d(detaching) {
        if (detaching)
          detach(p);
      }
    };
  }
  function create_fragment3(ctx) {
    let div1;
    let form;
    let input;
    let input_placeholder_value;
    let t0;
    let div0;
    let t1;
    let form_aria_label_value;
    let current;
    let mounted;
    let dispose;
    let if_block0 = (
      /*initializing*/
      ctx[7] && create_if_block_6(ctx)
    );
    let if_block1 = (
      /*searched*/
      ctx[10] && create_if_block3(ctx)
    );
    return {
      c() {
        div1 = element("div");
        form = element("form");
        input = element("input");
        t0 = space();
        div0 = element("div");
        if (if_block0)
          if_block0.c();
        t1 = space();
        if (if_block1)
          if_block1.c();
        attr(input, "class", "pagefind-ui__search-input svelte-142hkwb");
        attr(input, "type", "text");
        attr(input, "placeholder", input_placeholder_value = /*translate*/
        ctx[14]("placeholder"));
        attr(div0, "class", "pagefind-ui__drawer svelte-142hkwb");
        toggle_class(div0, "pagefind-ui__hidden", !/*searched*/
        ctx[10]);
        attr(form, "class", "pagefind-ui__form svelte-142hkwb");
        attr(form, "role", "search");
        attr(form, "aria-label", form_aria_label_value = /*translate*/
        ctx[14]("search_label"));
        attr(form, "action", "javascript:void(0);");
        attr(div1, "class", "pagefind-ui svelte-142hkwb");
        toggle_class(
          div1,
          "pagefind-ui--reset",
          /*reset_styles*/
          ctx[0]
        );
      },
      m(target, anchor) {
        insert(target, div1, anchor);
        append(div1, form);
        append(form, input);
        set_input_value(
          input,
          /*val*/
          ctx[5]
        );
        append(form, t0);
        append(form, div0);
        if (if_block0)
          if_block0.m(div0, null);
        append(div0, t1);
        if (if_block1)
          if_block1.m(div0, null);
        current = true;
        if (!mounted) {
          dispose = [
            listen(
              input,
              "focus",
              /*init*/
              ctx[15]
            ),
            listen(
              input,
              "input",
              /*input_input_handler*/
              ctx[23]
            ),
            listen(form, "submit", submit_handler)
          ];
          mounted = true;
        }
      },
      p(ctx2, dirty) {
        if (dirty[0] & /*val*/
        32 && input.value !== /*val*/
        ctx2[5]) {
          set_input_value(
            input,
            /*val*/
            ctx2[5]
          );
        }
        if (
          /*initializing*/
          ctx2[7]
        ) {
          if (if_block0) {
            if_block0.p(ctx2, dirty);
            if (dirty[0] & /*initializing*/
            128) {
              transition_in(if_block0, 1);
            }
          } else {
            if_block0 = create_if_block_6(ctx2);
            if_block0.c();
            transition_in(if_block0, 1);
            if_block0.m(div0, t1);
          }
        } else if (if_block0) {
          group_outros();
          transition_out(if_block0, 1, 1, () => {
            if_block0 = null;
          });
          check_outros();
        }
        if (
          /*searched*/
          ctx2[10]
        ) {
          if (if_block1) {
            if_block1.p(ctx2, dirty);
            if (dirty[0] & /*searched*/
            1024) {
              transition_in(if_block1, 1);
            }
          } else {
            if_block1 = create_if_block3(ctx2);
            if_block1.c();
            transition_in(if_block1, 1);
            if_block1.m(div0, null);
          }
        } else if (if_block1) {
          group_outros();
          transition_out(if_block1, 1, 1, () => {
            if_block1 = null;
          });
          check_outros();
        }
        if (!current || dirty[0] & /*searched*/
        1024) {
          toggle_class(div0, "pagefind-ui__hidden", !/*searched*/
          ctx2[10]);
        }
        if (!current || dirty[0] & /*reset_styles*/
        1) {
          toggle_class(
            div1,
            "pagefind-ui--reset",
            /*reset_styles*/
            ctx2[0]
          );
        }
      },
      i(local) {
        if (current)
          return;
        transition_in(if_block0);
        transition_in(if_block1);
        current = true;
      },
      o(local) {
        transition_out(if_block0);
        transition_out(if_block1);
        current = false;
      },
      d(detaching) {
        if (detaching)
          detach(div1);
        if (if_block0)
          if_block0.d();
        if (if_block1)
          if_block1.d();
        mounted = false;
        run_all(dispose);
      }
    };
  }
  var submit_handler = (e) => e.preventDefault();
  function instance3($$self, $$props, $$invalidate) {
    const availableTranslations = {}, languages = filenames.map((file) => file.match(/([^\/]+)\.json$/)[1]);
    for (let i = 0; i < languages.length; i++) {
      availableTranslations[languages[i]] = {
        language: languages[i],
        ...__default[i].strings
      };
    }
    let { base_path = "/_pagefind/" } = $$props;
    let { reset_styles = true } = $$props;
    let { show_images = true } = $$props;
    let { process_result = null } = $$props;
    let { process_term = null } = $$props;
    let { show_empty_filters = true } = $$props;
    let { debounce_timeout_ms = 300 } = $$props;
    let { pagefind_options = {} } = $$props;
    let { merge_index = [] } = $$props;
    let { trigger_search_term = "" } = $$props;
    let { translations = {} } = $$props;
    let val = "";
    let pagefind;
    let initializing = false;
    let searchResult = [];
    let loading = false;
    let searched = false;
    let search_id = 0;
    let search_term = "";
    let show = 5;
    let initial_filters = null;
    let available_filters = null;
    let selected_filters = {};
    let automatic_translations = availableTranslations["en"];
    const translate = (key) => {
      return translations[key] ?? automatic_translations[key] ?? "";
    };
    onMount(() => {
      let lang = document?.querySelector?.("html")?.getAttribute?.("lang") || "en";
      let parsedLang = parse(lang.toLocaleLowerCase());
      automatic_translations = availableTranslations[`${parsedLang.language}-${parsedLang.script}-${parsedLang.region}`] || availableTranslations[`${parsedLang.language}-${parsedLang.region}`] || availableTranslations[`${parsedLang.language}`] || availableTranslations["en"];
    });
    const init2 = async () => {
      if (initializing)
        return;
      $$invalidate(7, initializing = true);
      if (!pagefind) {
        let imported_pagefind = await import(`${base_path}pagefind.js`);
        await imported_pagefind.options(pagefind_options || {});
        for (const index of merge_index) {
          if (!index.bundlePath) {
            throw new Error("mergeIndex requires a bundlePath parameter");
          }
          const url = index.bundlePath;
          delete index["bundlePath"];
          await imported_pagefind.mergeIndex(url, index);
        }
        pagefind = imported_pagefind;
        loadFilters();
      }
    };
    const loadFilters = async () => {
      if (pagefind) {
        initial_filters = await pagefind.filters();
        if (!available_filters || !Object.keys(available_filters).length) {
          $$invalidate(13, available_filters = initial_filters);
        }
      }
    };
    const parseSelectedFilters = (filters) => {
      let filter = {};
      Object.entries(filters).filter(([, selected]) => selected).forEach(([selection]) => {
        let [key, value] = selection.split(/:(.*)$/);
        filter[key] = filter[key] || [];
        filter[key].push(value);
      });
      return filter;
    };
    let timer;
    const debouncedSearch = async (term, raw_filters) => {
      if (!term) {
        $$invalidate(10, searched = false);
        if (timer)
          clearTimeout(timer);
        return;
      }
      const filters = parseSelectedFilters(raw_filters);
      const executeSearchFunc = () => search(term, filters);
      if (debounce_timeout_ms > 0 && term) {
        if (timer)
          clearTimeout(timer);
        timer = setTimeout(executeSearchFunc, debounce_timeout_ms);
        await waitForApiInit();
        pagefind.preload(term, { filters });
      } else {
        executeSearchFunc();
      }
    };
    const waitForApiInit = async () => {
      while (!pagefind) {
        init2();
        await new Promise((resolve) => setTimeout(resolve, 50));
      }
    };
    const search = async (term, filters) => {
      $$invalidate(11, search_term = term || "");
      if (typeof process_term === "function") {
        term = process_term(process_term);
      }
      $$invalidate(9, loading = true);
      $$invalidate(10, searched = true);
      await waitForApiInit();
      const local_search_id = ++search_id;
      const results = await pagefind.search(term, { filters });
      if (search_id === local_search_id) {
        if (results.filters && Object.keys(results.filters)?.length) {
          $$invalidate(13, available_filters = results.filters);
        }
        $$invalidate(8, searchResult = results);
        $$invalidate(9, loading = false);
        $$invalidate(12, show = 5);
      }
    };
    const showMore = (e) => {
      e?.preventDefault();
      $$invalidate(12, show += 5);
    };
    function input_input_handler() {
      val = this.value;
      $$invalidate(5, val), $$invalidate(17, trigger_search_term);
    }
    function filters_selected_filters_binding(value) {
      selected_filters = value;
      $$invalidate(6, selected_filters);
    }
    $$self.$$set = ($$props2) => {
      if ("base_path" in $$props2)
        $$invalidate(18, base_path = $$props2.base_path);
      if ("reset_styles" in $$props2)
        $$invalidate(0, reset_styles = $$props2.reset_styles);
      if ("show_images" in $$props2)
        $$invalidate(1, show_images = $$props2.show_images);
      if ("process_result" in $$props2)
        $$invalidate(2, process_result = $$props2.process_result);
      if ("process_term" in $$props2)
        $$invalidate(19, process_term = $$props2.process_term);
      if ("show_empty_filters" in $$props2)
        $$invalidate(3, show_empty_filters = $$props2.show_empty_filters);
      if ("debounce_timeout_ms" in $$props2)
        $$invalidate(20, debounce_timeout_ms = $$props2.debounce_timeout_ms);
      if ("pagefind_options" in $$props2)
        $$invalidate(21, pagefind_options = $$props2.pagefind_options);
      if ("merge_index" in $$props2)
        $$invalidate(22, merge_index = $$props2.merge_index);
      if ("trigger_search_term" in $$props2)
        $$invalidate(17, trigger_search_term = $$props2.trigger_search_term);
      if ("translations" in $$props2)
        $$invalidate(4, translations = $$props2.translations);
    };
    $$self.$$.update = () => {
      if ($$self.$$.dirty[0] & /*trigger_search_term*/
      131072) {
        $:
          if (trigger_search_term) {
            $$invalidate(5, val = trigger_search_term);
            $$invalidate(17, trigger_search_term = "");
          }
      }
      if ($$self.$$.dirty[0] & /*val, selected_filters*/
      96) {
        $:
          debouncedSearch(val, selected_filters);
      }
    };
    return [
      reset_styles,
      show_images,
      process_result,
      show_empty_filters,
      translations,
      val,
      selected_filters,
      initializing,
      searchResult,
      loading,
      searched,
      search_term,
      show,
      available_filters,
      translate,
      init2,
      showMore,
      trigger_search_term,
      base_path,
      process_term,
      debounce_timeout_ms,
      pagefind_options,
      merge_index,
      input_input_handler,
      filters_selected_filters_binding
    ];
  }
  var Ui = class extends SvelteComponent {
    constructor(options) {
      super();
      init(
        this,
        options,
        instance3,
        create_fragment3,
        safe_not_equal,
        {
          base_path: 18,
          reset_styles: 0,
          show_images: 1,
          process_result: 2,
          process_term: 19,
          show_empty_filters: 3,
          debounce_timeout_ms: 20,
          pagefind_options: 21,
          merge_index: 22,
          trigger_search_term: 17,
          translations: 4
        },
        null,
        [-1, -1]
      );
    }
  };
  var ui_default = Ui;

  // ui.js
  var scriptBundlePath;
  try {
    scriptBundlePath = new URL(document.currentScript.src).pathname.match(/^(.*\/)(?:pagefind-)?ui.js.*$/)[1];
  } catch (e) {
    scriptBundlePath = "/_pagefind/";
    console.warn(`Pagefind couldn't determine the base of the bundle from the javascript import path. Falling back to the default of ${bundlePath}.`);
    console.warn("You can configure this by passing a bundlePath option to PagefindUI");
    console.warn(`[DEBUG: Loaded from ${document?.currentScript?.src ?? "unknown"}]`);
  }
  var PagefindUI = class {
    constructor(opts) {
      this._pfs = null;
      let selector = opts.element ?? "[data-pagefind-ui]";
      let bundlePath2 = opts.bundlePath ?? scriptBundlePath;
      let resetStyles = opts.resetStyles ?? true;
      let showImages = opts.showImages ?? true;
      let processResult = opts.processResult ?? null;
      let processTerm = opts.processTerm ?? null;
      let showEmptyFilters = opts.showEmptyFilters ?? true;
      let debounceTimeoutMs = opts.debounceTimeoutMs ?? 300;
      let mergeIndex = opts.mergeIndex ?? [];
      let translations = opts.translations ?? [];
      delete opts["element"];
      delete opts["bundlePath"];
      delete opts["resetStyles"];
      delete opts["showImages"];
      delete opts["processResult"];
      delete opts["processTerm"];
      delete opts["showEmptyFilters"];
      delete opts["debounceTimeoutMs"];
      delete opts["mergeIndex"];
      delete opts["translations"];
      const dom = document.querySelector(selector);
      if (dom) {
        this._pfs = new ui_default({
          target: dom,
          props: {
            base_path: bundlePath2,
            reset_styles: resetStyles,
            show_images: showImages,
            process_result: processResult,
            process_term: processTerm,
            show_empty_filters: showEmptyFilters,
            debounce_timeout_ms: debounceTimeoutMs,
            merge_index: mergeIndex,
            translations,
            pagefind_options: opts
          }
        });
      } else {
        console.error(`Pagefind UI couldn't find the selector ${selector}`);
      }
    }
    triggerSearch(term) {
      this._pfs.$$set({ "trigger_search_term": term });
    }
  };
  window.PagefindUI = PagefindUI;
})();
