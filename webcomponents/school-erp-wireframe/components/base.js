// Base class every custom component extends.
// - Uses Shadow DOM for style isolation.
// - Relies on CSS Custom Properties (design tokens) INHERITING from
//   the host document's :root, so we don't need to re-link tokens.css
//   inside every shadow root. This avoids the FOUC/flash caused by
//   each component blocking on its own <link> load.
// - Provides tiny helpers: this.$('sel'), this.$$('sel'), this.emit()

export class BaseElement extends HTMLElement {
  static styles = ``; // override in subclass

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    if (this.shadowRoot.childElementCount === 0) {
      const body = (typeof this.render === 'function') ? this.render() : '';
      // Only a single <style> tag — tokens are inherited from :root.
      this.shadowRoot.innerHTML = `<style>${this.constructor.styles || ''}</style>${body}`;
    }
    if (typeof this.afterRender === 'function') this.afterRender();
  }

  $(sel)  { return this.shadowRoot.querySelector(sel); }
  $$(sel) { return [...this.shadowRoot.querySelectorAll(sel)]; }

  emit(name, detail = {}) {
    this.dispatchEvent(new CustomEvent(name, { detail, bubbles: true, composed: true }));
  }
}

// Small helper: read attribute with a default fallback.
export function attr(el, name, fallback = '') {
  const v = el.getAttribute(name);
  return v === null ? fallback : v;
}
