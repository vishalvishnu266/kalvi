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
import './app-shell.js';

// ---- Restore saved theme + primary color before first paint ----
const savedTheme = localStorage.getItem('erp.theme');
if (savedTheme) document.documentElement.dataset.theme = savedTheme;

const savedPrimary = localStorage.getItem('erp.primary');
if (savedPrimary) {
  document.documentElement.style.setProperty('--color-primary', savedPrimary);
}
