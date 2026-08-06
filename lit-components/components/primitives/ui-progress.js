import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-progress
 *   value="65"           → current progress (0 → max)
 *   max="100"            → maximum value (default 100)
 *   tone="brand"         → neutral (default) | brand | success | warning | danger
 *   label="Uploading…"   → optional label rendered above the bar
 *   indeterminate>       → boolean; animate a moving pip instead of a filled bar
 * </ui-progress>
 *
 * Pure display. Determinate mode fills `value / max`. Indeterminate mode
 * animates a small pip across the track (used when % is unknown, e.g. a
 * request in flight).
 *
 * Emits nothing.
 */
class UIProgress extends LitBaseElement {
  static properties = {
    value:         { type: Number, reflect: true },
    max:           { type: Number, reflect: true },
    tone:          { type: String, reflect: true },
    label:         { type: String, reflect: true },
    indeterminate: { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: block;
      font-family: inherit;
      color: var(--color-text);
      font-size: var(--fs-sm);
    }
    .label {
      display: block;
      margin-bottom: var(--space-2);
      color: var(--color-text-muted);
    }
    .track {
      position: relative;
      width: 100%;
      height: 6px;
      background: var(--color-border);
      border-radius: 999px;
      overflow: hidden;
    }
    .fill {
      height: 100%;
      background: var(--tone-color, var(--color-primary, #4f46e5));
      border-radius: inherit;
      transition: width .18s ease;
    }
    /* Tone presets — resolved via a CSS variable so the same rule stays
       generic and future tones only need one more selector. */
    :host([tone="neutral"]) { --tone-color: var(--color-text-muted); }
    :host([tone="brand"])   { --tone-color: var(--color-primary, #4f46e5); }
    :host([tone="success"]) { --tone-color: var(--color-success, #16a34a); }
    :host([tone="warning"]) { --tone-color: var(--color-warning, #d97706); }
    :host([tone="danger"])  { --tone-color: var(--color-danger,  #dc2626); }

    /* Indeterminate: hide the fill, animate a pip. */
    :host([indeterminate]) .fill { display: none; }
    .pip {
      position: absolute;
      top: 0;
      height: 100%;
      width: 30%;
      background: var(--tone-color, var(--color-primary, #4f46e5));
      border-radius: inherit;
      animation: ui-progress-slide 1.2s ease-in-out infinite;
    }
    :host(:not([indeterminate])) .pip { display: none; }
    @keyframes ui-progress-slide {
      0%   { left: -30%; }
      100% { left: 100%; }
    }
  `;

  constructor() {
    super();
    this.value = 0;
    this.max   = 100;
    this.tone  = 'neutral';
    this.label = '';
    this.indeterminate = false;
  }

  render() {
    const pct = this.indeterminate
      ? 0
      : Math.max(0, Math.min(100, (Number(this.value) / Number(this.max)) * 100));
    return html`
      ${this.label ? html`<span class="label">${this.label}</span>` : ''}
      <div class="track"
           role="progressbar"
           aria-valuemin="0"
           aria-valuemax=${this.max}
           aria-valuenow=${this.indeterminate ? undefined : this.value}>
        <div class="fill" style=${`width: ${pct}%`}></div>
        <div class="pip"></div>
      </div>
    `;
  }
}

customElements.define('ui-progress', UIProgress);
