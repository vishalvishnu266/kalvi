// Critical bundle — components visible above the fold on every page.
//
// Loaded eagerly and awaited by the FOUCE gate so the shell paints once
// and stays painted. Keep this file SMALL. If a component isn't on the
// first screen of every page, put it in `extras.js` or make it lazy.
//
// Import-only side effects: each module calls customElements.define().

// Primitives (see ./primitives/ — atomic building blocks: form inputs +
// small display atoms). Kept in the critical bundle because they appear on
// every first-paint page.
import './primitives/ui-icon.js';
import './primitives/ui-button.js';
import './primitives/ui-badge.js';
import './primitives/ui-avatar.js';
import './primitives/ui-skeleton.js';

// Layout primitives (see ./layout/) — must be defined before first paint
// so children don't reflow when the tag upgrades (avoids CLS).
// Compose these to build every page skeleton — body, top-bar, split
// panels, cards grid, etc. — without writing bespoke CSS.
import './layout/ui-columns.js';
import './layout/ui-stack.js';
import './layout/ui-cluster.js';
import './layout/ui-grid.js';
import './layout/ui-sidebar.js';
import './layout/ui-center.js';

// Non-primitive critical components.
import './ui-card.js';
import './ui-stat.js';
import './ui-breadcrumb.js';
import './ui-toast.js';

// NOTE: the framework `ui-app-shell`, `ui-fragment`, `ui-copilot`, and
// `ui-launcher` are imported here so they are guaranteed to be defined
// before the first paint (they own the layout, the fragment-application
// pipeline, the agent-driven copilot pane, and the ⌘K launcher
// respectively).
import './ui-fragment.js';
import './ui-app-shell.js';
import './ui-copilot.js';
import './ui-launcher.js';
import './ui-primary-bar.js';

// ---- Restore saved theme + primary color BEFORE first paint ----
// (Was in the old index.js; kept here because it must run in the critical
// path to avoid a colour flash on hard refresh.)
const savedTheme = localStorage.getItem('erp.theme');
if (savedTheme) document.documentElement.dataset.theme = savedTheme;

const savedPrimary = localStorage.getItem('erp.primary');
if (savedPrimary) {
  document.documentElement.style.setProperty('--color-primary', savedPrimary);
}
