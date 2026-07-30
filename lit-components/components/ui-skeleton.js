import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-skeleton></ui-skeleton>                     – 1 line block
 * <ui-skeleton width="60%"></ui-skeleton>
 * <ui-skeleton shape="circle" width="40" height="40"></ui-skeleton>
 * <ui-skeleton shape="rect" width="100%" height="120"></ui-skeleton>
 * <ui-skeleton lines="3"></ui-skeleton>            – multi-line paragraph
 *
 * DSL surface:
 *   - shape  : "line" (default) | "rect" | "circle"
 *   - width  : CSS length (e.g. "60%", "100px") – default "100%"
 *   - height : CSS length – default depends on shape
 *   - lines  : integer, render N stacked lines (only when shape="line")
 */
class UISkeleton extends LitBaseElement {
  static properties = {
    shape:  { type: String, reflect: true },
    width:  { type: String, reflect: true },
    height: { type: String, reflect: true },
    lines:  { type: Number, reflect: true },
  };

  static styles = css`
    :host { display: block; --sk-radius: var(--radius-sm); }
    :host([shape="rect"])   { --sk-radius: var(--radius-md); }
    :host([shape="circle"]) { --sk-radius: 50%; }

    .sk {
      display: block;
      background: linear-gradient(
        90deg,
        color-mix(in srgb, var(--color-text) 6%, transparent) 0%,
        color-mix(in srgb, var(--color-text) 12%, transparent) 40%,
        color-mix(in srgb, var(--color-text) 6%, transparent) 80%
      );
      background-size: 200% 100%;
      border-radius: var(--sk-radius);
      animation: sk-shine 1.4s ease-in-out infinite;
    }
    .stack > .sk + .sk { margin-top: 8px; }
    @keyframes sk-shine {
      0%   { background-position:  100% 0; }
      100% { background-position: -100% 0; }
    }
    @media (prefers-reduced-motion: reduce) {
      .sk { animation: none; }
    }
  `;

  constructor() {
    super();
    this.shape = 'line';
    this.width = '';
    this.height = '';
    this.lines = 1;
  }

  #style(idx) {
    const width = this.width || (this.shape === 'circle' ? '32px' : '100%');
    const height = this.height || (this.shape === 'circle' ? width : (this.shape === 'rect' ? '80px' : '14px'));
    // last line: 60% width for a natural paragraph shape
    const w = (idx === this.lines - 1 && this.lines > 1) ? '60%' : width;
    return `width:${w}; height:${height};`;
  }

  render() {
    if (this.shape === 'line' && this.lines > 1) {
      return html`<div class="stack">
        ${Array.from({length: this.lines}, (_, i) => html`
          <span class="sk" style=${this.#style(i)}></span>`)}
      </div>`;
    }
    return html`<span class="sk" style=${this.#style(0)}></span>`;
  }
}
customElements.define('ui-skeleton', UISkeleton);
