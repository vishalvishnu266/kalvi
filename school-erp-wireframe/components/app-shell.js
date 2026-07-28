import { BaseElement, attr } from './base.js';

/**
 * <app-shell page-title="Dashboard" active="dashboard">…</app-shell>
 *
 * Desktop  → macOS-style layout:
 *              1. Menu bar at the very top (File, Edit, View, Window, Help)
 *              2. Translucent title/toolbar row (page title + search + swatches)
 *              3. Full-width content
 *              4. Floating dock at bottom with all apps
 *
 * Mobile   → Native mobile pattern:
 *              1. Compact top bar (logo + page title + theme + notif + avatar)
 *              2. Full-width content
 *              3. Solid 5-icon bottom tab bar (no floating dock)
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

// Bottom tab bar keeps only 5 primary items on mobile (native convention)
const MOBILE_TABS = ['dashboard', 'apps', 'students', 'attendance', 'settings'];

const PRIMARY_SWATCHES = [
  '#0a84ff', '#5e5ce6', '#af52de', '#ff375f',
  '#ff9f0a', '#34c759', '#5ac8fa', '#ffd60a'
];

// macOS-style menu-bar structure. Items with `href` navigate, items with
// `onclick` invoke an inline script, `divider: true` renders a separator.
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
    { label: 'Undo',     shortcut: '⌘Z' },
    { label: 'Redo',     shortcut: '⇧⌘Z' },
    { divider: true },
    { label: 'Cut',      shortcut: '⌘X' },
    { label: 'Copy',     shortcut: '⌘C' },
    { label: 'Paste',    shortcut: '⌘V' },
    { divider: true },
    { label: 'Find…',    shortcut: '⌘F' },
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
    { label: 'Minimize',    shortcut: '⌘M' },
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

class AppShell extends BaseElement {
  static styles = `
    :host { display: block; min-height: 100dvh; color: var(--color-text); }
    main {
      display: flex; flex-direction: column;
      min-height: 100dvh; min-width: 0;
    }

    /* =========================================================
       MENU BAR (desktop only) — macOS style, very thin, translucent
       ========================================================= */
    .menubar {
      position: sticky; top: 0; z-index: 25;
      display: flex; align-items: center; gap: 4px;
      height: 26px;
      padding: 0 8px;
      background: color-mix(in srgb, var(--color-surface) 85%, transparent);
      backdrop-filter: var(--chrome-blur);
      -webkit-backdrop-filter: var(--chrome-blur);
      border-bottom: 1px solid var(--chrome-border);
      font-size: 13px;
      user-select: none;
    }
    .mb-logo {
      display: inline-flex; align-items: center; gap: 6px;
      padding: 0 8px 0 2px; margin-right: 4px;
      font-weight: var(--fw-semibold);
    }
    .mb-logo ui-icon { color: var(--color-primary); }
    .mb-item {
      position: relative;
      display: inline-flex; align-items: center;
      padding: 0 9px; height: 20px;
      border-radius: 5px;
      color: var(--color-text);
      cursor: pointer;
      font-size: 13px;
      font-weight: var(--fw-medium);
    }
    .mb-item:hover, .mb-item[aria-expanded="true"] {
      background: var(--color-primary);
      color: var(--color-primary-contrast);
    }
    .mb-status {
      margin-left: auto;
      display: inline-flex; align-items: center; gap: 12px;
      font-size: 12px; color: var(--color-text-muted);
    }
    .mb-status .time { font-variant-numeric: tabular-nums; color: var(--color-text); font-weight: var(--fw-medium); }

    /* Menu dropdown panel */
    .mb-menu {
      position: absolute; top: 100%; left: 0;
      background: color-mix(in srgb, var(--color-surface) 96%, transparent);
      backdrop-filter: var(--chrome-blur);
      -webkit-backdrop-filter: var(--chrome-blur);
      border: 1px solid var(--color-border);
      border-radius: 8px;
      box-shadow: var(--shadow-lg);
      min-width: 220px;
      padding: 4px;
      display: none;
      z-index: 1500;
    }
    .mb-item[aria-expanded="true"] .mb-menu { display: block; }
    .mb-row {
      display: flex; align-items: center;
      padding: 5px 10px; border-radius: 5px;
      font-size: 13px; color: var(--color-text);
      cursor: pointer;
      text-decoration: none;
      font-weight: var(--fw-regular);
    }
    .mb-row:hover { background: var(--color-primary); color: var(--color-primary-contrast); }
    .mb-row .sc { margin-left: auto; font-size: 12px; color: var(--color-text-muted); font-variant-numeric: tabular-nums; }
    .mb-row:hover .sc { color: var(--color-primary-contrast); opacity: 0.9; }
    .mb-sep { height: 1px; background: var(--color-border); margin: 4px 6px; }

    /* =========================================================
       TOP BAR (title + search + status)
       ========================================================= */
    header.top {
      position: sticky; top: 26px;              /* under the menu bar */
      z-index: 20;
      display: flex; align-items: center; gap: var(--space-3);
      padding: 0 var(--space-5);
      height: var(--topbar-h);
      background: var(--chrome-bg);
      backdrop-filter: var(--chrome-blur);
      -webkit-backdrop-filter: var(--chrome-blur);
      border-bottom: 1px solid var(--chrome-border);
      min-width: 0;
    }
    header.top h1 {
      margin: 0; font-size: var(--fs-md); font-weight: var(--fw-semibold);
      flex: 1 1 auto; min-width: 0;
      letter-spacing: -0.01em;
      white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }
    .search {
      display: flex; align-items: center; gap: var(--space-2);
      background: color-mix(in srgb, var(--color-text) 5%, transparent);
      border: 1px solid transparent;
      border-radius: 8px;
      padding: 5px 10px;
      width: 240px; max-width: 30vw;
      color: var(--color-text-muted); font-size: var(--fs-sm);
      transition: all var(--dur-fast) var(--ease);
    }
    .search:focus-within { background: var(--color-surface); border-color: var(--color-border-strong); box-shadow: var(--shadow-focus); }
    .search input { border: 0; outline: 0; background: transparent; color: var(--color-text); width: 100%; font: inherit; }
    kbd {
      font: 11px var(--font-mono); background: var(--color-surface-alt);
      border: 1px solid var(--color-border);
      border-radius: 4px; padding: 1px 5px; color: var(--color-text-muted);
    }

    .icon-btn {
      background: transparent; border: 0;
      width: 30px; height: 30px; border-radius: 8px;
      display: grid; place-items: center; cursor: pointer;
      color: var(--color-text-muted);
      transition: background var(--dur-fast) var(--ease), color var(--dur-fast) var(--ease);
    }
    .icon-btn:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); color: var(--color-text); }
    .swatches { display: flex; gap: 6px; align-items: center; }
    .sw {
      width: 16px; height: 16px; border-radius: 50%; cursor: pointer;
      border: 2px solid var(--color-surface); box-shadow: 0 0 0 1px var(--color-border);
      transition: transform var(--dur-fast) var(--ease-spring);
    }
    .sw:hover { transform: scale(1.2); }
    .sw[aria-current="true"] { box-shadow: 0 0 0 2px var(--color-text); }
    .divider { width:1px; height:20px; background: var(--color-border); margin: 0 4px; }

    /* =========================================================
       CONTENT
       ========================================================= */
    .content {
      padding: var(--space-6) var(--space-6) calc(var(--dock-h) + var(--space-8));
      max-width: var(--content-max);
      width: 100%;
      margin: 0 auto;
      min-width: 0;
      box-sizing: border-box;
    }

    /* =========================================================
       DESKTOP DOCK (macOS floating pill) — hidden on mobile
       ========================================================= */
    :host { --dock-h: 78px; --bottomnav-h: 62px; }
    nav.dock {
      position: fixed;
      left: 50%; transform: translateX(-50%);
      bottom: calc(12px + env(safe-area-inset-bottom, 0));
      z-index: 900;
      display: flex; align-items: end; gap: 6px;
      padding: 8px 10px;
      background: color-mix(in srgb, var(--color-surface) 65%, transparent);
      backdrop-filter: var(--chrome-blur);
      -webkit-backdrop-filter: var(--chrome-blur);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-2xl);
      box-shadow:
        0 20px 40px rgba(0,0,0,.18),
        0 6px 16px rgba(0,0,0,.10),
        inset 0 1px 0 rgba(255,255,255,.4);
      max-width: calc(100vw - 16px);
      overflow-x: auto;
      scrollbar-width: none;
    }
    nav.dock::-webkit-scrollbar { display: none; }
    nav.dock a {
      position: relative;
      display: flex; flex-direction: column; align-items: center;
      text-decoration: none;
      flex: 0 0 auto;
    }
    .tile {
      width: 52px; height: 52px; border-radius: 14px;
      display: grid; place-items: center;
      color: #fff;
      box-shadow: var(--shadow-sm), inset 0 -6px 12px rgba(0,0,0,.10), inset 0 1px 0 rgba(255,255,255,.25);
      transition: transform var(--dur-med) var(--ease-spring);
      transform-origin: bottom center;
    }
    nav.dock a:hover .tile, nav.dock a:focus-visible .tile { transform: translateY(-10px) scale(1.28); }
    nav.dock a:hover + a .tile, nav.dock a:has(+ a:hover) .tile { transform: translateY(-4px) scale(1.12); }
    .tip {
      position: absolute; bottom: calc(100% + 8px);
      left: 50%; transform: translateX(-50%) translateY(4px);
      background: var(--color-text); color: var(--color-bg);
      padding: 3px 8px; border-radius: 6px;
      font-size: 11px; font-weight: var(--fw-medium);
      white-space: nowrap;
      opacity: 0; pointer-events: none;
      transition: opacity var(--dur-fast) var(--ease), transform var(--dur-fast) var(--ease);
    }
    nav.dock a:hover .tip, nav.dock a:focus-visible .tip {
      opacity: 1; transform: translateX(-50%) translateY(0);
    }
    nav.dock a::after {
      content: ''; display: block;
      width: 4px; height: 4px; border-radius: 50%;
      background: transparent; margin-top: 4px;
      transition: background var(--dur-fast) var(--ease);
    }
    nav.dock a[aria-current="page"]::after { background: var(--color-text); }

    /* =========================================================
       MOBILE — hide dock + menu bar; show native bottom tab bar
       ========================================================= */
    nav.mobile-tabs { display: none; }

    @media (max-width: 860px) {
      .menubar { display: none; }
      nav.dock  { display: none; }
      header.top {
        top: 0;
        padding: 0 var(--space-4);
      }
      .search, .swatches, .divider { display: none; }
      .content {
        padding: var(--space-4) var(--space-4) calc(var(--bottomnav-h) + var(--space-6) + env(safe-area-inset-bottom, 0));
      }

      /* Native mobile bottom tab bar (solid, opaque, 5 items) */
      nav.mobile-tabs {
        display: flex;
        position: fixed; left: 0; right: 0; bottom: 0;
        height: calc(var(--bottomnav-h) + env(safe-area-inset-bottom, 0));
        padding-bottom: env(safe-area-inset-bottom, 0);
        background: color-mix(in srgb, var(--color-surface) 96%, transparent);
        backdrop-filter: var(--chrome-blur);
        -webkit-backdrop-filter: var(--chrome-blur);
        border-top: 1px solid var(--color-border);
        z-index: 900;
      }
      nav.mobile-tabs a {
        flex: 1;
        display: flex; flex-direction: column; align-items: center; justify-content: center;
        gap: 3px; text-decoration: none;
        color: var(--color-text-subtle);
        font-size: 10px; font-weight: var(--fw-medium);
      }
      nav.mobile-tabs a[aria-current="page"] { color: var(--color-primary); }
      nav.mobile-tabs a ui-icon {
        transition: transform var(--dur-fast) var(--ease-spring);
      }
      nav.mobile-tabs a[aria-current="page"] ui-icon { transform: scale(1.1); }
    }
  `;

  render() {
    const active = attr(this, 'active', 'dashboard');
    const pageTitle = attr(this, 'page-title', 'Overview');

    // ---- Menu bar ----
    const menuHtml = MENUS.map((m, i) => `
      <div class="mb-item" data-menu-idx="${i}">
        ${m.label}
        <div class="mb-menu">
          ${m.items.map(it => {
            if (it.divider) return `<div class="mb-sep"></div>`;
            const attrs = it.href
              ? `href="${it.href}"`
              : `href="#" ${it.onclick ? `onclick="event.preventDefault();${it.onclick}"` : `onclick="event.preventDefault()"`}`;
            return `<a class="mb-row" ${attrs}>${it.label}${it.shortcut ? `<span class="sc">${it.shortcut}</span>` : ''}</a>`;
          }).join('')}
        </div>
      </div>
    `).join('');

    // ---- Desktop dock ----
    const dockLink = n => `
      <a href="${n.href}" ${n.id === active ? 'aria-current="page"' : ''}>
        <span class="tile" style="background:${n.color}">
          <ui-icon name="${n.icon}" size="24"></ui-icon>
        </span>
        <span class="tip">${n.label}</span>
      </a>`;
    const sep = `<span style="width:1px; height:36px; background: var(--color-border); align-self:center; margin: 0 4px; flex: 0 0 auto;"></span>`;
    const left  = NAV.filter(n => ['dashboard', 'apps'].includes(n.id));
    const mid   = NAV.filter(n => !['dashboard', 'apps', 'settings'].includes(n.id));
    const right = NAV.filter(n => ['settings'].includes(n.id));

    // ---- Mobile tabs ----
    const tabsHtml = MOBILE_TABS.map(id => {
      const n = NAV.find(x => x.id === id);
      return `<a href="${n.href}" ${n.id === active ? 'aria-current="page"' : ''}>
        <ui-icon name="${n.icon}" size="22"></ui-icon><span>${n.label}</span>
      </a>`;
    }).join('');

    const swatches = PRIMARY_SWATCHES.map(c =>
      `<span class="sw" data-color="${c}" style="background:${c}"></span>`
    ).join('');

    return `
      <div class="menubar">
        <span class="mb-logo"><ui-icon name="home" size="14"></ui-icon> Sunrise</span>
        ${menuHtml}
        <div class="mb-status">
          <span data-role="clock" class="time"></span>
        </div>
      </div>

      <main>
        <header class="top">
          <h1>${pageTitle}</h1>
          <label class="search">
            <ui-icon name="search" size="14"></ui-icon>
            <input placeholder="Search…">
            <kbd>⌘K</kbd>
          </label>
          <div class="swatches" role="group" aria-label="Primary color">${swatches}</div>
          <span class="divider"></span>
          <button class="icon-btn" id="__themeToggle" title="Toggle theme">
            <ui-icon name="moon" size="16"></ui-icon>
          </button>
          <button class="icon-btn" title="Notifications">
            <ui-icon name="bell" size="16"></ui-icon>
          </button>
          <ui-avatar name="Anita R." size="sm" title="Anita R."></ui-avatar>
        </header>
        <div class="content"><slot></slot></div>
      </main>

      <nav class="dock" role="navigation" aria-label="Apps">
        ${left.map(dockLink).join('')}
        ${sep}
        ${mid.map(dockLink).join('')}
        ${sep}
        ${right.map(dockLink).join('')}
      </nav>

      <nav class="mobile-tabs" role="navigation" aria-label="Sections">${tabsHtml}</nav>
    `;
  }

  afterRender() {
    const toggle = this.$('#__themeToggle');
    const updateThemeIcon = () => {
      const isDark = document.documentElement.dataset.theme === 'dark';
      toggle.innerHTML = `<ui-icon name="${isDark ? 'sun' : 'moon'}" size="16"></ui-icon>`;
    };
    updateThemeIcon();
    toggle.addEventListener('click', () => {
      const root = document.documentElement;
      const next = root.dataset.theme === 'dark' ? 'light' : 'dark';
      root.dataset.theme = next;
      localStorage.setItem('erp.theme', next);
      updateThemeIcon();
    });

    this.$$('.sw').forEach(sw => {
      sw.addEventListener('click', () => {
        const c = sw.dataset.color;
        document.documentElement.style.setProperty('--color-primary', c);
        document.documentElement.style.setProperty('--color-primary-hover', shade(c, -12));
        localStorage.setItem('erp.primary', c);
        this.$$('.sw').forEach(s => s.setAttribute('aria-current', s === sw));
      });
      const current = localStorage.getItem('erp.primary') || '#0a84ff';
      if (sw.dataset.color === current) sw.setAttribute('aria-current', 'true');
    });

    // ---- Menu bar behaviour: click to open, hover to switch, click outside to close ----
    const menuItems = this.$$('.mb-item');
    const closeAll = () => menuItems.forEach(m => m.removeAttribute('aria-expanded'));
    menuItems.forEach(mi => {
      mi.addEventListener('click', (e) => {
        e.stopPropagation();
        const open = mi.getAttribute('aria-expanded') === 'true';
        closeAll();
        if (!open) mi.setAttribute('aria-expanded', 'true');
      });
      mi.addEventListener('mouseenter', () => {
        // If any menu is already open, switch to this one on hover.
        if (this.$$('.mb-item[aria-expanded="true"]').length) {
          closeAll();
          mi.setAttribute('aria-expanded', 'true');
        }
      });
    });
    document.addEventListener('click', (e) => {
      if (!this.contains(e.target)) closeAll();
      else if (!e.composedPath().some(el => el?.classList?.contains?.('mb-item'))) closeAll();
    });
    document.addEventListener('keydown', e => { if (e.key === 'Escape') closeAll(); });

    // ---- Clock in the menu-bar status area ----
    const clock = this.$('[data-role="clock"]');
    const tick = () => {
      if (!clock) return;
      const d = new Date();
      const opts = { weekday: 'short', day: 'numeric', month: 'short',
                     hour: 'numeric', minute: '2-digit' };
      clock.textContent = d.toLocaleString(undefined, opts);
    };
    tick();
    this._clockTimer = setInterval(tick, 30_000);
  }

  disconnectedCallback() {
    if (this._clockTimer) clearInterval(this._clockTimer);
  }
}

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

customElements.define('app-shell', AppShell);
