// Critical bundle — every primitive + every layout primitive.
//
// After the non-primitive purge, this is now the ENTIRE component surface
// of the library: 13 atomic primitives + 6 layout primitives. All of it
// is small enough to ship in one eager bundle, so we no longer split into
// tiers or lazy-load anything.
//
// Import-only side effects: each module calls customElements.define().

// Primitives — atomic building blocks (form inputs + small display atoms).
import './primitives/ui-icon.js';
import './primitives/ui-button.js';
import './primitives/ui-badge.js';
import './primitives/ui-avatar.js';
import './primitives/ui-skeleton.js';
import './primitives/ui-input.js';
import './primitives/ui-select.js';
import './primitives/ui-checkbox.js';
import './primitives/ui-radio.js';
import './primitives/ui-switch.js';
import './primitives/ui-tooltip.js';
import './primitives/ui-datepicker.js';
import './primitives/ui-daterange.js';
import './primitives/ui-slider.js';
import './primitives/ui-progress.js';
import './primitives/ui-color-swatch.js';
import './primitives/ui-theme-toggle.js';
import './primitives/ui-heading.js';

// Layout primitives — must be defined before first paint so children
// don't reflow when the tag upgrades (avoids CLS). Compose these to build
// every page skeleton without writing bespoke CSS.
import './layout/ui-columns.js';
import './layout/ui-stack.js';
import './layout/ui-cluster.js';
import './layout/ui-grid.js';
import './layout/ui-sidebar.js';
import './layout/ui-center.js';

// Restore saved theme + primary color BEFORE first paint (avoids a
// colour flash on hard refresh).
const savedTheme = localStorage.getItem('erp.theme');
if (savedTheme) document.documentElement.dataset.theme = savedTheme;

const savedPrimary = localStorage.getItem('erp.primary');
if (savedPrimary) {
  document.documentElement.style.setProperty('--color-primary', savedPrimary);
}
