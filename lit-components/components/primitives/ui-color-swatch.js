import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-color-swatch
 *   color="#4f46e5"     → any CSS color string (hex / rgb / hsl / named)
 *   size="md"           → sm (16px) | md (24px, default) | lg (36px)
 *   selectable          → boolean; makes it focusable and clickable, emits ui-select
 *   selected>           → boolean; shows a check mark ring
 * </ui-color-swatch>
 *
 * A single circular colour dot. Compose many inside a `<ui-cluster>` to
 * build palettes, colour pickers, avatar-colour choosers, etc.
 *
 * When `selectable`, emits `ui-select` (bubbling, composed) with
 * `{ color }` on click / Enter / Space. Consumers manage the "which one
 * is selected" state themselves — the swatch just reports its intent.
 */
class UIColorSwatch extends LitBaseElement {
  static properties = {
    color:      { type: String,  reflect: true },
    size:       { type: String,  reflect: true },
    selectable: { type: Boolean, reflect: true },
    selected:   { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: inline-block;
      /* The dot itself. Sized via a CSS var so the tone rules stay small. */
      --_sz: 24px;
      width: var(--_sz);
      height: var(--_sz);
      border-radius: 50%;
      background: var(--color, transparent);
      /* Subtle border so light colours are visible against light bg. */
      box-shadow: inset 0 0 0 1px rgba(0,0,0,.12);
      position: relative;
      vertical-align: middle;
    }
    :host([size="sm"]) { --_sz: 16px; }
    :host([size="md"]) { --_sz: 24px; }
    :host([size="lg"]) { --_sz: 36px; }

    :host([selectable]) {
      cursor: pointer;
      outline: none;
    }
    :host([selectable]:focus-visible) {
      box-shadow:
        inset 0 0 0 1px rgba(0,0,0,.12),
        0 0 0 3px var(--color-focus, rgba(79,70,229,.35));
    }

    /* Selected ring + check mark. The check inherits its stroke from
       currentColor which we compute against the swatch's own bg so it
       stays visible on any colour (approximated with mix-blend). */
    :host([selected])::after {
      content: "";
      position: absolute;
      inset: 0;
      border-radius: inherit;
      box-shadow: 0 0 0 2px var(--color-surface, #fff), 0 0 0 4px var(--color, currentColor);
    }
    :host([selected]) .check {
      position: absolute;
      inset: 0;
      display: grid;
      place-items: center;
      color: #fff;
      mix-blend-mode: difference;
      font-size: calc(var(--_sz) * 0.55);
      font-weight: 700;
      line-height: 1;
      pointer-events: none;
    }
    :host(:not([selected])) .check { display: none; }
  `;

  constructor() {
    super();
    this.color      = '';
    this.size       = 'md';
    this.selectable = false;
    this.selected   = false;
  }

  connectedCallback() {
    super.connectedCallback();
    this.#syncA11y();
  }
  updated(changed) {
    if (changed.has('selectable') || changed.has('color')) this.#syncA11y();
    // Reflect the color as a CSS variable so ::after can read it too.
    if (changed.has('color')) this.style.setProperty('--color', this.color || '');
  }
  #syncA11y() {
    if (this.selectable) {
      this.tabIndex = 0;
      this.setAttribute('role', 'button');
      this.setAttribute('aria-label', `Choose ${this.color}`);
    } else {
      this.removeAttribute('tabindex');
      this.removeAttribute('role');
      this.removeAttribute('aria-label');
    }
  }

  #onActivate = (e) => {
    if (!this.selectable) return;
    if (e.type === 'keydown' && e.key !== 'Enter' && e.key !== ' ') return;
    e.preventDefault();
    this.emit('ui-select', { color: this.color });
  };

  render() {
    return html`
      <div
        class="root"
        @click=${this.#onActivate}
        @keydown=${this.#onActivate}
      >
        <span class="check">✓</span>
      </div>
    `;
  }
}

customElements.define('ui-color-swatch', UIColorSwatch);
