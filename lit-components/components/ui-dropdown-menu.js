import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-dropdown-menu>
 *   <ui-button slot="trigger" variant="ghost" icon="more"></ui-button>
 *   <a href="#edit">Edit</a>
 *   <a href="#duplicate">Duplicate</a>
 *   <hr>
 *   <a href="#delete" data-tone="danger">Delete</a>
 * </ui-dropdown-menu>
 *
 * DSL surface:
 *   - align   : "start" (default) | "end"     – align to trigger's left/right edge
 *   - open    : boolean
 *
 * Any slotted <a>, <button>, or element with a data-tone stripe:
 *   - data-tone="danger" makes the row red.
 *   - <hr> renders as a divider.
 *
 * Emits `ui-select` with { item } whenever a menu row is clicked.
 */
class UIDropdownMenu extends LitBaseElement {
  static properties = {
    align: { type: String, reflect: true },
    open:  { type: Boolean, reflect: true },
  };

  static styles = css`
    :host { display: inline-block; position: relative; }
    :host([open]) { z-index: 1500; }
    .menu {
      position: absolute; top: calc(100% + 6px);
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
      box-shadow: var(--shadow-lg);
      min-width: 180px;
      padding: 4px;
      display: none;
      z-index: 1000;
    }
    :host([align="end"]) .menu { right: 0; } :host(:not([align="end"])) .menu { left: 0; }
    :host([open]) .menu { display: block; }
    ::slotted(a), ::slotted(button) {
      display: flex; align-items: center; gap: 8px;
      padding: 7px 10px; border-radius: 6px;
      font-size: var(--fs-sm); color: var(--color-text) !important;
      text-decoration: none; cursor: pointer;
      background: transparent; border: 0; width: 100%; text-align: left;
      font: inherit; font-size: var(--fs-sm);
    }
    ::slotted(a:hover), ::slotted(button:hover) {
      background: var(--color-primary-soft); color: var(--color-primary) !important;
    }
    ::slotted([data-tone="danger"]) {
      color: var(--color-danger) !important;
    }
    ::slotted([data-tone="danger"]:hover) {
      background: color-mix(in srgb, var(--color-danger) 12%, transparent) !important;
      color: var(--color-danger) !important;
    }
    ::slotted(hr) {
      border: 0; border-top: 1px solid var(--color-border);
      margin: 4px 4px;
    }
    ::slotted([disabled]) {
      opacity: 0.4; cursor: not-allowed; pointer-events: none;
    }
  `;

  constructor() {
    super();
    this.align = 'start';
    this.open = false;
    this._outside = (e) => { if (!this.contains(e.target)) this.#close(); };
  }

  connectedCallback() {
    super.connectedCallback();
    this.addEventListener('click', this.#onClick);
    this.addEventListener('keydown', (e) => { if (e.key === 'Escape') this.#close(); });
  }
  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener('mousedown', this._outside);
  }

  #onClick = (e) => {
    // Click on the trigger toggles the menu
    const trig = e.composedPath().find(el => el.getAttribute?.('slot') === 'trigger');
    if (trig) {
      e.stopPropagation();
      this.open ? this.#close() : this.#openMenu();
      return;
    }
    // Click on a menu item closes it and emits ui-select
    const item = e.target.closest('a, button');
    if (item && this.contains(item) && item.getAttribute?.('slot') !== 'trigger') {
      this.emit('ui-select', { item, href: item.getAttribute('href') });
      this.#close();
    }
  };

  #openMenu() {
    this.open = true;
    document.addEventListener('mousedown', this._outside);
  }
  #close() {
    this.open = false;
    document.removeEventListener('mousedown', this._outside);
  }

  render() {
    return html`
      <slot name="trigger"></slot>
      <div class="menu" role="menu">
        <slot></slot>
      </div>
    `;
  }
}
customElements.define('ui-dropdown-menu', UIDropdownMenu);
