// Small shared header + nav for the per-component demo pages.
// Injected by each demo/*.html page via a <script type="module"> at the top.

// Nav for the demo pages. After the non-primitive purge, the surface is
// 13 primitives + 6 layout primitives (all rolled into `layout kit`).
const COMPONENTS = [
  // ── Primitives ──
  ['ui-icon',        'icons.html'],
  ['ui-button',      'buttons.html'],
  ['ui-badge',       'badges.html'],
  ['ui-avatar',      'avatars.html'],
  ['ui-skeleton',    'skeleton.html'],
  ['ui-input',       'inputs.html'],
  ['ui-select',      'selects.html'],
  ['ui-checkbox',    'checkboxes.html'],
  ['ui-radio',       'radios.html'],
  ['ui-switch',      'switches.html'],
  ['ui-tooltip',     'tooltips.html'],
  ['ui-datepicker',  'datepicker.html'],
  ['ui-daterange',   'daterange.html'],
  // ── Layout primitives ──
  ['ui-columns',     'columns.html'],
  ['layout kit',     'layout.html'],
];

function mount() {
  const path = location.pathname.split('/').pop();
  const nav = document.getElementById('demo-nav');
  if (!nav) return;
  nav.innerHTML = COMPONENTS.map(([label, href]) =>
    `<a href="${href}" ${href === path ? 'aria-current="page"' : ''}>${label}</a>`
  ).join('');

  // Theme toggle
  const t = document.getElementById('demo-theme');
  if (t) {
    t.addEventListener('click', () => {
      const isDark = document.documentElement.dataset.theme === 'dark';
      const next = isDark ? 'light' : 'dark';
      document.documentElement.dataset.theme = next;
      localStorage.setItem('erp.theme', next);
    });
  }
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', mount);
} else {
  mount();
}
