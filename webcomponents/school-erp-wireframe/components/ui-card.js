import { BaseElement, attr } from './base.js';

/**
 * <ui-card title="Attendance" subtitle="This week" padded>
 *   <div slot="actions"><ui-button size="sm" variant="ghost">See all</ui-button></div>
 *   ...body...
 * </ui-card>
 */
class UICard extends BaseElement {
  static styles = `
    :host { display: block; }
    .card {
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      box-shadow: var(--shadow-sm);
      overflow: hidden;
    }
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
  render() {
    const title = attr(this, 'title');
    const subtitle = attr(this, 'subtitle');
    return `
      <div class="card" part="card">
        ${title || subtitle ? `
          <header>
            <div class="titles">
              ${title ? `<h3>${title}</h3>` : ''}
              ${subtitle ? `<p>${subtitle}</p>` : ''}
            </div>
            <slot name="actions"></slot>
          </header>` : ''}
        <div class="body"><slot></slot></div>
      </div>`;
  }
}
customElements.define('ui-card', UICard);
