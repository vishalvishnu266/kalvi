import { LitBaseElement, html, css, nothing } from './base.js';
import './ui-icon.js';

/**
 * <ui-alert tone="warning" icon="warning"
 *           title="Read-only mode"
 *           message="Your role doesn't allow editing fees."
 *           dismissible>
 * </ui-alert>
 *
 * Persistent page-level notice. Sits at the top of a page and stays until
 * dismissed or the underlying condition is resolved.
 *
 * NOT for cross-field form validation — use <ui-form-banner> for that.
 * NOT for transient feedback (save success, etc.) — use <ui-toast>.
 */
class UIAlert extends LitBaseElement {
  static properties = {
    tone:        { type: String, reflect: true },
    icon:        { type: String, reflect: true },
    title:       { type: String, reflect: true },
    message:     { type: String, reflect: true },
    dismissible: { type: Boolean, reflect: true },
  };

  static styles = css`
    :host { display: block; box-sizing: border-box; width: 100%; margin-bottom: var(--space-4); }
    .box {
      display: flex; gap: var(--space-3); align-items: flex-start;
      padding: var(--space-4);
      border: 1px solid var(--border, var(--color-border));
      background: var(--bg, var(--color-surface));
      color: var(--fg, var(--color-text));
      border-radius: var(--radius-md);
    }
    .body { flex: 1; min-width: 0; }
    .title { font-weight: var(--fw-semibold); margin: 0 0 4px; font-size: var(--fs-sm); }
    .msg   { margin: 0; font-size: var(--fs-sm); color: var(--fg-muted, var(--color-text-muted)); }
    button.close {
      background: transparent; border: 0; cursor: pointer; padding: 4px 8px;
      color: var(--fg-muted, var(--color-text-muted)); font-size: 18px; line-height: 1;
      border-radius: var(--radius-sm);
    }
    button.close:hover { background: var(--color-surface-hover, rgba(0,0,0,.05)); }

    :host([tone="danger"])  .box { --bg: var(--color-danger-soft,  #fee2e2);
                                    --border: var(--color-danger, #dc2626);
                                    --fg: var(--color-danger-strong, #991b1b);
                                    --fg-muted: var(--color-danger, #991b1b); }
    :host([tone="warning"]) .box { --bg: var(--color-warning-soft, #fef3c7);
                                    --border: var(--color-warning, #f59e0b);
                                    --fg: var(--color-warning-strong, #78350f);
                                    --fg-muted: var(--color-warning, #78350f); }
    :host([tone="success"]) .box { --bg: var(--color-success-soft, #dcfce7);
                                    --border: var(--color-success, #16a34a);
                                    --fg: var(--color-success-strong, #14532d);
                                    --fg-muted: var(--color-success, #14532d); }
    :host([tone="info"])    .box { --bg: var(--color-info-soft, #dbeafe);
                                    --border: var(--color-info, #2563eb);
                                    --fg: var(--color-info-strong, #1e3a8a);
                                    --fg-muted: var(--color-info, #1e3a8a); }
  `;

  constructor() {
    super();
    this.tone = 'info';
    this.icon = '';
    this.title = '';
    this.message = '';
    this.dismissible = false;
  }

  #dismiss = () => { this.remove(); };

  render() {
    return html`
      <div class="box" role=${this.tone === 'danger' ? 'alert' : 'status'} aria-live=${this.tone === 'danger' ? 'assertive' : 'polite'}>
        ${this.icon ? html`<ui-icon name=${this.icon} size="20"></ui-icon>` : nothing}
        <div class="body">
          ${this.title ? html`<p class="title">${this.title}</p>` : nothing}
          ${this.message ? html`<p class="msg">${this.message}</p>` : nothing}
          <slot></slot>
        </div>
        ${this.dismissible
          ? html`<button class="close" @click=${this.#dismiss} aria-label="Dismiss">×</button>`
          : nothing}
      </div>
    `;
  }
}
customElements.define('ui-alert', UIAlert);
