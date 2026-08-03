import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-collapse title="Personal details" open>
 *   <ui-input label="First name"></ui-input>
 *   <ui-input label="Last name"></ui-input>
 * </ui-collapse>
 *
 * A single foldable section — the "accordion panel" primitive that lets
 * long ERP forms (Student admission, Employee onboarding, Fee structure,
 * Purchase order) group fields into collapsible chunks so users don't
 * scroll through 60 inputs at once.
 *
 * DSL surface (HTML-only, matches the Rust `form_section()` builder):
 *   title       : header label (required in practice)
 *   subtitle    : optional secondary line under the title
 *   open        : boolean — expanded state. **Defaults to true** so users
 *                 never miss fields. Set `collapsed` (or `open=""`) to
 *                 start folded, e.g. for optional / rarely-used sections.
 *   collapsed   : boolean — inverse convenience attribute. When present,
 *                 forces the section to start collapsed regardless of
 *                 the `open` default.
 *   disabled    : boolean — greys the header and prevents toggling
 *   tone        : "" | "danger" — paints the header red when the section
 *                 has field-level errors so the user knows to open it
 *
 * Slots:
 *   default  — the fields inside the panel
 *   actions  — optional buttons/status pills to the right of the title
 *              (e.g. a "3 errors" badge, or "Reset section")
 *   banner   — optional <ui-form-banner> or <ui-alert> that renders at
 *              the top of the panel body (under the header, above the
 *              fields). Useful for section-scoped error summaries.
 *
 * Emits:
 *   ui-toggle { open: boolean }  — bubbles + composed
 *
 * Implementation note: uses a native <details>/<summary> under the hood
 * so it works without JS, is keyboard-accessible (Space / Enter toggle),
 * and reflects the `open` attribute on both sides.
 */
class UICollapse extends LitBaseElement {
  static properties = {
    title:     { type: String,  reflect: true },
    subtitle:  { type: String,  reflect: true },
    open:      { type: Boolean, reflect: true },
    /**
     * Inverse of `open`. Kept as a separate attribute so authors can write
     * `<ui-collapse collapsed>` without having to think about default
     * flips ("do I set open=false or omit open?" — HTML booleans make
     * that awkward). If both are present, `collapsed` wins.
     */
    collapsed: { type: Boolean, reflect: true },
    disabled:  { type: Boolean, reflect: true },
    tone:      { type: String,  reflect: true },
  };

  static styles = css`
    :host { display: block; }
    details {
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
      background: var(--color-surface);
      overflow: hidden;
    }
    summary {
      list-style: none;
      cursor: pointer;
      display: flex; align-items: center; gap: var(--space-3);
      padding: var(--space-3) var(--space-4);
      user-select: none;
    }
    summary::-webkit-details-marker { display: none; }
    .chevron {
      display: inline-block;
      transition: transform .15s ease;
      width: 12px; height: 12px;
      flex-shrink: 0;
      border-right: 2px solid var(--color-text-muted);
      border-bottom: 2px solid var(--color-text-muted);
      transform: rotate(-45deg);
      margin-right: var(--space-1);
    }
    :host([open]) .chevron { transform: rotate(45deg); }
    .titles { flex: 1; min-width: 0; }
    .titles h4 {
      margin: 0; font-size: var(--fs-md); font-weight: var(--fw-semibold);
      color: var(--color-text);
    }
    .titles p {
      margin: 2px 0 0; font-size: var(--fs-xs); color: var(--color-text-muted);
    }
    .body {
      padding: var(--space-4);
      border-top: 1px solid var(--color-border);
      display: flex; flex-direction: column; gap: var(--space-3);
    }
    /* Section-scoped banner sits above the fields with its own margin. */
    ::slotted([slot="banner"]) { display: block; margin-bottom: var(--space-1); }
    :host([tone="danger"]) summary { color: var(--color-danger); }
    :host([tone="danger"]) .titles h4 { color: var(--color-danger); }
    :host([tone="danger"]) details { border-color: var(--color-danger); }
    :host([disabled]) summary { pointer-events: none; opacity: .55; }
  `;

  constructor() {
    super();
    this.title = '';
    this.subtitle = '';
    // Expanded by default — ERP forms would rather show all fields than
    // risk users overlooking a whole section behind a closed header.
    // Authors opt in to a collapsed start with the `collapsed` attribute.
    this.open = true;
    this.collapsed = false;
    this.disabled = false;
    this.tone = '';
  }

  /**
   * Resolve `collapsed` (author intent) at connect time. HTML boolean
   * attributes are always "present or absent", so if the tag was
   * written as `<ui-collapse collapsed>` we honour it once here rather
   * than every render.
   */
  connectedCallback() {
    super.connectedCallback();
    if (this.hasAttribute('collapsed')) {
      this.open = false;
    }
  }

  // Guard flag so we don't re-enter the toggle handler when we
  // programmatically sync <details>.open with our reflected `open`
  // property inside updated().
  #syncing = false;

  #onToggle = (e) => {
    if (this.#syncing) return;
    if (this.disabled) {
      // Never allow toggling in disabled mode — snap back.
      e.currentTarget.open = this.open;
      return;
    }
    const nowOpen = e.currentTarget.open;
    if (nowOpen !== this.open) {
      this.open = nowOpen;
      this.emit('ui-toggle', { open: this.open });
    }
  };

  /**
   * One-way *property → DOM* sync. We deliberately do NOT bind
   * `?open=${this.open}` inside render(), because Lit re-writing that
   * attribute on every render fires a spurious `toggle` event that can
   * ping-pong with #onToggle and cause the panel to flicker or refuse
   * to stay open. Instead we set `open` imperatively here, once, when
   * the reactive property actually changes.
   */
  updated(changed) {
    if (changed.has('open')) {
      const details = this.renderRoot?.querySelector('details');
      if (details && details.open !== this.open) {
        this.#syncing = true;
        details.open = this.open;
        // Release the guard after the browser has fired the resulting
        // toggle event (it's dispatched synchronously, so a microtask
        // is enough).
        queueMicrotask(() => { this.#syncing = false; });
      }
    }
  }

  /**
   * Public API — programmatic toggle. Useful for "expand all" buttons
   * or opening a section from JS after a validation failure.
   */
  toggle(force) {
    if (this.disabled) return;
    this.open = typeof force === 'boolean' ? force : !this.open;
  }

  render() {
    // NOTE: no `?open=` binding here — see updated() above for why.
    return html`
      <details @toggle=${this.#onToggle}>
        <summary>
          <span class="chevron" aria-hidden="true"></span>
          <div class="titles">
            ${this.title    ? html`<h4>${this.title}</h4>`  : nothing}
            ${this.subtitle ? html`<p>${this.subtitle}</p>` : nothing}
          </div>
          <slot name="actions"></slot>
        </summary>
        <div class="body">
          <slot name="banner"></slot>
          <slot></slot>
        </div>
      </details>
    `;
  }

  firstUpdated() {
    // Apply the initial `open` state once the shadow DOM exists.
    if (this.open) {
      const details = this.renderRoot?.querySelector('details');
      if (details) details.open = true;
    }
  }
}
customElements.define('ui-collapse', UICollapse);
