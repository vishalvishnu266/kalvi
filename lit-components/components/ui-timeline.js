import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Vertical activity timeline.
 *
 * <ui-timeline>
 *   <ui-timeline-item icon="check" tone="success" time="10:24 AM">
 *     <strong>Fees paid</strong>
 *     <div>Invoice #INV-1042 · ₹4,500</div>
 *   </ui-timeline-item>
 *   <ui-timeline-item icon="edit" time="Yesterday">
 *     <strong>Profile updated</strong>
 *   </ui-timeline-item>
 * </ui-timeline>
 *
 * `ui-timeline-item` DSL:
 *   - icon : name for <ui-icon>
 *   - tone : "brand" (default) | "success" | "warning" | "danger" | "info" | "muted"
 *   - time : right-side timestamp label
 */
class UITimeline extends LitBaseElement {
  static styles = css`
    :host { display: block; position: relative; padding-left: 6px; }
  `;
  render() { return html`<slot></slot>`; }
}
customElements.define('ui-timeline', UITimeline);

class UITimelineItem extends LitBaseElement {
  static properties = {
    icon: { type: String, reflect: true },
    tone: { type: String, reflect: true },
    time: { type: String, reflect: true },
  };

  static styles = css`
    :host {
      display: grid;
      grid-template-columns: 28px 1fr auto;
      gap: 12px;
      position: relative;
      padding: 4px 0 18px;
      --tone: var(--color-primary);
    }
    :host([tone="success"]) { --tone: var(--color-success); }
    :host([tone="warning"]) { --tone: var(--color-warning); }
    :host([tone="danger"])  { --tone: var(--color-danger); }
    :host([tone="info"])    { --tone: var(--color-info); }
    :host([tone="muted"])   { --tone: var(--color-text-subtle); }

    .dot {
      grid-column: 1;
      width: 28px; height: 28px; border-radius: 50%;
      background: color-mix(in srgb, var(--tone) 15%, transparent);
      color: var(--tone);
      display: grid; place-items: center;
      z-index: 1;
    }
    :host::before {
      content: '';
      position: absolute;
      left: 13px; top: 32px; bottom: -4px;
      width: 2px;
      background: var(--color-border);
    }
    :host(:last-of-type)::before,
    :host(:last-child)::before { display: none; }
    .body {
      grid-column: 2;
      font-size: var(--fs-sm); color: var(--color-text);
      min-width: 0;
      align-self: center;
    }
    .body > * { display: block; }
    .body strong { font-size: var(--fs-sm); font-weight: var(--fw-semibold); }
    .time {
      grid-column: 3;
      font-size: var(--fs-xs);
      color: var(--color-text-muted);
      font-variant-numeric: tabular-nums;
      align-self: center;
      white-space: nowrap;
    }
  `;

  constructor() {
    super();
    this.icon = 'activity';
    this.tone = 'brand';
    this.time = '';
  }

  render() {
    return html`
      <span class="dot"><ui-icon name=${this.icon} size="14"></ui-icon></span>
      <div class="body"><slot></slot></div>
      ${this.time ? html`<span class="time">${this.time}</span>` : nothing}
    `;
  }
}
customElements.define('ui-timeline-item', UITimelineItem);
