import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Horizontal (or vertical) numbered step indicator for wizards.
 *
 * <ui-stepper current="1">
 *   <span>Basic info</span>
 *   <span>Guardian</span>
 *   <span>Documents</span>
 *   <span>Review</span>
 * </ui-stepper>
 *
 * DSL surface:
 *   - current     : 0-based index of the active step
 *   - orientation : "horizontal" (default) | "vertical"
 *   - clickable   : boolean, lets the user click a step to jump to it
 *
 * Emits `ui-change` with { current, label } when a step is clicked.
 */
class UIStepper extends LitBaseElement {
  static properties = {
    current:     { type: Number,  reflect: true },
    orientation: { type: String,  reflect: true },
    clickable:   { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: block;
      --step-sz: 28px;
    }
    .wrap {
      display: flex; align-items: center; gap: 0;
      width: 100%;
    }
    :host([orientation="vertical"]) .wrap {
      flex-direction: column; align-items: stretch; gap: 0;
    }
    .step {
      display: flex; align-items: center; gap: 10px;
      flex: 0 0 auto;
      cursor: default;
      color: var(--color-text-muted);
      font-size: var(--fs-sm);
    }
    :host([clickable]) .step { cursor: pointer; }
    .dot {
      width: var(--step-sz); height: var(--step-sz);
      border-radius: 50%;
      background: var(--color-surface-alt);
      color: var(--color-text-muted);
      border: 1.5px solid var(--color-border-strong);
      display: grid; place-items: center;
      font-size: var(--fs-xs); font-weight: var(--fw-semibold);
      font-variant-numeric: tabular-nums;
      flex: 0 0 auto;
      transition:
        background var(--dur-fast) var(--ease),
        border-color var(--dur-fast) var(--ease),
        color var(--dur-fast) var(--ease);
    }
    .step[aria-current="step"] .dot {
      background: var(--color-primary); border-color: var(--color-primary);
      color: var(--color-primary-contrast);
    }
    .step.done .dot {
      background: var(--color-primary); border-color: var(--color-primary);
      color: var(--color-primary-contrast);
    }
    .step[aria-current="step"] .label,
    .step.done .label { color: var(--color-text); }
    .step[aria-current="step"] .label { font-weight: var(--fw-semibold); }
    .conn {
      flex: 1 1 auto;
      height: 2px; background: var(--color-border);
      margin: 0 8px; border-radius: 999px;
      min-width: 20px;
      transition: background var(--dur-med) var(--ease);
    }
    .conn.done { background: var(--color-primary); }
    :host([orientation="vertical"]) .conn {
      width: 2px; height: 24px; margin: 0 0 0 calc(var(--step-sz) / 2 - 1px);
    }
    :host([orientation="vertical"]) .step { padding: 4px 0; }
  `;

  constructor() {
    super();
    this.current = 0;
    this.orientation = 'horizontal';
    this.clickable = false;
    this._steps = [];
  }

  connectedCallback() {
    super.connectedCallback();
    this._mo = new MutationObserver(() => this.#rebuild());
    this._mo.observe(this, { childList: true });
    queueMicrotask(() => this.#rebuild());
  }
  disconnectedCallback() { super.disconnectedCallback(); this._mo?.disconnect(); }

  #rebuild() {
    this._steps = [...this.children]
      .filter(el => el.nodeType === 1)
      .map(el => (el.textContent || '').trim());
    this.requestUpdate();
  }

  #onClick = (i) => {
    if (!this.clickable) return;
    this.current = i;
    this.emit('ui-change', { current: i, label: this._steps[i] });
  };

  render() {
    return html`
      <div class="wrap">
        ${this._steps.map((label, i) => {
          const state = i < this.current ? 'done' : (i === this.current ? 'current' : '');
          return html`
            <div class="step ${state}"
                 aria-current=${i === this.current ? 'step' : nothing}
                 @click=${() => this.#onClick(i)}>
              <div class="dot">${i < this.current ? html`<ui-icon name="check" size="14"></ui-icon>` : i + 1}</div>
              <span class="label">${label}</span>
            </div>
            ${i < this._steps.length - 1
              ? html`<div class="conn ${i < this.current ? 'done' : ''}"></div>`
              : nothing}
          `;
        })}
      </div>
      <div style="display:none"><slot></slot></div>
    `;
  }
}
customElements.define('ui-stepper', UIStepper);
