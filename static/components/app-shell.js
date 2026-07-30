import { BaseElement, html, css } from './base.js';

class AppShell extends BaseElement {
  static properties = {
    brandName: { type: String, attribute: 'brand-name' },
    userName: { type: String, attribute: 'user-name' },
  };

  static styles = css`
    :host {
      display: grid;
      grid-template-columns: var(--sidebar-w) 1fr;
      height: 100vh;
      overflow: hidden;
    }

    .sidebar {
      background: var(--color-surface-alt);
      border-right: 1px solid var(--color-border);
      display: flex;
      flex-direction: column;
    }

    .brand {
      height: var(--topbar-h);
      padding: 0 var(--space-6);
      display: flex;
      align-items: center;
      font-weight: var(--fw-bold);
      font-size: var(--fs-lg);
      border-bottom: 1px solid var(--color-border);
    }

    .main {
      display: flex;
      flex-direction: column;
      background: var(--bg-gradient);
    }

    .topbar {
      height: var(--topbar-h);
      padding: 0 var(--space-6);
      background: var(--chrome-bg);
      backdrop-filter: var(--chrome-blur);
      border-bottom: 1px solid var(--chrome-border);
      display: flex;
      align-items: center;
      justify-content: space-between;
    }

    .content {
      flex: 1;
      padding: var(--space-8);
      overflow-y: auto;
    }

    @media (max-width: 768px) {
      :host { grid-template-columns: 1fr; }
      .sidebar { display: none; }
    }
  `;

  render() {
    return html`
      <div class="sidebar">
        <div class="brand">${this.brandName}</div>
        <slot name="sidebar"></slot>
      </div>
      <div class="main">
        <div class="topbar">
          <slot name="topbar"></slot>
          <div class="user-pill">${this.userName}</div>
        </div>
        <div class="content">
          <slot></slot>
        </div>
      </div>
    `;
  }
}

customElements.define('app-shell', AppShell);
