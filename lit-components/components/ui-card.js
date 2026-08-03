import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-card title="Attendance" subtitle="This week" padded>
 *   <div slot="actions"><ui-button size="sm" variant="ghost">See all</ui-button></div>
 *   ...body...
 * </ui-card>
 *
 * DSL surface:
 *   - title    : optional header title
 *   - subtitle : optional header subtitle
 *   - padded         : boolean, add padding to body
 *   - flush          : boolean, hide the header border
 *   - sticky-friendly: boolean, drop the default overflow:hidden clipping
 *                      so a <ui-form sticky> child can pin against the
 *                      viewport instead of being trapped inside the card
 *
 * Slots: default (body), "actions" (top-right of header).
 */
class UICard extends LitBaseElement {
  static properties = {
    title:           { type: String,  reflect: true },
    subtitle:        { type: String,  reflect: true },
    padded:          { type: Boolean, reflect: true },
    flush:           { type: Boolean, reflect: true },
    stickyFriendly:  { type: Boolean, reflect: true, attribute: 'sticky-friendly' },
  };

  static styles = css`
    :host { display: block; }
    .card {
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      box-shadow: var(--shadow-sm);
      overflow: hidden;
    }
    /* Opt-out: cards hosting a <ui-form sticky> must NOT clip their
       overflow, otherwise the sticky action bar pins against the card
       box (a few hundred pixels tall) instead of the viewport. */
    :host([sticky-friendly]) .card { overflow: visible; }
    header {
      display: flex; align-items: center; gap: var(--space-3);
      padding: var(--space-4) var(--space-5);
      border-bottom: 1px solid var(--color-border);
    }
    header .titles { flex: 1; min-width: 0; }
    header h3 {
      margin: 0; font-size: var(--fs-md); font-weight: var(--fw-semibold);
      color: var(--color-text);
    }
    header p {
      margin: 2px 0 0; font-size: var(--fs-xs); color: var(--color-text-muted);
    }
    .body { padding: 0; }
    :host([padded]) .body { padding: var(--space-5); }
    :host([flush]) header { border-bottom: 0; padding-bottom: 0; }
  `;

  constructor() {
    super();
    this.title = '';
    this.subtitle = '';
    this.padded = false;
    this.flush = false;
    this.stickyFriendly = false;
  }

  render() {
    const hasHeader = this.title || this.subtitle;
    return html`
      <div class="card" part="card">
        ${hasHeader
          ? html`
              <header>
                <div class="titles">
                  ${this.title    ? html`<h3>${this.title}</h3>`    : nothing}
                  ${this.subtitle ? html`<p>${this.subtitle}</p>`   : nothing}
                </div>
                <slot name="actions"></slot>
              </header>`
          : nothing}
        <div class="body"><slot></slot></div>
      </div>
    `;
  }
}
customElements.define('ui-card', UICard);
