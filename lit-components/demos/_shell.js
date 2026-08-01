// Small shared header + nav for the per-component demo pages.
// Injected by each demo/*.html page via a <script type="module"> at the top.

const COMPONENTS = [
  ['ui-icon',        'icons.html'],
  ['ui-button',      'buttons.html'],
  ['ui-badge',       'badges.html'],
  ['ui-input',       'inputs.html'],
  ['ui-card',        'cards.html'],
  ['ui-stat',        'stats.html'],
  ['ui-avatar',      'avatars.html'],
  ['ui-list-item',   'list-items.html'],
  ['ui-table',       'tables.html'],
  ['ui-tab-bar',     'tab-bars.html'],
  ['ui-segmented',   'segmented.html'],
  ['ui-modal',       'modals.html'],
  ['ui-select',      'selects.html'],
  ['ui-datepicker',  'datepicker.html'],
  ['ui-daterange',   'daterange.html'],
  ['ui-toast',       'toasts.html'],
  ['ui-checkbox',    'checkboxes.html'],
  ['ui-radio',       'radios.html'],
  ['ui-switch',      'switches.html'],
  ['ui-form',        'forms.html'],
  ['ui-pagination',  'pagination.html'],
  ['ui-empty-state', 'empty-state.html'],
  ['ui-skeleton',    'skeleton.html'],
  ['ui-tooltip',       'tooltips.html'],
  ['ui-avatar-group',  'avatar-groups.html'],
  ['ui-breadcrumb',    'breadcrumbs.html'],
  ['ui-drawer',        'drawers.html'],
  ['ui-dropdown-menu', 'dropdowns.html'],
  ['ui-file-upload',   'file-upload.html'],
  ['ui-progress',      'progress.html'],
  ['ui-stepper',       'stepper.html'],
  ['ui-inline-edit',   'inline-edit.html'],
  ['ui-timeline',      'timeline.html'],
  ['ui-kanban',        'kanban.html'],
  ['ui-command',       'command.html'],
  ['ui-data-table',    'data-table.html'],
  ['app-shell',        'app-shell.html'],
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
