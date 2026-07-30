import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-modal id="m1" title="Add Student">
 *   <ui-input label="Name"></ui-input>
 *   <div slot="footer">
 *     <ui-button variant="secondary" data-close>Cancel</ui-button>
 *     <ui-button>Save</ui-button>
 *   </div>
 * </ui-modal>
 *
 * JS API: modal.open() / modal.close()
 * Attribute API: <ui-modal open>
 * Emits: "ui-open", "ui-close" (bubbling).
 */
class UIModal extends LitBaseElement {
  static properties = {
    title: { type: String, reflect: true },
    open:  { type: Boolean, reflect: true },
  };

  static styles = css`
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
    @keyframes pop {
      from { transform: translate(-50%, -46%); opacity: 0; }
      to   { transform: translate(-50%, -50%); opacity: 1; }
    }
    header {
      padding: var(--space-4) var(--space-5);
      border-bottom: 1px solid var(--color-border);
      display:flex; align-items:center;
    }
    header h3 {
      margin:0; font-size: var(--fs-lg);
      font-weight: var(--fw-semibold); flex: 1;
    }
    .close {
      background: none; border: 0; font-size: 1.2rem;
      color: var(--color-text-muted); cursor: pointer;
    }
    .body   { padding: var(--space-5); overflow: auto; }
    footer  {
      padding: var(--space-4) var(--space-5);
      border-top: 1px solid var(--color-border);
      display:flex; gap: var(--space-2); justify-content: flex-end;
    }

    @media (max-width: 640px) {
      .panel {
        left: 0; top: auto; bottom: 0; transform: none;
        width: 100%;
        border-radius: var(--radius-xl) var(--radius-xl) 0 0;
        max-height: 92vh;
        animation: slide var(--dur-med) var(--ease);
      }
      @keyframes slide {
        from { transform: translateY(30%); opacity: .7; }
        to   { transform: none; opacity: 1; }
      }
    }
  `;

  constructor() {
    super();
    this.title = '';
    this.open = false;
  }

  #onShadowClick = (e) => {
    if (e.target.matches?.('[data-close]') || e.composedPath().some(el => el?.matches?.('[data-close]'))) {
      this.close();
    }
  };

  firstUpdated() {
    this.renderRoot.addEventListener('click', this.#onShadowClick);
    // Also handle [data-close] elements slotted from the light DOM (e.g. footer buttons)
    this.addEventListener('click', (e) => {
      if (e.target.closest('[data-close]')) this.close();
    });
  }

  openModal()  { this.open = true;  this.emit('ui-open'); }
  closeModal() { this.open = false; this.emit('ui-close'); }
  // Back-compat aliases for the vanilla API used in the previous kit.
  open2() { this.openModal(); }
  close() { this.closeModal(); }

  render() {
    return html`
      <div class="scrim" data-close></div>
      <div class="panel">
        <header>
          <h3>${this.title}</h3>
          <button class="close" data-close>✕</button>
        </header>
        <div class="body"><slot></slot></div>
        <footer><slot name="footer"></slot></footer>
      </div>
    `;
  }
}
customElements.define('ui-modal', UIModal);
