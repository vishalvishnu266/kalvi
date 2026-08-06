import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-slider
 *   name="volume"       → form field name (for form serialization)
 *   min="0" max="100"   → bounds (numeric strings; default 0 → 100)
 *   step="1"            → increment (default 1)
 *   value="42"          → current value (default = min)
 *   show-value          → boolean; render the current value next to the track
 *   disabled>           → boolean; not interactive
 * </ui-slider>
 *
 * A styled thin wrapper over a native `<input type="range">`. Native
 * gives us keyboard support (←/→, PgUp/PgDn, Home/End), touch drag,
 * screen-reader announcements, and correct form participation — all for
 * free. We only theme it.
 *
 * Emits `ui-change` (bubbling, composed) with `{ value: Number, name }`
 * on every input step so consumers (forms, previews) can react.
 */
class UISlider extends LitBaseElement {
  static properties = {
    name:         { type: String,  reflect: true },
    min:          { type: Number,  reflect: true },
    max:          { type: Number,  reflect: true },
    step:         { type: Number,  reflect: true },
    value:        { type: Number,  reflect: true },
    'show-value': { type: Boolean, reflect: true, attribute: 'show-value' },
    showValue:    { type: Boolean, attribute: 'show-value', reflect: true },
    disabled:     { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: inline-grid;
      grid-template-columns: 1fr auto;
      align-items: center;
      gap: var(--space-3);
      min-width: 160px;
      width: 100%;
      color: var(--color-text);
      font-size: var(--fs-sm);
      font-family: inherit;
    }
    :host([disabled]) { opacity: .55; pointer-events: none; }

    input[type="range"] {
      appearance: none;
      -webkit-appearance: none;
      width: 100%;
      height: 4px;
      background: var(--color-border);
      border-radius: 999px;
      outline: none;
      margin: 0;
      cursor: pointer;
    }
    /* Thumb — WebKit + Firefox. */
    input[type="range"]::-webkit-slider-thumb {
      appearance: none;
      -webkit-appearance: none;
      width: 16px; height: 16px;
      background: var(--color-primary, var(--color-brand, #4f46e5));
      border-radius: 50%;
      border: 2px solid var(--color-surface, #fff);
      box-shadow: 0 1px 2px rgba(0,0,0,.25);
      cursor: pointer;
      transition: transform .12s ease;
    }
    input[type="range"]:focus-visible::-webkit-slider-thumb {
      transform: scale(1.15);
      box-shadow: 0 0 0 3px var(--color-focus, rgba(79,70,229,.35));
    }
    input[type="range"]::-moz-range-thumb {
      width: 16px; height: 16px;
      background: var(--color-primary, var(--color-brand, #4f46e5));
      border-radius: 50%;
      border: 2px solid var(--color-surface, #fff);
      cursor: pointer;
    }

    .val {
      font-variant-numeric: tabular-nums;
      min-width: 3ch;
      text-align: right;
      color: var(--color-text-muted);
    }
    :host(:not([show-value])) .val { display: none; }
  `;

  constructor() {
    super();
    this.name      = '';
    this.min       = 0;
    this.max       = 100;
    this.step      = 1;
    this.value     = 0;
    this.showValue = false;
    this.disabled  = false;
  }

  #onInput = (e) => {
    const v = Number(e.target.value);
    this.value = v;
    this.emit('ui-change', { value: v, name: this.name });
  };

  render() {
    return html`
      <input
        type="range"
        .value=${String(this.value)}
        min=${this.min}
        max=${this.max}
        step=${this.step}
        ?disabled=${this.disabled}
        name=${this.name || null}
        @input=${this.#onInput}
        aria-valuenow=${this.value}
        aria-valuemin=${this.min}
        aria-valuemax=${this.max}
      />
      <span class="val">${this.value}</span>
    `;
  }
}

customElements.define('ui-slider', UISlider);
