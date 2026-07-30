import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Reusable slide-in drawer/side-panel.
 *
 * <ui-drawer id="d" placement="right" title="Student profile">
 *   <p>Body content…</p>
 *   <div slot="footer">
 *     <ui-button variant="secondary" data-close>Cancel</ui-button>
 *     <ui-button>Save</ui-button>
 *   </div>
 * </ui-drawer>
 *
 * DSL surface:
 *   - title     : header title
 *   - open      : boolean (attribute-controlled state)
 *   - placement : "left" | "right" (default) | "bottom"
 *   - size      : "sm" (320px) | "md" (420px, default) | "lg" (600px) | "xl" (760px)
 *
 * Programmatic: drawer.openDrawer(), drawer.closeDrawer().
 * Also honours any child element with [data-close].
 * Emits `ui-open` / `ui-close`.
 */
class UIDrawer extends LitBaseElement {
  static properties = {
    title:     { type: String,  reflect: true },
    open:      { type: Boolean, reflect: true },
    placement: { type: String,  reflect: true },
    size:      { type: String,  reflect: true },
  };

  static styles = css`
    :host { position: fixed; inset: 0; display: none; z-index: 3000; }
    :host([open]) { display: block; }

    .scrim {
      position: absolute; inset: 0;
      background: var(--color-scrim);
      opacity: 0;
      transition: opacity var(--dur-med) var(--ease);
    }
    :host([open]) .scrim { opacity: 1; }

    .panel {
      position: absolute;
      background: var(--color-surface);
      box-shadow: var(--shadow-lg);
      display: flex; flex-direction: column;
      transition: transform var(--dur-med) var(--ease);
    }

    /* Placements */
    :host([placement="right"]) .panel,
    :host(:not([placement])) .panel {
      top: 0; right: 0; bottom: 0;
      width: var(--dr-sz, 420px); max-width: 96vw;
      transform: translateX(100%);
      border-left: 1px solid var(--color-border);
    }
    :host([placement="right"][open]) .panel,
    :host(:not([placement])[open]) .panel { transform: translateX(0); }

    :host([placement="left"]) .panel {
      top: 0; left: 0; bottom: 0;
      width: var(--dr-sz, 420px); max-width: 96vw;
      transform: translateX(-100%);
      border-right: 1px solid var(--color-border);
    }
    :host([placement="left"][open]) .panel { transform: translateX(0); }

    :host([placement="bottom"]) .panel {
      left: 0; right: 0; bottom: 0;
      max-height: 92vh;
      transform: translateY(100%);
      border-top: 1px solid var(--color-border);
      border-radius: var(--radius-xl) var(--radius-xl) 0 0;
    }
    :host([placement="bottom"][open]) .panel { transform: translateY(0); }

    /* Sizes */
    :host([size="sm"]) .panel { --dr-sz: 320px; }
    :host([size="md"]) .panel { --dr-sz: 420px; }
    :host([size="lg"]) .panel { --dr-sz: 600px; }
    :host([size="xl"]) .panel { --dr-sz: 760px; }

    header {
      display: flex; align-items: center;
      padding: 14px 16px;
      border-bottom: 1px solid var(--color-border);
      flex: 0 0 auto;
    }
    header h3 {
      margin: 0; flex: 1;
      font-size: var(--fs-md); font-weight: var(--fw-semibold);
      color: var(--color-text);
    }
    .close {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-text-muted); width: 28px; height: 28px;
      border-radius: 6px; display: grid; place-items: center;
    }
    .close:hover {
      background: color-mix(in srgb, var(--color-text) 8%, transparent);
      color: var(--color-text);
    }
    .body {
      padding: 16px;
      overflow: auto;
      flex: 1;
    }
    footer {
      display: flex; gap: 8px; justify-content: flex-end;
      padding: 12px 16px;
      border-top: 1px solid var(--color-border);
      flex: 0 0 auto;
    }
    footer:empty { display: none; }
  `;

  constructor() {
    super();
    this.title = '';
    this.open = false;
    this.placement = 'right';
    this.size = 'md';
  }

  connectedCallback() {
    super.connectedCallback();
    this.addEventListener('click', (e) => {
      if (e.target.closest('[data-close]')) this.closeDrawer();
    });
    document.addEventListener('keydown', this._esc = (e) => {
      if (e.key === 'Escape' && this.open) this.closeDrawer();
    });
  }
  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener('keydown', this._esc);
  }

  openDrawer()  { this.open = true;  this.emit('ui-open'); }
  closeDrawer() { this.open = false; this.emit('ui-close'); }

  render() {
    return html`
      <div class="scrim" data-close></div>
      <div class="panel">
        <header>
          <h3>${this.title}</h3>
          <button class="close" data-close title="Close">
            <ui-icon name="x" size="16"></ui-icon>
          </button>
        </header>
        <div class="body"><slot></slot></div>
        <footer><slot name="footer"></slot></footer>
      </div>
    `;
  }
}
customElements.define('ui-drawer', UIDrawer);
