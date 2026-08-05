// -----------------------------------------------------------------------------
// <ui-primary-bar> — the 4 always-visible primary controls.
//
//   Apps   → navigates to /apps (colored icon launcher page)
//   Search → opens <ui-launcher> (⌘K)
//   AI     → toggles <ui-copilot>
//   Profile → navigates to /settings (placeholder for now)
//
// Renders identically on desktop and mobile; CSS decides placement:
//   • data-slot="topbar"  → visible on ≥768px, hidden on mobile
//   • data-slot="bottom"  → hidden on ≥768px, visible on mobile
//     (fixed bottom bar, safe-area-aware)
//
// This is the ONE element the user learns to look for. Everything
// else in the app is reachable from these 4 icons.
// -----------------------------------------------------------------------------

class UiPrimaryBar extends HTMLElement {
  connectedCallback() {
    if (this._mounted) return;
    this._mounted = true;

    const slot = this.dataset.slot || 'topbar';

    // Install document-level styles once (both instances share them).
    UiPrimaryBar._installStyles();

    this.innerHTML = `
      <div class="ui-pb ui-pb--${slot}">
        <button type="button" class="ui-pb-btn" data-action="apps"    aria-label="Apps">
          <ui-icon name="grid" size="20"></ui-icon>
          <span class="ui-pb-label">Apps</span>
        </button>
        <button type="button" class="ui-pb-btn" data-action="search"  aria-label="Search (Cmd+K)">
          <ui-icon name="search" size="20"></ui-icon>
          <span class="ui-pb-label">Search</span>
        </button>
        <button type="button" class="ui-pb-btn ui-pb-btn--ai" data-action="ai" aria-label="AI">
          <span class="ui-pb-ai-glyph">✨</span>
          <span class="ui-pb-label">AI</span>
        </button>
        <button type="button" class="ui-pb-btn" data-action="profile" aria-label="Profile">
          <span class="ui-pb-avatar" aria-hidden="true">A</span>
          <span class="ui-pb-label">Profile</span>
        </button>
      </div>
    `;

    this.addEventListener('click', (e) => {
      const btn = e.target.closest('.ui-pb-btn');
      if (!btn) return;
      switch (btn.dataset.action) {
        case 'apps':    if (window.ui?.navigate)     window.ui.navigate('/apps');      break;
        case 'search':  if (window.ui?.openLauncher) window.ui.openLauncher();         break;
        case 'ai':      this._toggleCopilot();                                          break;
        case 'profile': if (window.ui?.navigate)     window.ui.navigate('/settings');  break;
      }
    });
  }

  _toggleCopilot() {
    const cop = document.querySelector('ui-copilot');
    if (cop && typeof cop.toggle === 'function') cop.toggle();
  }

  static _installStyles() {
    if (document.querySelector('style[data-ui-primary-bar]')) return;
    const style = document.createElement('style');
    style.dataset.uiPrimaryBar = '';
    style.textContent = `
      /* ─── Shared button ─── */
      .ui-pb { display: flex; align-items: center; }
      .ui-pb-btn {
        display: flex; flex-direction: column; align-items: center; justify-content: center;
        gap: 2px;
        background: transparent; color: inherit;
        border: 0; border-radius: 10px;
        cursor: pointer;
        font: inherit;
        padding: 6px 10px;
        opacity: .7;
        transition: opacity .12s, background .12s, transform .08s;
      }
      .ui-pb-btn:hover     { opacity: 1; background: var(--color-surface-2, #f5f5f7); }
      .ui-pb-btn:active    { transform: scale(.94); }
      .ui-pb-label {
        font-size: 10px; letter-spacing: .01em; line-height: 1;
      }
      /* Special: AI glyph gets a subtle gradient chip so it stands out. */
      .ui-pb-btn--ai .ui-pb-ai-glyph {
        display: grid; place-items: center;
        width: 22px; height: 22px; border-radius: 6px;
        font-size: 14px;
        background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%);
        color: #fff;
      }
      /* Profile avatar — plain colored circle for now. */
      .ui-pb-avatar {
        display: grid; place-items: center;
        width: 22px; height: 22px; border-radius: 50%;
        font-size: 11px; font-weight: 600;
        background: linear-gradient(135deg, #f59e0b, #ef4444);
        color: #fff;
      }

      /* ─── Desktop placement (inside topbar) ─── */
      .ui-pb--topbar { gap: 4px; }
      .ui-pb--topbar .ui-pb-label { display: none; }
      .ui-pb--topbar .ui-pb-btn   { padding: 6px 8px; border-radius: 8px; }

      /* ─── Mobile placement (fixed bottom bar) ─── */
      .ui-pb--bottom {
        position: fixed;
        left: 0; right: 0; bottom: 0;
        z-index: 80;
        justify-content: space-around;
        padding: 6px 4px max(6px, env(safe-area-inset-bottom));
        background: var(--color-bg, #fff);
        border-top: 1px solid var(--color-border, #e5e7eb);
      }
      .ui-pb--bottom .ui-pb-btn { flex: 1; }

      /* Show topbar-variant only on desktop, bottom-variant only on mobile. */
      @media (max-width: 768px) { .ui-pb--topbar { display: none; } }
      @media (min-width: 769px) { .ui-pb--bottom { display: none; } }
    `;
    document.head.appendChild(style);
  }
}

if (!customElements.get('ui-primary-bar')) {
  customElements.define('ui-primary-bar', UiPrimaryBar);
}
