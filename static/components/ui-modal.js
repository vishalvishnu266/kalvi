import { BaseElement, html, css } from './base.js';

class UIModal extends BaseElement {
  static properties = {
    label: { type: String },
    open: { type: Boolean, reflect: true },
    size: { type: String, reflect: true },
  };

  static styles = css`
    :host {
      display: none;
      position: fixed;
      inset: 0;
      z-index: 1000;
      align-items: center;
      justify-content: center;
      padding: var(--space-6);
    }
    :host([open]) { display: flex; }

    .backdrop {
      position: absolute;
      inset: 0;
      background: var(--color-scrim);
      backdrop-filter: blur(4px);
    }

    .panel {
      position: relative;
      background: var(--color-surface);
      border-radius: var(--radius-xl);
      box-shadow: var(--shadow-lg);
      width: 100%;
      max-width: 500px;
      display: flex;
      flex-direction: column;
      animation: modal-in 0.3s var(--ease);
    }
    @keyframes modal-in {
      from { opacity: 0; transform: scale(0.95) translateY(10px); }
      to { opacity: 1; transform: scale(1) translateY(0); }
    }

    .header {
      padding: var(--space-5);
      border-bottom: 1px solid var(--color-border);
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    .title { font-size: var(--fs-lg); font-weight: var(--fw-bold); }
    .close { cursor: pointer; border: none; background: none; color: var(--color-text-subtle); }

    .body { padding: var(--space-6); overflow-y: auto; }
  `;

  render() {
    return html`
      <div class="backdrop" @click="${() => this.open = false}"></div>
      <div class="panel">
        <div class="header">
          <div class="title">${this.label}</div>
          <button class="close" @click="${() => this.open = false}">
            <ui-icon name="x-lg" size="20"></ui-icon>
          </button>
        </div>
        <div class="body">
          <slot></slot>
        </div>
      </div>
    `;
  }
}

customElements.define('ui-modal', UIModal);
