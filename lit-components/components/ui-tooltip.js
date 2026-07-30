import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-tooltip text="Delete permanently" placement="top">
 *   <ui-button variant="danger" icon="x"></ui-button>
 * </ui-tooltip>
 *
 * DSL surface:
 *   - text      : the tooltip content
 *   - placement : "top" (default) | "bottom" | "left" | "right"
 *   - delay     : ms before showing (default 250)
 *
 * Shows on hover (mouseenter) AND focus (keyboard-friendly).
 * Auto-hides on mouseleave / blur / Escape.
 */
class UITooltip extends LitBaseElement {
  static properties = {
    text:      { type: String,  reflect: true },
    placement: { type: String,  reflect: true },
    delay:     { type: Number,  reflect: true },
    open:      { type: Boolean, reflect: true },
  };

  static styles = css`
    :host { display: inline-flex; position: relative; }
    .bubble {
      position: absolute;
      background: var(--color-text);
      color: var(--color-bg);
      padding: 5px 9px;
      border-radius: 6px;
      font-size: var(--fs-xs);
      font-weight: var(--fw-medium);
      line-height: 1.3;
      white-space: nowrap;
      pointer-events: none;
      opacity: 0;
      transform: translateY(4px);
      transition: opacity var(--dur-fast) var(--ease),
                  transform var(--dur-fast) var(--ease);
      z-index: 1000;
      box-shadow: var(--shadow-md);
      max-width: 240px;
    }
    :host([open]) .bubble { opacity: 1; transform: translateY(0); }

    /* Placements */
    :host([placement="top"])    .bubble { bottom: calc(100% + 6px); left: 50%; transform: translateX(-50%) translateY(4px); }
    :host([placement="top"][open])    .bubble { transform: translateX(-50%) translateY(0); }
    :host([placement="bottom"]) .bubble { top: calc(100% + 6px); left: 50%; transform: translateX(-50%) translateY(-4px); }
    :host([placement="bottom"][open]) .bubble { transform: translateX(-50%) translateY(0); }
    :host([placement="left"])   .bubble { right: calc(100% + 6px); top: 50%; transform: translateY(-50%) translateX(4px); }
    :host([placement="left"][open])   .bubble { transform: translateY(-50%) translateX(0); }
    :host([placement="right"])  .bubble { left: calc(100% + 6px); top: 50%; transform: translateY(-50%) translateX(-4px); }
    :host([placement="right"][open])  .bubble { transform: translateY(-50%) translateX(0); }
  `;

  constructor() {
    super();
    this.text = '';
    this.placement = 'top';
    this.delay = 250;
    this.open = false;
  }

  connectedCallback() {
    super.connectedCallback();
    this.addEventListener('mouseenter', this.#show);
    this.addEventListener('mouseleave', this.#hide);
    this.addEventListener('focusin',    this.#show);
    this.addEventListener('focusout',   this.#hide);
    this.addEventListener('keydown', (e) => { if (e.key === 'Escape') this.#hide(); });
  }

  #show = () => {
    clearTimeout(this._t);
    this._t = setTimeout(() => { this.open = true; }, this.delay);
  };
  #hide = () => {
    clearTimeout(this._t);
    this.open = false;
  };

  render() {
    return html`
      <slot></slot>
      ${this.text
        ? html`<span class="bubble" role="tooltip">${this.text}</span>`
        : nothing}
    `;
  }
}
customElements.define('ui-tooltip', UITooltip);
