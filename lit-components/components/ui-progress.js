import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Linear + circular progress indicator.
 *
 * <ui-progress value="65" max="100"></ui-progress>
 * <ui-progress value="65" max="100" label="Fee collection"></ui-progress>
 * <ui-progress value="65" shape="circle" size="72"></ui-progress>
 * <ui-progress indeterminate></ui-progress>
 *
 * DSL surface:
 *   - value         : current value (default 0)
 *   - max           : max value (default 100)
 *   - shape         : "linear" (default) | "circle"
 *   - size          : circular diameter in px (default 56)
 *   - tone          : "brand" (default) | "success" | "warning" | "danger" | "info"
 *   - label         : optional caption shown above (linear) or inside (circle)
 *   - show-value    : boolean, render `value / max` beside the bar/ring
 *   - indeterminate : boolean, animated shimmer for unknown-duration work
 */
class UIProgress extends LitBaseElement {
  static properties = {
    value:         { type: Number,  reflect: true },
    max:           { type: Number,  reflect: true },
    shape:         { type: String,  reflect: true },
    size:          { type: Number,  reflect: true },
    tone:          { type: String,  reflect: true },
    label:         { type: String,  reflect: true },
    'show-value':  { type: Boolean, reflect: true, attribute: 'show-value' },
    indeterminate: { type: Boolean, reflect: true },
  };

  static styles = css`
    :host { display: block; color: var(--color-text); --pg-tone: var(--color-primary); }
    :host([tone="success"]) { --pg-tone: var(--color-success); }
    :host([tone="warning"]) { --pg-tone: var(--color-warning); }
    :host([tone="danger"])  { --pg-tone: var(--color-danger); }
    :host([tone="info"])    { --pg-tone: var(--color-info); }

    /* ----- Linear ----- */
    .row { display: flex; align-items: center; gap: 10px; }
    .caption { font-size: var(--fs-xs); color: var(--color-text-muted); font-weight: var(--fw-medium); }
    .value { font-size: var(--fs-xs); color: var(--color-text-muted); font-variant-numeric: tabular-nums; margin-left: auto; }
    .bar {
      flex: 1;
      height: 8px; border-radius: 999px;
      background: color-mix(in srgb, var(--color-text) 10%, transparent);
      overflow: hidden;
      position: relative;
    }
    .bar > span {
      display: block; height: 100%;
      background: var(--pg-tone);
      transform-origin: left;
      transition: transform var(--dur-med) var(--ease);
      border-radius: 999px;
    }
    /* Indeterminate linear */
    :host([indeterminate]) .bar > span {
      transform: none !important;
      width: 40%;
      animation: pg-slide 1.2s var(--ease) infinite;
    }
    @keyframes pg-slide {
      0%   { margin-left: -40%; }
      100% { margin-left: 100%; }
    }

    /* ----- Circle ----- */
    .ring { position: relative; display: inline-grid; place-items: center; }
    svg { transform: rotate(-90deg); }
    circle { fill: none; }
    .track  { stroke: color-mix(in srgb, var(--color-text) 12%, transparent); }
    .prog   { stroke: var(--pg-tone); stroke-linecap: round;
              transition: stroke-dashoffset var(--dur-med) var(--ease); }
    .center {
      position: absolute; inset: 0;
      display: grid; place-items: center; gap: 2px;
      font-variant-numeric: tabular-nums;
    }
    .center .n { font-weight: var(--fw-semibold); font-size: var(--fs-md); }
    .center .lbl { font-size: 10px; color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.04em; }
    :host([indeterminate]) .prog { animation: pg-spin 1.2s linear infinite; }
    @keyframes pg-spin {
      0%   { stroke-dashoffset: 0; }
      100% { stroke-dashoffset: 1000; }
    }
  `;

  constructor() {
    super();
    this.value = 0;
    this.max = 100;
    this.shape = 'linear';
    this.size = 56;
    this.tone = 'brand';
    this.label = '';
    this['show-value'] = false;
    this.indeterminate = false;
  }

  #pct() {
    const v = Math.max(0, Math.min(this.max || 100, this.value || 0));
    return (v / (this.max || 100)) * 100;
  }

  updated() {
    this.setAttribute('role', 'progressbar');
    this.setAttribute('aria-valuemin', '0');
    this.setAttribute('aria-valuemax', String(this.max));
    if (!this.indeterminate) this.setAttribute('aria-valuenow', String(this.value));
    else this.removeAttribute('aria-valuenow');
  }

  render() {
    if (this.shape === 'circle') return this.#renderCircle();
    return this.#renderLinear();
  }

  #renderLinear() {
    const p = this.#pct();
    return html`
      ${this.label || this['show-value']
        ? html`<div class="row" style="margin-bottom: 6px;">
            ${this.label ? html`<span class="caption">${this.label}</span>` : nothing}
            ${this['show-value']
              ? html`<span class="value">${this.value} / ${this.max}</span>`
              : nothing}
          </div>`
        : nothing}
      <div class="bar">
        <span style="transform: scaleX(${p/100})"></span>
      </div>
    `;
  }

  #renderCircle() {
    const sz = this.size || 56;
    const stroke = Math.max(4, Math.round(sz / 10));
    const r = (sz - stroke) / 2;
    const C = 2 * Math.PI * r;
    const p = this.#pct();
    const dash = this.indeterminate ? `${C * 0.25} ${C * 0.75}` : `${(p/100) * C} ${C}`;
    return html`
      <div class="ring" style="width:${sz}px; height:${sz}px;">
        <svg width=${sz} height=${sz}>
          <circle class="track" cx=${sz/2} cy=${sz/2} r=${r} stroke-width=${stroke}></circle>
          <circle class="prog"  cx=${sz/2} cy=${sz/2} r=${r} stroke-width=${stroke}
                  stroke-dasharray=${dash}></circle>
        </svg>
        <div class="center">
          ${this.indeterminate
            ? html`<span class="n">…</span>`
            : html`<span class="n">${Math.round(p)}%</span>`}
          ${this.label ? html`<span class="lbl">${this.label}</span>` : nothing}
        </div>
      </div>
    `;
  }
}
customElements.define('ui-progress', UIProgress);
