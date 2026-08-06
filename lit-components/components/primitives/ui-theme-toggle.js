import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-theme-toggle></ui-theme-toggle>
 *
 * A one-click light/dark theme switcher. Zero configuration.
 *
 * On click:
 *   1. Flips `document.documentElement.dataset.theme` between 'light' and 'dark'.
 *   2. Persists to `localStorage['erp.theme']` — the same key `core.js`
 *      reads on load, so refresh keeps the choice.
 *   3. Emits `ui-theme-change` with `{ theme }`.
 *
 * Renders the sun/moon icon of the *opposite* theme (i.e. it shows the
 * icon that describes what a click will take you to — same as macOS,
 * iOS, GitHub, etc.).
 *
 * Uses the shared `<ui-icon>` primitive so it inherits the same icon
 * catalogue and sizing. No fetching, no scripts — purely CSS + one
 * click handler.
 */
class UIThemeToggle extends LitBaseElement {
  static properties = {
    // Reflected so CSS can `:host([theme="dark"])` style if it needs to.
    // Not intended as a public prop — it mirrors the <html> attribute.
    theme: { type: String, reflect: true },
  };

  static styles = css`
    :host {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      width: 36px;
      height: 36px;
      border-radius: 8px;
      cursor: pointer;
      color: var(--color-text);
      background: transparent;
      transition: background .12s ease;
      user-select: none;
      outline: none;
    }
    :host(:hover) { background: var(--color-hover, rgba(0,0,0,.06)); }
    :host(:focus-visible) {
      box-shadow: 0 0 0 3px var(--color-focus, rgba(79,70,229,.35));
    }
    ui-icon { display: block; }
  `;

  constructor() {
    super();
    this.theme = 'light';
  }

  connectedCallback() {
    super.connectedCallback();
    // Sync from whatever's on <html> at mount time (core.js already
    // restored the persisted value pre-paint).
    this.theme = document.documentElement.dataset.theme || 'light';
    this.tabIndex = 0;
    this.setAttribute('role', 'button');
    this.setAttribute('aria-label', 'Toggle theme');
    this.addEventListener('click',   this.#toggle);
    this.addEventListener('keydown', this.#onKey);
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    this.removeEventListener('click',   this.#toggle);
    this.removeEventListener('keydown', this.#onKey);
  }

  #onKey = (e) => {
    if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); this.#toggle(); }
  };

  #toggle = () => {
    const next = this.theme === 'dark' ? 'light' : 'dark';
    this.theme = next;
    document.documentElement.dataset.theme = next;
    try { localStorage.setItem('erp.theme', next); } catch { /* ignore quota errors */ }
    this.emit('ui-theme-change', { theme: next });
  };

  render() {
    // Show the icon of the target theme, matching common convention.
    // Dark theme → show sun (click to go light); Light theme → show moon.
    const iconName = this.theme === 'dark' ? 'sun' : 'moon';
    return html`<ui-icon name=${iconName} size="18"></ui-icon>`;
  }
}

customElements.define('ui-theme-toggle', UIThemeToggle);
