import { LitBaseElement, html, css, nothing } from './base.js';
import './ui-icon.js';

/**
 * <ui-ack-panel
 *   id="ack-fees-locked"
 *   tone="warning"
 *   placement="right"      | "left"
 *   icon="warning"
 *   title="Fees module is locked"
 *   message="Editing is disabled until…"
 *   ack-label="I understand"
 *   open>                   <!-- optional; open by default -->
 * </ui-ack-panel>
 *
 * A modal slide-in panel that carries a single message + an acknowledgment
 * button the user MUST press to dismiss. Purpose: blocking notices that
 * cannot be missed. NOT a general drawer — see <ui-drawer> for that.
 *
 * JS API:
 *   el.open()         // slide in
 *   el.close()        // slide out (rarely — user should press ACK)
 * Events:
 *   'ack'             // fired when the user clicks the ACK button
 *   'ack-panel:close' // fired after the close animation finishes
 */
class UIAckPanel extends LitBaseElement {
  static properties = {
    tone:        { type: String, reflect: true },  // info|warning|danger|success|brand|neutral
    placement:   { type: String, reflect: true },  // left|right
    icon:        { type: String, reflect: true },
    title:       { type: String, reflect: true },
    message:     { type: String, reflect: true },
    ackLabel:    { type: String, attribute: 'ack-label' },
    open:        { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      position: fixed; inset: 0;
      display: none;
      z-index: 10000;
      font: inherit;
    }
    :host([open]) { display: block; }

    .scrim {
      position: absolute; inset: 0;
      background: var(--color-scrim, rgba(0,0,0,.35));
      opacity: 0;
      transition: opacity var(--dur-med, 240ms) var(--ease, ease);
    }
    :host([open]) .scrim { opacity: 1; }

    .panel {
      position: absolute; top: 0; bottom: 0;
      width: min(420px, 92vw);
      display: flex; flex-direction: column;
      background: var(--color-surface, #fff);
      color: var(--color-text, #111);
      box-shadow: var(--shadow-lg, 0 20px 40px rgba(0,0,0,.2));
      transition: transform var(--dur-med, 240ms) var(--ease, cubic-bezier(.32,.72,0,1));
      border-left: 6px solid var(--accent, var(--color-info));
    }

    /* --- Placement --- */
    :host([placement="right"]) .panel { right: 0; transform: translateX(100%); border-left-width: 6px; border-right: 0; }
    :host([placement="left"])  .panel { left:  0; transform: translateX(-100%); border-right: 6px solid var(--accent, var(--color-info)); border-left: 0; }
    :host([open][placement="right"]) .panel,
    :host([open][placement="left"])  .panel { transform: translateX(0); }

    /* --- Tone accent --- */
    :host([tone="info"])    .panel { --accent: var(--color-info); }
    :host([tone="warning"]) .panel { --accent: var(--color-warning); }
    :host([tone="danger"])  .panel { --accent: var(--color-danger); }
    :host([tone="success"]) .panel { --accent: var(--color-success); }

    header {
      padding: var(--space-5, 20px) var(--space-5, 20px) var(--space-3, 12px);
      display: flex; gap: var(--space-3, 12px); align-items: flex-start;
    }
    header ui-icon { color: var(--accent, var(--color-info)); flex: 0 0 auto; margin-top: 2px; }
    .title { font-size: var(--fs-lg, 1.0625rem); font-weight: var(--fw-semibold, 600); margin: 0; }

    .body {
      padding: 0 var(--space-5, 20px);
      flex: 1 1 auto; overflow: auto;
      font-size: var(--fs-md, .9375rem);
      color: var(--color-text-muted, #475569);
      line-height: 1.5;
    }
    .body > ::slotted(*) { margin-top: var(--space-3, 12px); }
    .msg { margin: 0; }

    footer {
      padding: var(--space-5, 20px);
      display: flex; justify-content: flex-end;
      border-top: 1px solid var(--color-border);
    }
    button.ack {
      appearance: none; border: 0;
      background: var(--accent, var(--color-info));
      color: #fff;
      padding: 10px 18px;
      border-radius: var(--radius-md, 8px);
      font-weight: var(--fw-semibold, 600);
      cursor: pointer;
      transition: transform var(--dur-fast, 120ms) var(--ease), filter var(--dur-fast) var(--ease);
    }
    button.ack:hover  { filter: brightness(1.08); }
    button.ack:active { transform: translateY(1px); }
    button.ack:focus-visible {
      outline: 2px solid var(--color-primary-ring, rgba(10,132,255,.5));
      outline-offset: 2px;
    }
  `;

  constructor() {
    super();
    this.tone = 'info';
    this.placement = 'right';
    this.icon = '';
    this.title = '';
    this.message = '';
    this.ackLabel = 'OK';
    this.open = false;
  }

  /** Programmatic API. */
  openPanel()  { this.open = true;  this.#trapFocus(); }
  closePanel() {
    this.open = false;
    // Give the transition time to run, THEN fire the close event.
    setTimeout(() => this.dispatchEvent(new CustomEvent('ack-panel:close', { bubbles: true })), 260);
  }

  #onAck = () => {
    this.dispatchEvent(new CustomEvent('ack', { bubbles: true, composed: true }));
    this.closePanel();
  };

  // Move focus to the ACK button when the panel opens so keyboard users
  // can hit Enter immediately. Ignore scrim clicks — this is a blocking
  // acknowledgment, the user MUST press the button.
  #trapFocus() {
    // Wait one frame for the shadow DOM to render.
    requestAnimationFrame(() => {
      const btn = this.shadowRoot && this.shadowRoot.querySelector('button.ack');
      if (btn) btn.focus();
    });
  }

  updated(changed) {
    if (changed.has('open') && this.open) this.#trapFocus();
  }

  render() {
    const role = this.tone === 'danger' ? 'alertdialog' : 'dialog';
    return html`
      <div class="scrim" part="scrim"></div>
      <div class="panel" part="panel" role=${role}
           aria-modal="true"
           aria-labelledby=${this.title ? 'ap-title' : undefined}
           aria-describedby=${this.message ? 'ap-msg' : undefined}>
        <header>
          ${this.icon ? html`<ui-icon name=${this.icon} size="22"></ui-icon>` : nothing}
          ${this.title ? html`<h2 id="ap-title" class="title">${this.title}</h2>` : nothing}
        </header>
        <div class="body">
          ${this.message ? html`<p id="ap-msg" class="msg">${this.message}</p>` : nothing}
          <slot></slot>
        </div>
        <footer>
          <button class="ack" type="button" @click=${this.#onAck}>${this.ackLabel || 'OK'}</button>
        </footer>
      </div>
    `;
  }
}
customElements.define('ui-ack-panel', UIAckPanel);

// Alias the friendlier method names for external callers so both
//   el.open() / el.close()
// and
//   el.openPanel() / el.closePanel()
// work. `open` remains the *property* (Boolean attribute), unchanged.
UIAckPanel.prototype.close = UIAckPanel.prototype.closePanel;
Object.defineProperty(UIAckPanel.prototype, 'openNow', {
  value: function () { this.openPanel(); }
});
