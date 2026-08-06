import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-heading
 *   level="h1"           → h1 (default) | h2 | h3 | h4
 *   tone="default">      → default | brand | muted | success | warning | danger
 *   Section title
 * </ui-heading>
 *
 * A simple typographic heading primitive. Uses semantic `role="heading"`
 * with `aria-level` under the hood so screen readers get the right
 * outline without us relying on Shadow-DOM'd `<h1>`.
 *
 * Emits nothing.
 */
class UIHeading extends LitBaseElement {
  static properties = {
    level: { type: String, reflect: true },
    tone:  { type: String, reflect: true },
  };

  static styles = css`
    :host {
      display: block;
      font-weight: 700;
      letter-spacing: -0.01em;
      margin: 0;
      color: var(--_c, var(--color-text));
      line-height: 1.2;
    }
    /* Level → size mapping. Values chosen to match a common type scale
       (h1 ≈ 32px, h2 ≈ 24px, h3 ≈ 18px, h4 ≈ 15px). */
    :host([level="h1"]) { font-size: 2rem; }
    :host([level="h2"]) { font-size: 1.5rem; }
    :host([level="h3"]) { font-size: 1.125rem; }
    :host([level="h4"]) { font-size: 0.9375rem; text-transform: uppercase; letter-spacing: 0.05em; }

    /* Tone presets. */
    :host([tone="brand"])   { --_c: var(--color-primary, #4f46e5); }
    :host([tone="muted"])   { --_c: var(--color-text-muted); font-weight: 600; }
    :host([tone="success"]) { --_c: var(--color-success, #16a34a); }
    :host([tone="warning"]) { --_c: var(--color-warning, #d97706); }
    :host([tone="danger"])  { --_c: var(--color-danger,  #dc2626); }
  `;

  constructor() {
    super();
    this.level = 'h1';
    this.tone  = 'default';
  }

  connectedCallback() {
    super.connectedCallback();
    this.setAttribute('role', 'heading');
    this.setAttribute('aria-level', String(this.level).replace('h', ''));
  }
  updated(changed) {
    if (changed.has('level')) {
      this.setAttribute('aria-level', String(this.level).replace('h', ''));
    }
  }

  render() { return html`<slot></slot>`; }
}

customElements.define('ui-heading', UIHeading);
