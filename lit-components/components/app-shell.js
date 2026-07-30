import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <app-shell page-title="Dashboard" active="dashboard">…</app-shell>
 *
 * Desktop: macOS-style menu-bar + translucent title bar + floating dock.
 * Mobile:  compact topbar + solid 5-icon bottom tab bar.
 *
 * Lit port of the vanilla web-component. Behaviour parity:
 *   - Menu bar dropdowns (click to open, hover to switch, click-outside to close).
 *   - Theme toggle (persisted in localStorage under `erp.theme`).
 *   - Primary color swatches (persisted under `erp.primary`).
 *   - Live clock in the menu-bar status area.
 *   - Emits nothing; child <a> hrefs drive navigation.
 */

const NAV = [
  { id: 'dashboard',    label: 'Dashboard',    icon: 'home',      href: 'index.html',
    color: 'linear-gradient(135deg,#0a84ff,#5e5ce6)' },
  { id: 'apps',         label: 'Apps',         icon: 'grid',      href: 'apps.html',
    color: 'linear-gradient(135deg,#8e8e93,#3a3a3c)' },
  { id: 'students',     label: 'Students',     icon: 'student',   href: 'students.html',
    color: 'linear-gradient(135deg,#0a84ff,#5e5ce6)' },
  { id: 'attendance',   label: 'Attendance',   icon: 'clipboard', href: 'attendance.html',
    color: 'linear-gradient(135deg,#34c759,#30b6a3)' },
  { id: 'grades',       label: 'Grades',       icon: 'chart',     href: 'grades.html',
    color: 'linear-gradient(135deg,#ff9f0a,#ff6f0a)' },
  { id: 'fees',         label: 'Fees',         icon: 'card',      href: 'fees.html',
    color: 'linear-gradient(135deg,#0a84ff,#34c759)' },
  { id: 'transactions', label: 'Transactions', icon: 'wallet',    href: 'transactions.html',
    color: 'linear-gradient(135deg,#34c759,#0a84ff)' },
  { id: 'timetable',    label: 'Timetable',    icon: 'calendar',  href: 'timetable.html',
    color: 'linear-gradient(135deg,#af52de,#ff375f)' },
  { id: 'settings',     label: 'Settings',     icon: 'settings',  href: 'settings.html',
    color: 'linear-gradient(135deg,#8e8e93,#48484a)' },
];

const MOBILE_TABS = ['dashboard', 'apps', 'students', 'attendance', 'settings'];

const PRIMARY_SWATCHES = [
  '#0a84ff', '#5e5ce6', '#af52de', '#ff375f',
  '#ff9f0a', '#34c759', '#5ac8fa', '#ffd60a'
];

const MENUS = [
  { label: 'File', items: [
    { label: 'New Announcement', shortcut: '⌘N', onclick: "window.__openAnnouncement && window.__openAnnouncement()" },
    { label: 'New Student',      shortcut: '⇧⌘N', href: 'students.html' },
    { divider: true },
    { label: 'Import…',          shortcut: '⌘I' },
    { label: 'Export CSV',       shortcut: '⌘E', onclick: "window.toast && toast('Export queued', {tone:'info'})" },
    { divider: true },
    { label: 'Print…',           shortcut: '⌘P', onclick: "window.print()" },
  ]},
  { label: 'Edit', items: [
    { label: 'Undo', shortcut: '⌘Z' },
    { label: 'Redo', shortcut: '⇧⌘Z' },
    { divider: true },
    { label: 'Cut',   shortcut: '⌘X' },
    { label: 'Copy',  shortcut: '⌘C' },
    { label: 'Paste', shortcut: '⌘V' },
    { divider: true },
    { label: 'Find…', shortcut: '⌘F' },
  ]},
  { label: 'View', items: [
    { label: 'Dashboard',   href: 'index.html' },
    { label: 'Apps',        href: 'apps.html' },
    { label: 'Students',    href: 'students.html' },
    { label: 'Attendance',  href: 'attendance.html' },
    { label: 'Grades',      href: 'grades.html' },
    { label: 'Fees',        href: 'fees.html' },
    { label: 'Transactions',href: 'transactions.html' },
    { label: 'Timetable',   href: 'timetable.html' },
    { divider: true },
    { label: 'Toggle Theme', shortcut: '⌘T', onclick: "document.getElementById('__themeToggle')?.click()" },
  ]},
  { label: 'Window', items: [
    { label: 'Minimize', shortcut: '⌘M' },
    { label: 'Zoom' },
    { divider: true },
    { label: 'Bring All to Front' },
  ]},
  { label: 'Help', items: [
    { label: 'Sunrise K-12 Help', shortcut: '⌘?' },
    { label: 'Keyboard Shortcuts' },
    { divider: true },
    { label: 'About Sunrise K-12', onclick: "toast('Sunrise K-12 · Wireframe v0.1', {tone:'info'})" },
  ]},
];

function shade(hex, percent) {
  const num = parseInt(hex.slice(1), 16);
  let r = (num >> 16) + Math.round(2.55 * percent);
  let g = ((num >> 8) & 0xff) + Math.round(2.55 * percent);
  let b = (num & 0xff) + Math.round(2.55 * percent);
  r = Math.max(0, Math.min(255, r));
  g = Math.max(0, Math.min(255, g));
  b = Math.max(0, Math.min(255, b));
  return '#' + ((1 << 24) | (r << 16) | (g << 8) | b).toString(16).slice(1);
}

class AppShell extends LitBaseElement {
  static properties = {
    pageTitle: { type: String, attribute: 'page-title', reflect: true },
    active:    { type: String, reflect: true },
    _openMenu: { state: true },
    _clock:    { state: true },
    _primary:  { state: true },
    _theme:    { state: true },
  };

  static styles = css`
    :host { display: block; min-height: 100dvh; color: var(--color-text); }
    :host { --dock-h: 78px; --bottomnav-h: 62px; }
    main { display: flex; flex-direction: column; min-height: 100dvh; min-width: 0; }

    /* Menu bar */
    .menubar {
      position: sticky; top: 0; z-index: 25;
      display: flex; align-items: center; gap: 4px;
      height: 26px; padding: 0 8px;
      background: color-mix(in srgb, var(--color-surface) 85%, transparent);
      backdrop-filter: saturate(180%) blur(24px);
      -webkit-backdrop-filter: saturate(180%) blur(24px);
      border-bottom: 1px solid var(--color-border);
      font-size: 13px; user-select: none;
    }
    .mb-logo {
      display: inline-flex; align-items: center; gap: 6px;
      padding: 0 8px 0 2px; margin-right: 4px;
      font-weight: var(--fw-semibold);
    }
    .mb-logo ui-icon { color: var(--color-primary); }
    .mb-item {
      position: relative; display: inline-flex; align-items: center;
      padding: 0 9px; height: 20px; border-radius: 5px;
      color: var(--color-text); cursor: pointer;
      font-size: 13px; font-weight: var(--fw-medium);
    }
    .mb-item:hover, .mb-item[aria-expanded="true"] {
      background: var(--color-primary); color: var(--color-primary-contrast);
    }
    .mb-status {
      margin-left: auto;
      display: inline-flex; align-items: center; gap: 12px;
      font-size: 12px; color: var(--color-text-muted);
    }
    .mb-status .time { font-variant-numeric: tabular-nums; color: var(--color-text); font-weight: var(--fw-medium); }

    .mb-menu {
      position: absolute; top: 100%; left: 0;
      background: color-mix(in srgb, var(--color-surface) 96%, transparent);
      backdrop-filter: saturate(180%) blur(24px);
      -webkit-backdrop-filter: saturate(180%) blur(24px);
      border: 1px solid var(--color-border);
      border-radius: 8px; box-shadow: var(--shadow-lg);
      min-width: 220px; padding: 4px;
      display: none; z-index: 1500;
    }
    .mb-item[aria-expanded="true"] .mb-menu { display: block; }
    .mb-row {
      display: flex; align-items: center;
      padding: 5px 10px; border-radius: 5px;
      font-size: 13px; color: var(--color-text);
      cursor: pointer; text-decoration: none;
      font-weight: var(--fw-regular);
    }
    .mb-row:hover { background: var(--color-primary); color: var(--color-primary-contrast); }
    .mb-row .sc { margin-left: auto; font-size: 12px; color: var(--color-text-muted); font-variant-numeric: tabular-nums; }
    .mb-row:hover .sc { color: var(--color-primary-contrast); opacity: 0.9; }
    .mb-sep { height: 1px; background: var(--color-border); margin: 4px 6px; }

    /* Top bar */
    header.top {
      position: sticky; top: 26px; z-index: 20;
      display: flex; align-items: center; gap: var(--space-3);
      padding: 0 var(--space-5); height: 52px;
      background: color-mix(in srgb, var(--color-surface) 68%, transparent);
      backdrop-filter: saturate(180%) blur(24px);
      -webkit-backdrop-filter: saturate(180%) blur(24px);
      border-bottom: 1px solid var(--color-border);
      min-width: 0;
    }
    header.top h1 {
      margin: 0; font-size: var(--fs-md); font-weight: var(--fw-semibold);
      flex: 1 1 auto; min-width: 0; letter-spacing: -0.01em;
      white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }
    .search {
      display: flex; align-items: center; gap: var(--space-2);
      background: color-mix(in srgb, var(--color-text) 5%, transparent);
      border: 1px solid transparent; border-radius: 8px;
      padding: 5px 10px; width: 240px; max-width: 30vw;
      color: var(--color-text-muted); font-size: var(--fs-sm);
    }
    .search input { border: 0; outline: 0; background: transparent; color: var(--color-text); width: 100%; font: inherit; }
    kbd {
      font: 11px var(--font-mono, ui-monospace); background: var(--color-surface-alt);
      border: 1px solid var(--color-border); border-radius: 4px;
      padding: 1px 5px; color: var(--color-text-muted);
    }

    .icon-btn {
      background: transparent; border: 0;
      width: 30px; height: 30px; border-radius: 8px;
      display: grid; place-items: center; cursor: pointer;
      color: var(--color-text-muted);
    }
    .icon-btn:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); color: var(--color-text); }
    .swatches { display: flex; gap: 6px; align-items: center; }
    .sw {
      width: 16px; height: 16px; border-radius: 50%; cursor: pointer;
      border: 2px solid var(--color-surface); box-shadow: 0 0 0 1px var(--color-border);
    }
    .sw:hover { transform: scale(1.15); }
    .sw[aria-current="true"] { box-shadow: 0 0 0 2px var(--color-text); }
    .divider { width:1px; height:20px; background: var(--color-border); margin: 0 4px; }

    /* Content */
    .content {
      padding: var(--space-6) var(--space-6) calc(var(--dock-h) + var(--space-8));
      max-width: 1280px; width: 100%; margin: 0 auto;
      min-width: 0; box-sizing: border-box;
    }

    /* Desktop dock */
    nav.dock {
      position: fixed; left: 50%; transform: translateX(-50%);
      bottom: calc(12px + env(safe-area-inset-bottom, 0)); z-index: 900;
      display: flex; align-items: end; gap: 6px; padding: 8px 10px;
      background: color-mix(in srgb, var(--color-surface) 65%, transparent);
      backdrop-filter: saturate(180%) blur(24px);
      -webkit-backdrop-filter: saturate(180%) blur(24px);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-xl);
      box-shadow: 0 20px 40px rgba(0,0,0,.18), 0 6px 16px rgba(0,0,0,.10);
      max-width: calc(100vw - 16px); overflow-x: auto; scrollbar-width: none;
    }
    nav.dock::-webkit-scrollbar { display: none; }
    nav.dock a {
      position: relative; display: flex; flex-direction: column; align-items: center;
      text-decoration: none; flex: 0 0 auto;
    }
    .tile {
      width: 52px; height: 52px; border-radius: 14px;
      display: grid; place-items: center; color: #fff;
      box-shadow: var(--shadow-sm), inset 0 -6px 12px rgba(0,0,0,.10), inset 0 1px 0 rgba(255,255,255,.25);
      transition: transform var(--dur-med) var(--ease);
      transform-origin: bottom center;
    }
    nav.dock a:hover .tile { transform: translateY(-10px) scale(1.28); }
    .tip {
      position: absolute; bottom: calc(100% + 8px); left: 50%;
      transform: translateX(-50%) translateY(4px);
      background: var(--color-text); color: var(--color-bg);
      padding: 3px 8px; border-radius: 6px;
      font-size: 11px; font-weight: var(--fw-medium);
      white-space: nowrap; opacity: 0; pointer-events: none;
      transition: opacity var(--dur-fast) var(--ease), transform var(--dur-fast) var(--ease);
    }
    nav.dock a:hover .tip { opacity: 1; transform: translateX(-50%) translateY(0); }
    nav.dock a::after {
      content: ''; display: block;
      width: 4px; height: 4px; border-radius: 50%;
      background: transparent; margin-top: 4px;
    }
    nav.dock a[aria-current="page"]::after { background: var(--color-text); }

    /* Mobile */
    nav.mobile-tabs { display: none; }
    @media (max-width: 860px) {
      .menubar { display: none; }
      nav.dock { display: none; }
      header.top { top: 0; padding: 0 var(--space-4); }
      .search, .swatches, .divider { display: none; }
      .content {
        padding: var(--space-4) var(--space-4)
                 calc(var(--bottomnav-h) + var(--space-6) + env(safe-area-inset-bottom, 0));
      }
      nav.mobile-tabs {
        display: flex; position: fixed; left: 0; right: 0; bottom: 0;
        height: calc(var(--bottomnav-h) + env(safe-area-inset-bottom, 0));
        padding-bottom: env(safe-area-inset-bottom, 0);
        background: color-mix(in srgb, var(--color-surface) 96%, transparent);
        border-top: 1px solid var(--color-border);
        z-index: 900;
      }
      nav.mobile-tabs a {
        flex: 1; display: flex; flex-direction: column;
        align-items: center; justify-content: center; gap: 3px;
        text-decoration: none; color: var(--color-text-subtle);
        font-size: 10px; font-weight: var(--fw-medium);
      }
      nav.mobile-tabs a[aria-current="page"] { color: var(--color-primary); }
    }
  `;

  constructor() {
    super();
    this.pageTitle = 'Overview';
    this.active = 'dashboard';
    this._openMenu = -1;
    this._clock = '';
    this._primary = localStorage.getItem('erp.primary') || '#0a84ff';
    this._theme   = document.documentElement.dataset.theme || localStorage.getItem('erp.theme') || 'light';
    this._boundDocClick = (e) => {
      if (!this.contains(e.target)) this._openMenu = -1;
    };
    this._boundEsc = (e) => { if (e.key === 'Escape') this._openMenu = -1; };
  }

  connectedCallback() {
    super.connectedCallback();
    document.addEventListener('click', this._boundDocClick);
    document.addEventListener('keydown', this._boundEsc);
    this.#tick();
    this._clockTimer = setInterval(() => this.#tick(), 30_000);
  }
  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener('click', this._boundDocClick);
    document.removeEventListener('keydown', this._boundEsc);
    clearInterval(this._clockTimer);
  }

  #tick() {
    const d = new Date();
    const opts = { weekday: 'short', day: 'numeric', month: 'short', hour: 'numeric', minute: '2-digit' };
    this._clock = d.toLocaleString(undefined, opts);
  }

  #toggleTheme() {
    const next = this._theme === 'dark' ? 'light' : 'dark';
    this._theme = next;
    document.documentElement.dataset.theme = next;
    localStorage.setItem('erp.theme', next);
  }

  #pickPrimary(c) {
    this._primary = c;
    document.documentElement.style.setProperty('--color-primary', c);
    document.documentElement.style.setProperty('--color-primary-hover', shade(c, -12));
    localStorage.setItem('erp.primary', c);
  }

  #onMenuClick(e, i) {
    e.stopPropagation();
    this._openMenu = this._openMenu === i ? -1 : i;
  }
  #onMenuHover(i) {
    if (this._openMenu !== -1) this._openMenu = i;
  }

  render() {
    const left  = NAV.filter(n => ['dashboard', 'apps'].includes(n.id));
    const mid   = NAV.filter(n => !['dashboard', 'apps', 'settings'].includes(n.id));
    const right = NAV.filter(n => ['settings'].includes(n.id));

    const dockLink = (n) => html`
      <a href=${n.href} aria-current=${n.id === this.active ? 'page' : nothing}>
        <span class="tile" style=${`background:${n.color}`}>
          <ui-icon name=${n.icon} size="24"></ui-icon>
        </span>
        <span class="tip">${n.label}</span>
      </a>`;
    const sep = html`<span style="width:1px; height:36px; background: var(--color-border); align-self:center; margin: 0 4px; flex: 0 0 auto;"></span>`;

    return html`
      <div class="menubar">
        <span class="mb-logo"><ui-icon name="home" size="14"></ui-icon> Sunrise</span>
        ${MENUS.map((m, i) => html`
          <div class="mb-item"
               aria-expanded=${this._openMenu === i}
               @click=${(e) => this.#onMenuClick(e, i)}
               @mouseenter=${() => this.#onMenuHover(i)}>
            ${m.label}
            <div class="mb-menu">
              ${m.items.map(it => it.divider
                ? html`<div class="mb-sep"></div>`
                : html`<a class="mb-row"
                          href=${it.href || '#'}
                          @click=${(ev) => {
                            if (!it.href) ev.preventDefault();
                            if (it.onclick) { ev.preventDefault(); try { new Function(it.onclick)(); } catch {} }
                          }}>
                    ${it.label}
                    ${it.shortcut ? html`<span class="sc">${it.shortcut}</span>` : nothing}
                  </a>`)}
            </div>
          </div>`)}
        <div class="mb-status">
          <span class="time">${this._clock}</span>
        </div>
      </div>

      <main>
        <header class="top">
          <h1>${this.pageTitle}</h1>
          <label class="search">
            <ui-icon name="search" size="14"></ui-icon>
            <input placeholder="Search…">
            <kbd>⌘K</kbd>
          </label>
          <div class="swatches" role="group" aria-label="Primary color">
            ${PRIMARY_SWATCHES.map(c => html`
              <span class="sw"
                    style=${`background:${c}`}
                    aria-current=${c === this._primary ? 'true' : nothing}
                    @click=${() => this.#pickPrimary(c)}></span>`)}
          </div>
          <span class="divider"></span>
          <button class="icon-btn" id="__themeToggle" title="Toggle theme"
                  @click=${() => this.#toggleTheme()}>
            <ui-icon name=${this._theme === 'dark' ? 'sun' : 'moon'} size="16"></ui-icon>
          </button>
          <button class="icon-btn" title="Notifications">
            <ui-icon name="bell" size="16"></ui-icon>
          </button>
          <ui-avatar name="Anita R." size="sm" title="Anita R."></ui-avatar>
        </header>
        <div class="content"><slot></slot></div>
      </main>

      <nav class="dock" role="navigation" aria-label="Apps">
        ${left.map(dockLink)}
        ${sep}
        ${mid.map(dockLink)}
        ${sep}
        ${right.map(dockLink)}
      </nav>

      <nav class="mobile-tabs" role="navigation" aria-label="Sections">
        ${MOBILE_TABS.map(id => {
          const n = NAV.find(x => x.id === id);
          return html`<a href=${n.href}
                         aria-current=${n.id === this.active ? 'page' : nothing}>
            <ui-icon name=${n.icon} size="22"></ui-icon><span>${n.label}</span>
          </a>`;
        })}
      </nav>
    `;
  }
}
customElements.define('app-shell', AppShell);
