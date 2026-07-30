import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-breadcrumb>
 *   <a href="/">Home</a>
 *   <a href="/students">Students</a>
 *   <a href="/students/grade-5">Grade 5</a>
 *   <span>Aarav Kumar</span>            <!-- last is the current page -->
 * </ui-breadcrumb>
 *
 * DSL surface:
 *   - separator : text separator between items (default "/")
 *   - collapse  : boolean, hide middle items behind an ellipsis on narrow widths
 *
 * The LAST direct child is rendered as the current page (non-link style),
 * regardless of tag.
 */
class UIBreadcrumb extends LitBaseElement {
  static properties = {
    separator: { type: String, reflect: true },
    collapse:  { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: block;
      font-size: var(--fs-sm); color: var(--color-text-muted);
    }
    nav {
      display: flex; align-items: center; flex-wrap: wrap;
      gap: 6px;
    }
    ::slotted(a) {
      color: var(--color-text-muted) !important;
      text-decoration: none;
      transition: color var(--dur-fast) var(--ease);
      max-width: 240px;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    ::slotted(a:hover) { color: var(--color-primary) !important; }
    /* Style the current-page marker (any element with data-current). */
    ::slotted([data-current]) {
      color: var(--color-text) !important;
      font-weight: var(--fw-medium);
    }
    .sep {
      color: var(--color-text-subtle);
      user-select: none;
      font-size: var(--fs-xs);
    }
    .ellipsis {
      color: var(--color-text-subtle);
      cursor: pointer;
      padding: 0 4px;
      border-radius: 4px;
    }
    .ellipsis:hover {
      background: color-mix(in srgb, var(--color-text) 8%, transparent);
      color: var(--color-text);
    }
  `;

  constructor() {
    super();
    this.separator = '/';
    this.collapse  = false;
    this._items = [];
    this._collapsed = false;
  }

  connectedCallback() {
    super.connectedCallback();
    this._mo = new MutationObserver(() => this.#rebuild());
    this._mo.observe(this, { childList: true });
    queueMicrotask(() => this.#rebuild());
  }
  disconnectedCallback() { super.disconnectedCallback(); this._mo?.disconnect(); }

  #rebuild() {
    const items = [...this.children].filter(el => el.nodeType === 1);
    // Mark the last as current
    items.forEach((el, i) => {
      if (i === items.length - 1) el.setAttribute('data-current', '');
      else el.removeAttribute('data-current');
    });
    this._items = items;
    this.requestUpdate();
  }

  #toggleCollapse = () => { this._collapsed = !this._collapsed; this.requestUpdate(); };

  render() {
    // We render a nav that contains <slot>-like structure but with
    // interleaved separators. Since we need to interleave, we assign
    // each real child to a slot by name.
    const total = this._items.length;
    const useCollapse = this.collapse && total > 4 && !this._collapsed;
    const showIdx = useCollapse
      ? [0, total - 2, total - 1]
      : this._items.map((_, i) => i);

    return html`
      <nav aria-label="Breadcrumb">
        ${showIdx.map((idx, i) => html`
          ${i > 0 ? html`
            ${useCollapse && idx === total - 2
              ? html`<span class="ellipsis" title="Show all" @click=${this.#toggleCollapse}>…</span>
                     <span class="sep">${this.separator}</span>`
              : html`<span class="sep">${this.separator}</span>`}
          ` : nothing}
          <slot name="item-${idx}"></slot>
        `)}
      </nav>
    `;
  }

  updated() {
    // Assign each direct child element to its numbered slot.
    this._items.forEach((el, i) => el.setAttribute('slot', `item-${i}`));
  }
}
customElements.define('ui-breadcrumb', UIBreadcrumb);
