import { LitElement, html, css } from 'https://cdn.jsdelivr.net/gh/lit/dist@3/core/lit-core.min.js';

export { html, css };

/**
 * Base class for our UI components using Lit.
 */
export class BaseElement extends LitElement {
  // Helper for events
  emit(name, detail = {}) {
    this.dispatchEvent(new CustomEvent(name, { 
      detail, 
      bubbles: true, 
      composed: true 
    }));
  }

  // Helper for simple attribute access if needed outside Lit properties
  attr(name, fallback = '') {
    const v = this.getAttribute(name);
    return v === null ? fallback : v;
  }
}
