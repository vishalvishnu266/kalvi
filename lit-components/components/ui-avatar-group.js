import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-avatar-group max="3">
 *   <ui-avatar name="Aarav K."></ui-avatar>
 *   <ui-avatar name="Meera S."></ui-avatar>
 *   <ui-avatar name="Rohan P."></ui-avatar>
 *   <ui-avatar name="Diya V."></ui-avatar>
 *   <ui-avatar name="Ishaan T."></ui-avatar>
 * </ui-avatar-group>
 *
 * DSL surface:
 *   - max  : how many avatars to show before collapsing into "+N"
 *   - size : passes through to child <ui-avatar> defaults
 */
class UIAvatarGroup extends LitBaseElement {
  static properties = {
    max:  { type: Number, reflect: true },
    size: { type: String, reflect: true },
  };

  static styles = css`
    :host { display: inline-flex; align-items: center; }
    .stack { display: inline-flex; align-items: center; }
    /* Overlap slotted avatars using negative margins. */
    ::slotted(ui-avatar) {
      margin-left: -10px;
      transition: transform var(--dur-fast) var(--ease);
    }
    ::slotted(ui-avatar:first-child) { margin-left: 0; }
    ::slotted(ui-avatar:hover)       { transform: translateY(-2px); z-index: 1; }

    .more {
      margin-left: -10px;
      display: inline-grid; place-items: center;
      width: var(--group-sz, 40px); height: var(--group-sz, 40px);
      border-radius: 50%;
      background: var(--color-surface-alt);
      color: var(--color-text-muted);
      border: 2px solid var(--color-surface);
      font-size: calc(var(--group-sz, 40px) * .32);
      font-weight: var(--fw-semibold);
      font-variant-numeric: tabular-nums;
    }
    :host([size="sm"]) .more { --group-sz: 28px; }
    :host([size="lg"]) .more { --group-sz: 56px; }
    :host([size="xl"]) .more { --group-sz: 80px; }
  `;

  constructor() {
    super();
    this.max = 4;
    this.size = 'md';
  }

  #applyOverflow = () => {
    const items = [...this.querySelectorAll(':scope > ui-avatar')];
    items.forEach((el, i) => {
      el.style.display = (i >= this.max) ? 'none' : '';
      if (this.size) el.setAttribute('size', this.size);
    });
    this._overflow = Math.max(0, items.length - this.max);
    this.requestUpdate();
  };

  connectedCallback() {
    super.connectedCallback();
    this._mo = new MutationObserver(() => this.#applyOverflow());
    this._mo.observe(this, { childList: true, subtree: false });
    queueMicrotask(() => this.#applyOverflow());
  }
  disconnectedCallback() { super.disconnectedCallback(); this._mo?.disconnect(); }
  updated(changed) {
    if (changed.has('max') || changed.has('size')) this.#applyOverflow();
  }

  render() {
    return html`
      <div class="stack">
        <slot></slot>
        ${this._overflow > 0
          ? html`<span class="more" title="${this._overflow} more">+${this._overflow}</span>`
          : nothing}
      </div>
    `;
  }
}
customElements.define('ui-avatar-group', UIAvatarGroup);
