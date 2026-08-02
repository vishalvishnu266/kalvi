import { BaseElement, attr } from './base.js';

/**
 * <ui-modal id="m1" title="Add Student">
 *   <ui-input label="Name"></ui-input>
 *   <div slot="footer">
 *     <ui-button variant="secondary" data-close>Cancel</ui-button>
 *     <ui-button>Save</ui-button>
 *   </div>
 * </ui-modal>
 * document.getElementById('m1').open();
 */
class UIModal extends BaseElement {
  static styles = `
    :host { position: fixed; inset: 0; display: none; z-index: 2000; }
    :host([open]) { display: block; }
    .scrim {
      position: absolute; inset: 0;
      background: rgba(2, 6, 23, 0.55);
      backdrop-filter: blur(2px);
    }
    .panel {
      position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%);
      background: var(--color-surface);
      border-radius: var(--radius-xl);
      width: min(560px, calc(100% - 32px));
      max-height: 85vh;
      display: flex; flex-direction: column;
      box-shadow: var(--shadow-lg);
      overflow: hidden;
      animation: pop var(--dur-med) var(--ease);
    }
    @keyframes pop { from { transform: translate(-50%, -46%); opacity: 0; } to { transform: translate(-50%, -50%); opacity: 1; } }
    header { padding: var(--space-4) var(--space-5); border-bottom: 1px solid var(--color-border); display:flex; align-items:center; }
    header h3 { margin:0; font-size: var(--fs-lg); font-weight: var(--fw-semibold); flex: 1; }
    .close { background: none; border: 0; font-size: 1.2rem; color: var(--color-text-muted); cursor: pointer; }
    .body   { padding: var(--space-5); overflow: auto; }
    footer  { padding: var(--space-4) var(--space-5); border-top: 1px solid var(--color-border); display:flex; gap: var(--space-2); justify-content: flex-end; }

    @media (max-width: 640px) {
      .panel {
        left: 0; top: auto; bottom: 0; transform: none;
        width: 100%;
        border-radius: var(--radius-xl) var(--radius-xl) 0 0;
        max-height: 92vh;
        animation: slide var(--dur-med) var(--ease);
      }
      @keyframes slide { from { transform: translateY(30%); opacity: .7; } to { transform: none; opacity: 1; } }
    }
  `;
  render() {
    const title = attr(this, 'title');
    return `
      <div class="scrim" data-close></div>
      <div class="panel">
        <header><h3>${title}</h3><button class="close" data-close>✕</button></header>
        <div class="body"><slot></slot></div>
        <footer><slot name="footer"></slot></footer>
      </div>`;
  }
  afterRender() {
    this.shadowRoot.addEventListener('click', (e) => {
      if (e.target.matches('[data-close]')) this.close();
    });
    this.addEventListener('click', (e) => {
      if (e.target.closest('[data-close]')) this.close();
    });
  }
  open()  { this.setAttribute('open', ''); this.emit('open'); }
  close() { this.removeAttribute('open'); this.emit('close'); }
}
customElements.define('ui-modal', UIModal);
