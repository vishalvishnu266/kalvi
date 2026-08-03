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
 *   open        : boolean — start expanded (default: collapsed)
 *   disabled    : boolean — greys the header and prevents toggling
 *   tone        : "" | "danger" — paints the header red when the section
 *                 has field-level errors so the user knows to open it
 *
 * Slots:
 *   default  — the fields inside the panel
 *   actions  — optional buttons/status pills to the right of the title
 *              (e.g. a "3 errors" badge, or "Reset section")
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
    title:    { type: String,  reflect: true },
    subtitle: { type: String,  reflect: true },
    open:     { type: Boolean, reflect: true },
    disabled: { type: Boolean, reflect: true },
    tone:     { type: String,  reflect: true },
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
    :host([tone="danger"]) summary { color: var(--color-danger); }
    :host([tone="danger"]) .titles h4 { color: var(--color-danger); }
    :host([tone="danger"]) details { border-color: var(--color-danger); }
    :host([disabled]) summary { pointer-events: none; opacity: .55; }
  `;

  constructor() {
    super();
    this.title = '';
    this.subtitle = '';
    this.open = false;
    this.disabled = false;
    this.tone = '';
  }

  #onToggle = (e) => {
    // Sync the native `open` state back to the reflected property.
    const details = e.currentTarget;
    if (details.open !== this.open) {
      this.open = details.open;
      this.emit('ui-toggle', { open: this.open });
    }
  };

  render() {
    return html`
      <details ?open=${this.open} @toggle=${this.#onToggle}>
        <summary>
          <span class="chevron" aria-hidden="true"></span>
          <div class="titles">
            ${this.title    ? html`<h4>${this.title}</h4>`  : nothing}
            ${this.subtitle ? html`<p>${this.subtitle}</p>` : nothing}
          </div>
          <slot name="actions"></slot>
        </summary>
        <div class="body"><slot></slot></div>
      </details>
    `;
  }
}
customElements.define('ui-collapse', UICollapse);
