// Entry file — import once per page.
//
// In a Rust/Axum template, add:
//   <script type="module" src="/lit-components/components/index.js"></script>
//
// Each component self-registers via customElements.define().

import './ui-icon.js';
import './ui-button.js';
import './ui-card.js';
import './ui-badge.js';
import './ui-input.js';
import './ui-stat.js';
import './ui-avatar.js';
import './ui-list-item.js';
import './ui-table.js';
import './ui-tab-bar.js';
import './ui-segmented.js';
import './ui-modal.js';
import './ui-select.js';
import './ui-datepicker.js';
import './ui-daterange.js';
import './ui-toast.js';
// Tier-1 form + data components
import './ui-checkbox.js';
import './ui-radio.js';
import './ui-switch.js';
import './ui-form.js';
import './ui-combobox.js';
import './ui-pagination.js';
import './ui-empty-state.js';
import './ui-skeleton.js';
// Tier-2 UX components
import './ui-tooltip.js';
import './ui-avatar-group.js';
import './ui-breadcrumb.js';
import './ui-drawer.js';
import './ui-dropdown-menu.js';
import './ui-file-upload.js';
// Tier-3 heavy-hitters
import './ui-progress.js';
import './ui-stepper.js';
import './ui-inline-edit.js';
import './ui-timeline.js';
import './ui-kanban.js';
import './ui-command.js';
import './ui-data-table.js';
import './app-shell.js';

// ---- Restore saved theme + primary color before first paint ----
const savedTheme = localStorage.getItem('erp.theme');
if (savedTheme) document.documentElement.dataset.theme = savedTheme;

const savedPrimary = localStorage.getItem('erp.primary');
if (savedPrimary) {
  document.documentElement.style.setProperty('--color-primary', savedPrimary);
}
