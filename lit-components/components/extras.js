// Deferred bundle — common but not first-paint.
//
// Imported by index.js AFTER the critical `core.js` bundle has resolved.
// These modules are cheap enough to bundle together and land in one
// network request, so we don't fragment them into per-tag chunks.
//
// If a component here starts causing measurable layout shift, promote it
// to `core.js`. If it becomes rarely-used, demote it to `heavy.js`.

// Primitives that are common but not first-paint critical.
import './primitives/ui-input.js';
import './primitives/ui-select.js';
import './primitives/ui-checkbox.js';
import './primitives/ui-radio.js';
import './primitives/ui-switch.js';
import './primitives/ui-tooltip.js';
import './primitives/ui-datepicker.js';
import './primitives/ui-daterange.js';

// Non-primitive extras.
import './ui-form.js';
import './ui-list-item.js';
import './ui-tab-bar.js';
import './ui-segmented.js';
import './ui-avatar-group.js';
import './ui-empty-state.js';
import './ui-pagination.js';
import './ui-modal.js';
