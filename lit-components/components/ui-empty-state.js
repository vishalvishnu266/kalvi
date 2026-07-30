import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-empty-state
 *   icon="users"
 *   title="No students yet"
 *   description="Add your first student to see attendance and grades here.">
 *   <ui-button slot="actions" icon="plus">Add student</ui-button>
 * </ui-empty-state>
 *
 * Slots:
 *   - "media"   : override the built-in icon with anything (illustration, image)
 *   - "actions" : one or more buttons under the description
 *   - default   : extra body content between description and actions
 */
class UIEmptyState extends LitBaseElement {
  static properties = {
    icon:        { type: String, reflect: true },
    title:       { type: String, reflect: true },
    description: { type: String, reflect: true },
    compact:     { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: block;
      text-align: center;
      padding: var(--space-8) var(--space-5);
      color: var(--color-text);
    }
    :host([compact]) { padding: var(--space-5); }
    .media {
      width: 64px; height: 64px; border-radius: 50%;
      background: var(--color-primary-soft);
      color: var(--color-primary);
      display: inline-grid; place-items: center;
      margin-bottom: var(--space-4);
    }
    :host([compact]) .media { width: 44px; height: 44px; margin-bottom: var(--space-3); }
    h3 {
      margin: 0 0 var(--space-2);
      font-size: var(--fs-lg); font-weight: var(--fw-semibold);
      letter-spacing: -0.01em;
    }
    p {
      margin: 0 auto;
      max-width: 44ch;
      color: var(--color-text-muted); font-size: var(--fs-sm);
    }
    ::slotted([slot="actions"]) { margin-top: var(--space-4); }
    .actions {
      margin-top: var(--space-4);
      display: flex; justify-content: center; gap: var(--space-2); flex-wrap: wrap;
    }
  `;

  constructor() {
    super();
    this.icon = 'library';
    this.title = 'Nothing here yet';
    this.description = '';
    this.compact = false;
  }

  render() {
    return html`
      <div class="media">
        <slot name="media">
          <ui-icon name=${this.icon} size=${this.compact ? '22' : '30'}></ui-icon>
        </slot>
      </div>
      <h3>${this.title}</h3>
      ${this.description ? html`<p>${this.description}</p>` : nothing}
      <slot></slot>
      <div class="actions"><slot name="actions"></slot></div>
    `;
  }
}
customElements.define('ui-empty-state', UIEmptyState);
